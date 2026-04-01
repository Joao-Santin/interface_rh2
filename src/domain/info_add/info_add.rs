use chrono::{NaiveDateTime};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::domain::afd::afd::MarcacaoPonto;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TypeOrigin{
    AFD,
    Correcao(MarcacaoPonto),
    Criacao,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct ManualPonto{
    pub typemanual:TypeOrigin,
    pub date_time: NaiveDateTime,
    pub cpf_empregado: String,
}
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct InfoAdd{
    pub manualponto: Vec<ManualPonto>
}
