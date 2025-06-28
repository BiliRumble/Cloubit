use log::{debug, error};
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;
use tokio::sync::broadcast;

use crate::models::audio::BackendState;

pub struct AudioBackend {
    pub event_sender: broadcast::Sender<BackendState>,
}

// 实现有点傻逼，但是跑起来了
impl AudioBackend {
    pub fn new() -> Self {
        let (event_sender, event_receiver) = broadcast::channel(100);

        std::thread::spawn(move || {
            let stream_result = OutputStream::try_default();
            if let Err(e) = &stream_result {
                error!("Failed to initialize outputStream: {}", e);
                return;
            }

            let (_stream, stream_handle) = stream_result.unwrap();
            let sink_result = Sink::try_new(&stream_handle);
            if let Err(e) = &sink_result {
                error!("Failed to create sink: {}", e);
                return;
            }

            let sink = sink_result.unwrap();

            let rt_result = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build();

            if let Err(e) = &rt_result {
                error!("Failed to build runtime: {}", e);
                return;
            }

            let rt = rt_result.unwrap();

            rt.block_on(async {
                let mut receiver = event_receiver;
                while let Ok(e) = receiver.recv().await {
                    match e {
                        BackendState::Set(target) => {
                            sink.clear();

                            let resp = match tauri_plugin_http::reqwest::get(&target).await {
                                Ok(r) => r,
                                Err(e) => {
                                    error!("Failed to fetch {}: {}", target, e);
                                    continue;
                                }
                            };

                            if !resp.status().is_success() {
                                error!(
                                    "Server returned error status: {} for {}",
                                    resp.status(),
                                    target
                                );
                                continue;
                            }

                            let bytes = match resp.bytes().await {
                                Ok(b) => b,
                                Err(e) => {
                                    error!("Failed to read data: {}", e);
                                    continue;
                                }
                            };

                            let cursor = Cursor::new(bytes);

                            let source = match Decoder::new(cursor) {
                                Ok(s) => s,
                                Err(e) => {
                                    error!("Failed to decode data: {}", e);
                                    continue;
                                }
                            };

                            sink.append(source);
                            sink.play();
                        }
                        BackendState::Play(resume) => {
                            if resume {
                                sink.play();
                            } else {
                                sink.pause();
                            }
                        }
                        BackendState::Speed(speed) => {
                            sink.set_speed(speed);
                        }
                        BackendState::Volume(volume) => {
                            sink.set_volume(volume);
                        }
                        BackendState::Seek(pos) => {
                            match sink.try_seek(std::time::Duration::from_secs(pos)) {
                                Ok(_) => debug!("Seeked to position {}s", pos),
                                Err(e) => error!("Failed to seek: {}", e),
                            }
                        }
                    }
                }
            });
        });

        Self { event_sender }
    }
}
