use chrono::{NaiveDateTime};
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
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct InfoAdd{
    pub manualponto: Vec<ManualPonto>
}
impl InfoAdd{
    pub fn create(&mut self, novo: ManualPonto){
        self.manualponto.push(novo);
    }
    pub fn edit(&mut self, novo: ManualPonto){
        self.manualponto.push(novo)
    }
    pub fn delete(
        &mut self,
        delete: ManualPonto,
    ){
        self.manualponto.retain(|m| {
            !(m.cpf_empregado == delete.cpf_empregado && m.date_time == delete.date_time)
        });
    }
}
