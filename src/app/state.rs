use crate::ui::components::calendar::Calendar;
// app/state.rs
use crate::{domain::afd::parser::parse_afd_lines, ui::screens::Screen};
use crate::domain::afd::afd::AFD;
use crate::infra::afd_loader::decode_from_win1252_to_utf8;
use crate::domain::tally::tally::Tally;
use chrono::{NaiveDateTime, Local};
use std::path::PathBuf;

#[derive(Default)]
pub struct AppState {
    pub current_screen: Screen,
    pub afd: AFD,
    pub tally: HashMap<NaiveDateTime, Tally>,
    pub last_afd_got: Option<NaiveDateTime>,
    pub last_add_info_got: Option<NaiveDateTime>,
    pub sel_dates: Calendar
}

impl AppState {
    pub fn load_afd(&mut self, path: PathBuf) {
        if let Some(decoded) = decode_from_win1252_to_utf8(path){
            self.afd = AFD::default();
            let agora_local = Local::now().naive_local();
            self.last_afd_got = Some(agora_local);
            parse_afd_lines(self, decoded);
        }
    }
}
