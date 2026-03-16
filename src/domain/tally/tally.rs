use crate::domain::afd::afd::MarcacaoPonto;
use crate::domain::info_add::info_add::{InfoAdd, ManualPonto};
use std::collections::HashMap;
use chrono::{NaiveDateTime};
pub struct Tally{
    cpf: String,
    origin: ManualPonto,
    entry_manu: Option<NaiveDateTime>,//mostra a marcacao manual feita
    entry_marca: Option<NaiveDateTime>,// mostra a marcacao feita pela maquina de pontos.
    entry_display: NaiveDateTime,// este declara qual será mostrado na tela.
}
fn calculate_tally(infoadd: InfoAdd, marcacaoponto: MarcacaoPonto)->Vec<Tally>{
    let mut tallies = Vec::new(); // aqui vai ser os tally para retorno.
    let mut hash_verify: HashMap<NaiveDateTime, Tally> = HashMap::new(); // aqui vai ser um HashSet que
    // vai aceita
    for ponto_manual in infoadd.manualponto.values(){
    }
    tallies
}
