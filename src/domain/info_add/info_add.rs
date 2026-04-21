use chrono::{NaiveDateTime, NaiveDate};
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
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum DayOffType{
    Holiday,
    Vacation,
    SickLeave,
    CollectiveLeave,
    Other(String),
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CompanyDayOff{
    pub typ: DayOffType,
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub more_info: String,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EmployeeDayOff{
    pub cpf: String,
    pub typ: DayOffType,
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub more_info: String,
    pub uses_time_off_balance: bool,
}
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct InfoAdd{
    #[serde(default)]
    pub manualponto: Vec<ManualPonto>,
    #[serde(default)]
    pub company_day_off: Vec<CompanyDayOff>,
    #[serde(default)]
    pub employee_day_off: Vec<EmployeeDayOff>
}
impl InfoAdd{
    pub fn create_manual_ponto(&mut self, novo: ManualPonto){
        self.manualponto.push(novo);
    }
    pub fn edit_manual_ponto(&mut self, novo: ManualPonto){
        self.manualponto.push(novo)
    }
    pub fn delete_manual_ponto(
        &mut self,
        delete: ManualPonto,
    ){
        self.manualponto.retain(|m| {
            !(m.cpf_empregado == delete.cpf_empregado && m.date_time == delete.date_time)
        });
    }
}
