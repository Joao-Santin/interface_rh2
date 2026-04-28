use crate::domain::afd::afd::MarcacaoPonto;
use crate::domain::info_add::info_add::{InfoAdd, TypeOrigin};
use chrono::{NaiveDateTime, NaiveDate};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tally{
    pub cpf: String,
    pub origin: TypeOrigin,
    pub entry_manu: Option<NaiveDateTime>,//mostra a marcacao manual feita
    pub entry_marca: Option<NaiveDateTime>,// mostra a marcacao feita pela maquina de pontos.
    pub entry_display: NaiveDateTime,// este declara qual será mostrado na tela.
}
pub fn calculate_tally(
    infoadd: InfoAdd,
    marcacaoponto: Vec<MarcacaoPonto>
) -> Vec<(NaiveDateTime, Tally)> {

    let mut vec_verify: Vec<(NaiveDateTime, Tally)> = Vec::new();

    use std::collections::HashSet;

    let mut corrigidos: HashSet<(String, NaiveDateTime)> = HashSet::new();

    // identifica quais AFD foram corrigidos
    for ponto_manual in &infoadd.manualponto {
        if let TypeOrigin::Correcao(ref original) = ponto_manual.typemanual {
            corrigidos.insert((
                ponto_manual.cpf_empregado.clone(),
                original.date_time
            ));
        }
    }

    // Pontos manuais
    for ponto_manual in infoadd.manualponto {
        let data_original = if let TypeOrigin::Correcao(ref original) = ponto_manual.typemanual {
            Some(original.date_time)
        } else {
            None
        };

        vec_verify.push((
            ponto_manual.date_time,
            Tally {
                cpf: ponto_manual.cpf_empregado.clone(),
                origin: ponto_manual.typemanual,
                entry_manu: Some(ponto_manual.date_time),
                entry_marca: data_original,
                entry_display: ponto_manual.date_time,
            }
        ));
    }

    // Pontos da máquina (AFD)
    for ponto_afd in marcacaoponto {

        if corrigidos.contains(&(ponto_afd.cpf_empregado.clone(), ponto_afd.date_time)) {
            continue;
        }

        vec_verify.push((
            ponto_afd.date_time,
            Tally {
                cpf: ponto_afd.cpf_empregado,
                origin: TypeOrigin::AFD,
                entry_manu: None,
                entry_marca: Some(ponto_afd.date_time),
                entry_display: ponto_afd.date_time,
            }
        ));
    }
    vec_verify
}
pub fn group_tally_by_day(tallys: &Vec<(NaiveDateTime, Tally)>)->Vec<(NaiveDate, Vec<(NaiveDateTime, Tally)>)>{
    let mut sorted = tallys.clone();
    sorted.sort_by_key(|(dt, _)| *dt);
    let mut grouped: Vec<(NaiveDate, Vec<(NaiveDateTime, Tally)>)> = Vec::new();
    for (dt, tally) in sorted {
        if let Some((last_date, vec)) = grouped.last_mut() {
            if *last_date == dt.date() {
                vec.push((dt, tally));
                continue;
            }
        }
        grouped.push((dt.date(), vec![(dt, tally)]));
    }
    grouped.sort_by_key(|(date, _)| *date);
    grouped
}
