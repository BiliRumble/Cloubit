use log::debug;
use reqwest::header::{HeaderMap, HeaderValue, RANGE};
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};
use std::sync::{Arc, Mutex};

use crate::error::AppError;

const CHUNK_SIZE: usize = 256 * 1024;
const MAX_CACHED_CHUNKS: usize = 8;

#[derive(Clone)]
pub struct Reader {
    url: String,
    client: reqwest::Client,
    position: Arc<Mutex<u64>>,
    chunks: Arc<Mutex<HashMap<u64, Arc<Vec<u8>>>>>,
}

// TODO: 优化网络请求
impl Reader {
    pub fn new(url: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "User-Agent",
            HeaderValue::from_str("Mozilla/5.0").expect("Invalid User-Agent"),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Faild to create reqwest client");

        Self {
            url,
            client,
            position: Arc::new(Mutex::new(0)),
            chunks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn wait_preload(&self, chunk_count: u64) -> Result<(), AppError> {
        let start_chunk = *self.position.lock()? / CHUNK_SIZE as u64;

        debug!("预加载 {} 个数据块...", chunk_count);

        let mut handles = Vec::new();
        for i in 0..chunk_count {
            let chunk_index = start_chunk + i;
            let self_clone = self.clone();
            let handle = std::thread::spawn(move || self_clone.get_chunk(chunk_index));
            handles.push(handle);
        }

        Ok(())
    }

    fn fetch_chunk(&self, chunk_index: u64) -> Result<Arc<Vec<u8>>, AppError> {
        let (tx, rx) = std::sync::mpsc::channel();
        let client = self.client.clone();
        let url = self.url.clone();

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let result = rt.block_on(async move {
                let start = chunk_index * CHUNK_SIZE as u64;
                let end = start + CHUNK_SIZE as u64 - 1;
                let range_header = format!("bytes={}-{}", start, end);

                let response = client.get(&url).header(RANGE, range_header).send().await?;

                if response.status().is_success() || response.status().as_u16() == 206 {
                    let bytes = response.bytes().await?;
                    Ok(Arc::new(bytes.to_vec()))
                } else {
                    Err(AppError::Network(format!(
                        "HTTP error when fetching chunk {}: {}",
                        chunk_index,
                        response.status()
                    )))
                }
            });
            let _ = tx.send(result);
        });

        rx.recv()?
    }

    fn get_chunk(&self, chunk_index: u64) -> Result<Arc<Vec<u8>>, AppError> {
        {
            let chunks = self.chunks.lock()?;
            if let Some(chunk) = chunks.get(&chunk_index) {
                return Ok(Arc::clone(chunk));
            }
        }

        let chunk_data = self.fetch_chunk(chunk_index)?;
        let mut chunks = self.chunks.lock()?;

        if chunks.len() >= MAX_CACHED_CHUNKS {
            let current_pos = *self.position.lock()?;
            let current_chunk = current_pos / CHUNK_SIZE as u64;

            chunks.retain(|&k, _| k >= current_chunk.saturating_sub(2) && k <= current_chunk + 6);
            debug!("清理 {} 个数据块", chunks.len());
        }

        chunks.insert(chunk_index, Arc::clone(&chunk_data));
        Ok(chunk_data)
    }
}

impl Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let position = *self.position.lock().map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, format!("Mutex error: {}", e))
        })?;

        let chunk_index = position / CHUNK_SIZE as u64;
        let chunk_offset = (position % CHUNK_SIZE as u64) as usize;

        let chunk_data = self.get_chunk(chunk_index).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to fetch chunk: {}", e),
            )
        })?;

        let available = chunk_data.len().saturating_sub(chunk_offset);
        if available == 0 {
            return Ok(0);
        }

        let to_read = std::cmp::min(buf.len(), available);
        buf[..to_read].copy_from_slice(&chunk_data[chunk_offset..chunk_offset + to_read]);

        *self.position.lock().map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, format!("Mutex error: {}", e))
        })? += to_read as u64;

        if chunk_offset + to_read > CHUNK_SIZE * 3 / 4 {
            let self_clone = self.clone();
            std::thread::spawn(move || {
                if let Err(e) = self_clone.get_chunk(chunk_index + 1) {
                    debug!("预加载下一个数据块失败: {}", e);
                }
            });
        }

        Ok(to_read)
    }
}

impl Seek for Reader {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let new_position = match pos {
            SeekFrom::Start(offset) => offset,
            SeekFrom::Current(offset) => {
                let current = *self.position.lock().map_err(|e| {
                    std::io::Error::new(std::io::ErrorKind::Other, format!("Mutex error: {}", e))
                })?;
                (current as i64 + offset) as u64
            }
            SeekFrom::End(offset) => {
                let current = *self.position.lock().map_err(|e| {
                    std::io::Error::new(std::io::ErrorKind::Other, format!("Mutex error: {}", e))
                })?;
                std::cmp::max(0, current as i64 + offset) as u64
            }
        };

        *self.position.lock().map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, format!("Mutex error: {}", e))
        })? = new_position;

        let chunk_index = new_position / CHUNK_SIZE as u64;
        let self_clone = self.clone();
        std::thread::spawn(move || {
            if let Err(e) = self_clone.get_chunk(chunk_index) {
                debug!("预加载 seek 位置数据块失败: {}", e);
            }
        });

        Ok(new_position)
    }
}
