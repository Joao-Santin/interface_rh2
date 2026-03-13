use chrono::{NaiveDateTime};
use std::collections::HashMap;

use crate::domain::afd::afd::MarcacaoPonto;

pub enum TypeManualPonto{
    Correcao(MarcacaoPonto),
    Criacao,
}
pub struct ManualPonto{
    typemanual:TypeManualPonto,
    date_time: NaiveDateTime,
    cpf_empregado: String,
}
#[derive(Default)]
pub struct InfoAdd{
    manualponto: HashMap<String, Vec<ManualPonto>>
}
