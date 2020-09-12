use crate::prelude::*;
use todo_web_shared::views::Renderer;

#[derive(Debug)]
pub struct AppState {
    renderer: Renderer,
}

impl AppState {
    #[instrument]
    pub async fn load() -> Result<Self> {
        let renderer = Renderer::new()?;
        let state = Self { renderer };
        Ok(state)
    }

    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }
}
