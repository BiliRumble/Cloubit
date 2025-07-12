use log::{debug, error};
use reqwest::header::{HeaderMap, HeaderValue, RANGE};
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::error::AppError;

const CHUNK_SIZE: usize = 256 * 1024;
const MAX_CACHED_CHUNKS: usize = 8;

#[derive(Clone)]
pub struct Reader {
    url: String,
    client: reqwest::Client,
    position: Arc<Mutex<u64>>,
    chunks: Arc<Mutex<HashMap<u64, Arc<Vec<u8>>>>>,
    runtime: Arc<tokio::runtime::Runtime>,
}

impl Reader {
    pub fn new(url: String) -> Self {
        let client = reqwest::Client::builder()
            .default_headers({
                let mut headers = HeaderMap::new();
                headers.insert("User-Agent", HeaderValue::from_static("AudioReader/1.0"));
                headers.insert("Accept-Ranges", HeaderValue::from_static("bytes"));
                headers
            })
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .pool_max_idle_per_host(6)
            .pool_idle_timeout(Duration::from_secs(90))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            url,
            client,
            position: Arc::new(Mutex::new(0)),
            chunks: Arc::new(Mutex::new(HashMap::new())),
            runtime: Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime")),
        }
    }

    pub fn wait_preload(&self, chunk_count: u64) -> Result<(), AppError> {
        let start_chunk = *self.position.lock()? / CHUNK_SIZE as u64;
        debug!("Preloading {} data chunks...", chunk_count);

        let handles: Vec<_> = (0..chunk_count)
            .map(|i| {
                let self_clone = self.clone();
                let chunk_index = start_chunk + i;
                std::thread::spawn(move || self_clone.get_chunk(chunk_index))
            })
            .collect();

        for handle in handles {
            if let Err(e) = handle.join() {
                error!("Preload task failed: {:?}", e);
            }
        }
        Ok(())
    }

    fn fetch_chunk(&self, chunk_index: u64) -> Result<Arc<Vec<u8>>, AppError> {
        let (tx, rx) = std::sync::mpsc::channel();
        let client = self.client.clone();
        let url = self.url.clone();

        self.runtime.spawn(async move {
            let start = chunk_index * CHUNK_SIZE as u64;
            let range_header = format!("bytes={}-{}", start, start + CHUNK_SIZE as u64 - 1);

            let result = async {
                let response = client.get(&url).header(RANGE, range_header).send().await?;
                if response.status().is_success() || response.status().as_u16() == 206 {
                    Ok(Arc::new(response.bytes().await?.to_vec()))
                } else {
                    Err(AppError::Network(format!(
                        "HTTP {} for chunk {}",
                        response.status(),
                        chunk_index
                    )))
                }
            }
            .await;

            let _ = tx.send(result);
        });

        rx.recv()?
    }

    fn get_chunk(&self, chunk_index: u64) -> Result<Arc<Vec<u8>>, AppError> {
        if let Some(chunk) = self.chunks.lock()?.get(&chunk_index) {
            return Ok(Arc::clone(chunk));
        }

        let chunk_data = self.fetch_chunk(chunk_index)?;
        let mut chunks = self.chunks.lock()?;

        if chunks.len() >= MAX_CACHED_CHUNKS {
            let current_chunk = *self.position.lock()? / CHUNK_SIZE as u64;
            let keep_range = current_chunk.saturating_sub(2)..=current_chunk + 6;

            let old_len = chunks.len();
            chunks.retain(|&k, _| keep_range.contains(&k));

            while chunks.len() >= MAX_CACHED_CHUNKS {
                if let Some(&farthest_key) = chunks.keys().max_by_key(|&&k| {
                    if k > current_chunk {
                        k - current_chunk
                    } else {
                        current_chunk - k
                    }
                }) {
                    chunks.remove(&farthest_key);
                } else {
                    break;
                }
            }

            debug!(
                "Cache cleanup: reduced from {} to {} data chunks",
                old_len,
                chunks.len()
            );
        }

        chunks.insert(chunk_index, Arc::clone(&chunk_data));
        Ok(chunk_data)
    }
}

impl Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let position = *self
            .position
            .lock()
            .map_err(|e| std::io::Error::other(e.to_string()))?;
        let chunk_index = position / CHUNK_SIZE as u64;
        let chunk_offset = (position % CHUNK_SIZE as u64) as usize;

        let chunk_data = self
            .get_chunk(chunk_index)
            .map_err(|e| std::io::Error::other(format!("Failed to fetch chunk: {}", e)))?;

        let available = chunk_data.len().saturating_sub(chunk_offset);
        if available == 0 {
            return Ok(0);
        }

        let to_read = std::cmp::min(buf.len(), available);
        buf[..to_read].copy_from_slice(&chunk_data[chunk_offset..chunk_offset + to_read]);

        *self
            .position
            .lock()
            .map_err(|e| std::io::Error::other(e.to_string()))? += to_read as u64;

        if chunk_offset + to_read > CHUNK_SIZE * 2 / 3 {
            let self_clone = self.clone();
            let next_chunk = chunk_index + 1;
            std::thread::spawn(move || {
                for i in 0..2 {
                    if self_clone.get_chunk(next_chunk + i).is_err() {
                        break;
                    }
                }
            });
        }

        Ok(to_read)
    }
}

impl Seek for Reader {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let current = *self
            .position
            .lock()
            .map_err(|e| std::io::Error::other(e.to_string()))?;

        let new_position = match pos {
            SeekFrom::Start(offset) => offset,
            SeekFrom::Current(offset) => (current as i64 + offset).max(0) as u64,
            SeekFrom::End(offset) => (current as i64 + offset).max(0) as u64,
        };

        *self
            .position
            .lock()
            .map_err(|e| std::io::Error::other(e.to_string()))? = new_position;

        let chunk_index = new_position / CHUNK_SIZE as u64;
        let self_clone = self.clone();
        std::thread::spawn(move || {
            for i in chunk_index.saturating_sub(3)..=chunk_index + 3 {
                if self_clone.get_chunk(i).is_err() {
                    break;
                }
            }
        });

        Ok(new_position)
    }
}
