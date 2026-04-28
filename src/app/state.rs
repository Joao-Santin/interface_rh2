use crate::domain::info_add::info_add::{InfoAdd, DayOffType};
use crate::ui::components::textinputs::TextInputs;
use crate::ui::components::calendar::Calendar;
// app/state.rs
use crate::{domain::afd::parser::parse_afd_lines, ui::screens::Screen};
use crate::domain::afd::afd::{AFD, get_funcionarios};
use crate::infra::afd_loader::decode_from_win1252_to_utf8;
use crate::domain::tally::tally::{calculate_tally, Tally};
use crate::domain::day_result::day_result::{DayResult};
use crate::infra::info_add_loader::load_info_add;

use chrono::{NaiveDateTime, Local};
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[derive(Default)]
pub struct AppState {
    pub current_screen: Screen,
    pub text_inputs: TextInputs,
    pub picked_dayoff_company_creating: Option<DayOffType>,
    pub picked_dayoff_employee_creating: Option<DayOffType>,
    pub uses_time_off_balance_dayoff_employee_creating: bool,
    pub afd: AFD,//varios vecs, não ordenado
    pub info_add: InfoAdd,
    pub tally: Vec<(NaiveDateTime, Tally)>,//separa por NAIVEDATETIME
    pub day_result: Vec<DayResult>,//resultado de trabalhado por dia
    pub employees: HashMap<String, String>,
    pub last_afd_got: Option<NaiveDateTime>,
    pub last_add_info_got: Option<NaiveDateTime>,
    pub sel_dates: Calendar
}

impl AppState {
    pub fn save_info_add(&mut self, path: PathBuf){
        match File::create(&path) {
            Ok(mut file) => {
                match serde_json::to_string_pretty(&self.info_add) {
                    Ok(json) => {
                        if let Err(e) = file.write_all(json.as_bytes()) {
                            println!("Erro ao escrever arquivo: {}", e);
                        } else {
                            println!("Salvo com sucesso em {:?}", path);
                        }
                    }
                    Err(e) => {
                        println!("Erro ao serializar JSON: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Erro ao criar arquivo: {}", e);
            }
        }

    }
    pub fn load_tally(&mut self) {
        self.tally = calculate_tally(self.info_add.clone(), self.afd.marcacaoponto.clone());
        self.day_result = DayResult::from_appstate(&self)
    }
    pub fn load_afd(&mut self, path: PathBuf) {
        if let Some(decoded) = decode_from_win1252_to_utf8(path){
            self.afd = AFD::default();
            let agora_local = Local::now().naive_local();
            self.last_afd_got = Some(agora_local);
            parse_afd_lines(self, decoded);
            self.employees = get_funcionarios(self);
            self.load_tally()
        }
    }
    pub fn load_info_add(&mut self, path: PathBuf){
        match load_info_add(path) {
            Ok(info_add) => {
                self.info_add = info_add;
                let agora_local = Local::now().naive_local();
                self.last_add_info_got = Some(agora_local);
                self.load_tally()
            }
            Err(e) => {
                eprintln!("Erro ao carregar info_dd:{}", e)
            }
        }
        
    }

}
