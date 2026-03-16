use chrono::{NaiveDateTime};
use std::collections::HashMap;

use crate::domain::afd::afd::MarcacaoPonto;

pub enum TypeOrigin{
    AFD,
    Correcao(MarcacaoPonto),
    Criacao,
}
pub struct ManualPonto{
    pub typemanual:TypeOrigin,
    pub date_time: NaiveDateTime,
    pub cpf_empregado: String,
}
#[derive(Default)]
pub struct InfoAdd{
    pub manualponto: HashMap<String, Vec<ManualPonto>>
}
