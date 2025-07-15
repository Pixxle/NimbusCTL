pub mod state;
pub mod events;
pub mod config;
pub mod startup;
pub mod settings;

use crate::utils::error::Result;
use crossterm::event::KeyEvent;
use state::AppState;

pub struct App {
    pub state: AppState,
}

impl App {
    pub async fn new() -> Result<Self> {
        let state = AppState::new().await?;
        Ok(Self { state })
    }

    pub async fn handle_input(&mut self, key: KeyEvent) -> Result<()> {
        self.state.handle_input(key).await
    }

    pub async fn update(&mut self) -> Result<()> {
        self.state.update().await
    }
}