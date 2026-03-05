// app/state.rs
use crate::ui::screens::Screen;

#[derive(Default)]
pub struct AppState {
    pub current_screen: Screen,
}

