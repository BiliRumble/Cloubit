use crate::AppError;
use log::{debug, error};
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;
use std::sync::mpsc;
use std::thread;

use crate::models::audio::BackendState;

pub struct AudioBackend {
    pub command_sender: mpsc::Sender<BackendState>,
}

// 优化过的狗屎
impl AudioBackend {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (command_sender, command_receiver) = mpsc::channel();

        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            if let Err(e) = rt.block_on(Self::audio_thread_main(command_receiver)) {
                error!("Audio thread terminated with error: {}", e);
            }
        });

        Ok(Self { command_sender })
    }

    async fn audio_thread_main(
        receiver: mpsc::Receiver<BackendState>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        while let Ok(command) = receiver.recv() {
            if let Err(e) = Self::handle_command(&sink, command).await {
                error!("Failed to handle audio command: {}", e);
            }
        }

        drop(sink);
        Ok(())
    }

    async fn handle_command(sink: &Sink, command: BackendState) -> Result<(), AppError> {
        match command {
            BackendState::Set(target) => {
                sink.clear();
                let resp = reqwest::get(target).await?;
                if !resp.status().is_success() {
                    return Err(AppError::Network("Could not download audio".to_string()));
                }

                let bytes = resp.bytes().await?;
                let cursor = Cursor::new(bytes);
                let source = Decoder::new(cursor)?;

                sink.append(source);
                sink.play();
                // pass
            }
            BackendState::Play(resume) => {
                if resume {
                    sink.play();
                } else {
                    sink.pause();
                }
                // pass
            }
            BackendState::Speed(speed) => {
                sink.set_speed(speed);
                // pass
            }
            BackendState::Volume(volume) => {
                sink.set_volume(volume);
                // pass
            }
            BackendState::Seek(pos) => {
                sink.try_seek(std::time::Duration::from_secs(pos))?;
                debug!("Seeked to position {}s", pos);
                // pass
            }
        }
        Ok(())
    }

    pub fn dummy() -> Self {
        let (command_sender, _) = mpsc::channel();
        Self { command_sender }
    }
}
