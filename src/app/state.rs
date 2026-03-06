// app/state.rs
use crate::ui::screens::Screen;
use crate::domain::afd::afd::AFD;
use chrono::NaiveDateTime;

#[derive(Default)]
pub struct AppState {
    pub current_screen: Screen,
    pub afd: AFD,
    pub last_afd_got: Option<NaiveDateTime>
}

