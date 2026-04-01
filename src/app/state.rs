use crate::domain::info_add::info_add::InfoAdd;
use crate::ui::components::textinputs::TextInputs;
use crate::ui::components::calendar::Calendar;
// app/state.rs
use crate::{domain::afd::parser::parse_afd_lines, ui::screens::Screen};
use crate::domain::afd::afd::{AFD, get_funcionarios};
use crate::infra::afd_loader::decode_from_win1252_to_utf8;
use crate::domain::tally::tally::Tally;
use crate::infra::info_add_loader::load_info_add;

use chrono::{NaiveDateTime, Local};
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Default)]
pub struct AppState {
    pub current_screen: Screen,
    pub text_inputs: TextInputs,
    pub afd: AFD,//varios vecs, não ordenado
    pub info_add: InfoAdd,
    pub tally: HashMap<NaiveDateTime, Tally>,//separa por NAIVEDATETIME
    pub employees: HashMap<String, String>,
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
            self.employees = get_funcionarios(self);
            println!("{:?}", self.employees)
        }
    }
    pub fn load_info_add(&mut self, path: PathBuf){
        match load_info_add(path) {
            Ok(info_add) => {
                self.info_add = info_add;
            }
            Err(e) => {
                eprintln!("Erro ao carregar tally:{}", e)
            }
        }
        
    }
}
