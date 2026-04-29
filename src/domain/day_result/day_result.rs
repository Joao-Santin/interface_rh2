use chrono::{Duration, NaiveDate, NaiveDateTime};
use std::collections::HashSet;

use crate::domain::tally::tally::{Tally, group_tally_by_day};
use crate::extensions::chrono_ext::{NaiveDateExt, NaiveDateTimePtBr};
use crate::domain::info_add::info_add::{InfoAdd};
use crate::AppState;

#[derive(Debug)]
pub struct DayResult{
    pub cpf: String,
    pub date: NaiveDate,
    pub worked: Duration,
    pub expected: Duration,
    pub balance: Duration,
}

pub fn calculate_worked_time(registros: &Vec<(NaiveDateTime, Tally)>)-> Duration{
    let mut total = chrono::Duration::zero();
    let mut pontos: Vec<NaiveDateTime> = registros.iter().map(|(dt, _)| dt.round_with_tolerance()).collect();
    pontos.sort();
    let mut iter = pontos.iter();
    while let (Some(start), Some(end)) = (iter.next(), iter.next()) {
        let diff = *end - *start;
        if diff> Duration::zero(){
            total = total + diff;
        }
    }
    total
}
pub fn total_balance_by_cpf(day_results:&[DayResult], cpf: &String) -> Duration{
    day_results
        .iter()
        .filter(|dr| dr.cpf.trim() == cpf.clone().trim())
        .map(|dr| dr.balance)
        .sum()
}
pub fn total_balance_by_cpf_with_dates(start: NaiveDate, end: NaiveDate, day_results:&[DayResult], cpf: &String) -> Duration{
    day_results
        .iter()
        .filter(|dr| dr.cpf.trim() ==cpf.trim())
        .filter(|dr| dr.date >= start && dr.date <= end)
        .map(|dr| dr.balance)
        .sum()
}

impl DayResult{
    pub fn from_group(cpf: String, registro_dia: (NaiveDate, Vec<(NaiveDateTime, Tally)>), info: &InfoAdd)->Self{
        let (date, registros) = registro_dia;
        let worked = calculate_worked_time(&registros);
        let is_company_day_off = info.company_day_off.iter().any(|cdo|{
            date>=cdo.start && date <= cdo.end
        });
        let is_employee_day_off = info.employee_day_off.iter().any(|edo|{
            edo.cpf.trim() == cpf.trim() && date >= edo.start && date <= edo.end
        });
        let expected = if is_company_day_off || is_employee_day_off {
            chrono::Duration::zero()
        }else{
            date.get_workday_duration()
        };
        let balance = worked - expected;
        Self{
            cpf,
            date,
            worked,
            expected,
            balance,
        }
    }
    pub fn from_appstate(state: &AppState)->Vec<DayResult>{
        let grouped_tally = group_tally_by_day(&state.tally);
        if grouped_tally.is_empty(){
            println!("Grouped Tally is empty")
        }
        let min_date = NaiveDate::parse_from_str("01-04-2026", "%d-%m-%Y").unwrap();
        let max_date = grouped_tally.iter().map(|(d, _)| *d).max().unwrap();
        let mut resultados = Vec::new();
        let mut current = min_date;

        while current <= max_date{
            let registros_do_dia = grouped_tally
                .iter()
                .find(|(d, _)| *d == current)
                .map(|(_, r)| r.clone())
                .unwrap_or_else(|| Vec::new());

            let cpfs: HashSet<String> = state.employees.keys().cloned().collect();
            // let cpfs: HashSet<String> = if registros_do_dia.is_empty() {
            //     state.employees.keys().cloned().collect()
            // } else {
            //     registros_do_dia
            //         .iter()
            //         .map(|(_, t)| t.cpf.clone())
            //         .collect()
            // };

            for cpf in cpfs {
                let filtrados: Vec<_> = registros_do_dia
                    .iter()
                    .filter(|(_, t)| t.cpf == cpf)
                    .cloned()
                    .collect();

                resultados.push(
                    DayResult::from_group(
                        cpf,
                        (current, filtrados),
                        &state.info_add
                    )
                );
            }
            current = current + Duration::days(1);
        }        resultados
    }
}
        // let resultados: Vec<DayResult> = grouped_tally
        //     .iter()
        //     .map(|(date, registros)| {
        //         let mut results = Vec::new();
        //
        //         let cpfs: HashSet<String> = registros
        //             .iter()
        //             .map(|(_, t)| t.cpf.clone())
        //             .collect();
        //
        //         for cpf in cpfs {
        //             let filtrados: Vec<_> = registros
        //                 .iter()
        //                 .filter(|(_, t)| t.cpf == cpf)
        //                 .cloned()
        //                 .collect();
        //
        //             results.push(
        //                 DayResult::from_group(
        //                     cpf,
        //                     (*date, filtrados),
        //                     &state.info_add
        //                 )
        //             );
        //         }
        //
        //         results
        //     })
        //     .flatten()
        //     .collect();

