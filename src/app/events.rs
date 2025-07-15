use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum AppEvent {
    Key(KeyEvent),
    Tick,
    Resize(u16, u16),
    Quit,
}

pub struct EventHandler {
    receiver: mpsc::Receiver<AppEvent>,
    _sender: mpsc::Sender<AppEvent>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(100);
        let _sender = sender.clone();

        // Spawn event listener task
        tokio::spawn(async move {
            loop {
                if let Ok(event) = crossterm::event::read() {
                    match event {
                        Event::Key(key) => {
                            if sender.send(AppEvent::Key(key)).await.is_err() {
                                break;
                            }
                        }
                        Event::Resize(width, height) => {
                            if sender.send(AppEvent::Resize(width, height)).await.is_err() {
                                break;
                            }
                        }
                        _ => {}
                    }
                }

                tokio::time::sleep(Duration::from_millis(16)).await;
            }
        });

        Self { receiver, _sender }
    }

    pub async fn next(&mut self) -> Option<AppEvent> {
        self.receiver.recv().await
    }
}
