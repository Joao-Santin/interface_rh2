use crate::domain::afd::afd::MarcacaoPonto;
use crate::domain::info_add::info_add::{InfoAdd, TypeOrigin};
use std::collections::HashMap;
use chrono::{NaiveDateTime};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tally{
    pub cpf: String,
    pub origin: TypeOrigin,
    pub entry_manu: Option<NaiveDateTime>,//mostra a marcacao manual feita
    pub entry_marca: Option<NaiveDateTime>,// mostra a marcacao feita pela maquina de pontos.
    pub entry_display: NaiveDateTime,// este declara qual será mostrado na tela.
}

pub fn calculate_tally(infoadd: InfoAdd, marcacaoponto: Vec<MarcacaoPonto>)->HashMap<NaiveDateTime, Tally>{
    let mut hash_verify: HashMap<NaiveDateTime, Tally> = HashMap::new(); // item para retorno no
    // campo tally do Appstate
    for ponto_manual in infoadd.manualponto{
        let data_original = if let TypeOrigin::Correcao(ref original) = ponto_manual.typemanual{
            Some(original.date_time)
        }else{
            None
        };
        hash_verify.insert(ponto_manual.date_time, Tally{
            cpf: ponto_manual.cpf_empregado.clone(),
            origin: ponto_manual.typemanual,
            entry_manu: Some(ponto_manual.date_time),
            entry_marca: data_original,
            entry_display: ponto_manual.date_time,
        });
    }
    for ponto_afd in marcacaoponto{
        if !hash_verify.contains_key(&ponto_afd.date_time){
            hash_verify.insert(ponto_afd.date_time, Tally{
                cpf: ponto_afd.cpf_empregado,
                origin: TypeOrigin::AFD,
                entry_manu: None,
                entry_marca: Some(ponto_afd.date_time),
                entry_display: ponto_afd.date_time
            });
        }
    }
    hash_verify
}
