use std::collections::HashMap;

use chrono::{NaiveDateTime, NaiveDate};
use serde::{Serialize, Deserialize};
use crate::domain::afd::afd::MarcacaoPonto;
use crate::domain::day_result::day_result::{DayResult};

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
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Copy)]
pub enum DayOffType{
    Holiday,//Feriados, usado pela empresa,n usa BH, n perde BON
    Vacation,// Ferias de 1 pessoa n usa BH, n perde BON
    SickLeave,// Atestado médico n usa BH, perde BOn
    ProgrammedLeave,// Programou saida e usa BH, n perde bon
    ParcialProgrammedLeave,// usa BH, n perde bon
    CollectiveLeave,// n usa BH, n perde BON
    MedicalLeave,// n usa BH, perde BON
}
impl DayOffType{
    pub fn uses_bh(&self)->bool{
        matches!(
        self,
            DayOffType::ProgrammedLeave | DayOffType::ParcialProgrammedLeave
    )
    }
}
impl std::fmt::Display for DayOffType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self{
            Self::CollectiveLeave => "Ferias Coletiva",
            Self::SickLeave => "Atestado",
            Self::Vacation => "Ferias",
            Self::Holiday => "Feriado",
            Self::MedicalLeave => "Afastamento Medico",
            Self::ProgrammedLeave => "Programa Banco Horas",
            Self::ParcialProgrammedLeave => "Programa Banco Parcial"
        })
    }
}
pub fn is_eligible_for_bonus(
    start: NaiveDate,
    end: NaiveDate,
    cpf: &str,
    employee_day_offs: &[EmployeeDayOff],
    day_results: &[DayResult],
) -> bool {

    let has_blocking_day_off = employee_day_offs
        .iter()
        .filter(|d| d.cpf == cpf)
        .filter(|d| d.start <= end && d.end >= start)
        .any(|d| matches!(
            d.typ,
            DayOffType::MedicalLeave
                | DayOffType::SickLeave
        ));
    let has_absence = day_results
        .iter()
        .filter(|dr| dr.cpf.trim() == cpf.trim())
        .filter(|dr| dr.date >= start && dr.date <= end)
        .any(|dr| {
            let is_programmed_leave = employee_day_offs.iter().any(|edo| {
                edo.cpf.trim() == cpf.trim()
                    && dr.date >= edo.start
                    && dr.date <= edo.end
                    && matches!(
                        edo.typ,
                        DayOffType::ProgrammedLeave | DayOffType::ParcialProgrammedLeave
                    )
            });

            // só considera falta se NÃO for programado
            dr.expected > chrono::Duration::zero()
                && dr.worked < dr.expected
                && !is_programmed_leave
        });
    // let has_absence = day_results
    //     .iter()
    //     .filter(|dr| dr.cpf.trim() == cpf.trim())
    //     .filter(|dr| dr.date >= start && dr.date <= end)
    //     .any(|dr| {
    //         dr.expected > chrono::Duration::zero() && dr.worked < dr.expected
    //     });

    !(has_blocking_day_off || has_absence)
}
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct CompanyDayOff{
    pub typ: DayOffType,
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub more_info: String,
}
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct EmployeeDayOff{
    pub cpf: String,
    pub typ: DayOffType,
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub more_info: String,
    pub uses_time_off_balance: bool,
    // pub perde_bonificacao:  bool,
}
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct InfoAdd{
    #[serde(default)]
    pub manualponto: Vec<ManualPonto>,
    #[serde(default)]
    pub company_day_off: Vec<CompanyDayOff>,
    #[serde(default)]
    pub employee_day_off: Vec<EmployeeDayOff>,
    #[serde(default)]
    pub bh_legado: HashMap<String, i32>
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
    pub fn create_company_day_off(&mut self, novo: CompanyDayOff){
        self.company_day_off.push(novo);
    }
    pub fn delete_company_day_off(&mut self, novo: CompanyDayOff){
        self.company_day_off.retain(|cdo| {
            !(cdo.typ == novo.typ && cdo.start == novo.start && cdo.end == novo.end)
        });
    }
    pub fn create_employee_day_off(&mut self, novo: EmployeeDayOff){
        self.employee_day_off.push(novo);
    }
    pub fn delete_employee_day_off(&mut self, novo: EmployeeDayOff){
        self.employee_day_off.retain(|cdo| {
            !(cdo.typ == novo.typ && cdo.start == novo.start && cdo.end == novo.end)
        });
    }
}
