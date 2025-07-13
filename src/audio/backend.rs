use crate::AppError;
use log::{debug, error};
use rodio::{Decoder, OutputStream, Sink};
use std::sync::mpsc;
use std::thread;

use crate::models::audio::BackendState;

pub struct AudioBackend {
    pub command_sender: mpsc::Sender<BackendState>,
}

// 优化过的狗屎
impl AudioBackend {
    pub fn new() -> Result<Self, AppError> {
        let (command_sender, command_receiver) = mpsc::channel();

        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            if let Err(e) = rt.block_on(Self::audio_thread_main(command_receiver)) {
                error!("Audio thread terminated with error: {}", e);
            }
        });

        Ok(Self { command_sender })
    }

    async fn audio_thread_main(receiver: mpsc::Receiver<BackendState>) -> Result<(), AppError> {
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

                let reader = crate::audio::reader::Reader::new(target.to_string());
                if let Err(e) = reader.wait_preload(3) {
                    return Err(AppError::Audio(format!(
                        "Failed to preload audio data: {}",
                        e
                    )));
                }

                sink.append(Decoder::new(reader.clone())?);
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
