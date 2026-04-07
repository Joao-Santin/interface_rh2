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
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ManualPonto{
    pub typemanual:TypeOrigin,
    pub date_time: NaiveDateTime,
    pub cpf_empregado: String,
}
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct InfoAdd{
    pub manualponto: Vec<ManualPonto>
}
impl InfoAdd{
    pub fn create(&mut self, novo: ManualPonto){
        self.manualponto.push(novo);
    }
    pub fn update(
        &mut self,
        cpf: &str,
        old_datetime: NaiveDateTime,
        new_datetime: NaiveDateTime,
    ) -> bool{
        if let Some(item) = self.manualponto.iter_mut().find(|m|{
            m.cpf_empregado == cpf && m.date_time == old_datetime
        }) {
            item.date_time = new_datetime;
            true
        }else{
            false
        }

    }
    pub fn delete(
        &mut self,
        cpf: &str,
        datetime: NaiveDateTime,
    ) -> bool{
        let initial_len = self.manualponto.len();
        self.manualponto.retain(|m| {
            !(m.cpf_empregado == cpf && m.date_time == datetime)
        });
        initial_len != self.manualponto.len()
    }
}
