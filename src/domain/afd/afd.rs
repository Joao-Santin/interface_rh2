use chrono::{NaiveDate, NaiveDateTime};
use serde::{Serialize, Deserialize};
use crate::app::state::AppState;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RegistryType{
    Cabecalho,
    CreateUpdateEmpresa,
    MarcacaoPonto,
    AjusteRelogio,
    CreateUpdateDeleteEmpregado,
    SensivelREP,
    MarcacaoPontoRepP,
    Trailer,
}



#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AFDBase{
    nsr: String,
    tipo: RegistryType
}
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Cabecalho{
    base: AFDBase,
    tipo_empregador: String,
    cnpj_empregador: String,
    cno_empregador: String,
    razao_social: String,
    id_rep: String,
    data_inicio: NaiveDate,
    data_final: NaiveDate,
    geracao_arquivo: String,
    versao_leiaute_afd: String,
    cnpj_fabricante_rep: String,
    modelo_rep: String,
    registro_hexa: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone, Debug)]
struct CreateUpdateEmpresa{
    base: AFDBase,
    date_time: NaiveDateTime,
    cpf_admin: String,
    tipo_empregador: String,
    cnpj_empregador: String,
    cno: String,
    razao_social: String,
    local_servico: String,
    registro_hexa: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MarcacaoPonto{
    pub base: AFDBase,
    pub date_time: NaiveDateTime,
    pub cpf_empregado: String,
    pub registro_hexa: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone, Debug)]
struct AjusteRelogio{
    base: AFDBase,
    date_time_antes_registro: NaiveDateTime,
    date_time_ajustado: NaiveDateTime,
    cpf_adm: String,
    registro_hexa: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone, Debug)]
struct CreateUpdateDeleteEmpregado{
    base: AFDBase,
    date_time: NaiveDateTime,
    tipo_operacao: String,
    cpf_empregado: String,
    nome_empregado: String,
    mais_dados_empregado: String,
    cpf_adm: String,
    registro_hexa: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone, Debug)]
struct SensivelREP{
    base: AFDBase,
    date_time: NaiveDateTime,
    evento: String,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AFD{
    cabecalho: Option<Cabecalho>,
    createupdateempresa: Vec<CreateUpdateEmpresa>,
    marcacaoponto: Vec<MarcacaoPonto>,
    ajusterelogio: Vec<AjusteRelogio>,
    createupdatedeleteempregado: Vec<CreateUpdateDeleteEmpregado>,
    sensivelrep: Vec<SensivelREP>
}

impl Default for AFD{
    fn default() -> Self {
        Self{
            cabecalho: None,
            createupdateempresa: Vec::new(),
            marcacaoponto: Vec::new(),
            ajusterelogio: Vec::new(),
            createupdatedeleteempregado: Vec::new(),
            sensivelrep: Vec::new()
        }
    }
}

impl RegistryType{
    pub fn get_registry_type_by_number(number: i8)->Option<Self>{
        match number{
            1 => Some(Self::Cabecalho),
            2 => Some(Self::CreateUpdateEmpresa),
            3 => Some(Self::MarcacaoPonto),
            4 => Some(Self::AjusteRelogio),
            5 => Some(Self::CreateUpdateDeleteEmpregado),
            6 => Some(Self::SensivelREP),
            7 => Some(Self::MarcacaoPontoRepP),
            9 => Some(Self::Trailer),
            _ => None,
        }
    }
    pub fn parse_afd(&self, appstate: &mut AppState, linha: &str){
        match self{
            Self::Cabecalho => {
                let n_serie = &linha[0..9];
                let tipo = RegistryType::Cabecalho;
                let mut tipo_empregador_txt = String::new();
                if let Some(char_casa_11) = linha.chars().nth(10){
                    let tipo_empregador = char_casa_11.to_digit(10).unwrap() as i8;
                    if tipo_empregador == 1{
                        tipo_empregador_txt = "CNPJ".to_string()
                    }else{
                        tipo_empregador_txt = "CPF".to_string()
                    };
                    // println!("tipo_empregador: {}", tipo_empregador_txt)

                }
                let cnpj_cpf = &linha[11..25];
                // println!("cnpj/cpj: {}", cnpj_cpf);
                let cno_caepf = &linha[25..39];
                // println!("cno: {}", cno_caepf);
                let razao_social = &linha[39..189];
                // println!("razao_social: {}", razao_social.to_string().trim());
                let id_rep = &linha[189..206];
                // println!("id_rep: {}", id_rep);

                let data_inicio = &linha[206..216];
                // println!("data_inicio: {}", data_inicio);

                let data_final = &linha[216..226];
                // println!("data_final: {}", data_final);

                let geracao_arquivo = &linha[226..250];
                // println!("data/hora geracao arquivo: {}", geracao_arquivo);

                let versao_leiaute_afd = &linha[250..253];
                // println!("leiaute_afd: {}", versao_leiaute_afd);

                let tipo_fabricante_rep_char = &linha[253..254];
                let _tipo_fabricante_rep = if tipo_fabricante_rep_char.to_string() == "1"{"CNPJ"}else{"CPF"};
                // println!("Tipo fabricante: {}", tipo_fabricante_rep);

                let cnpj_fabricante_rep = &linha[254..268];
                // println!("CNPJ fabricante rep: {}", cnpj_fabricante_rep.to_string().trim());

                let modelo_rep = &linha[268..298];
                // println!("Modelo REP: {}", modelo_rep.to_string().trim());

                let registro_hexa = &linha[298..302];
                // println!("Registro hexa: {}", registro_hexa);

                let cabecalho = Cabecalho{
                    base: AFDBase { nsr: n_serie.to_string(), tipo: tipo },
                    tipo_empregador: tipo_empregador_txt,
                    cnpj_empregador: cnpj_cpf.to_string(),
                    cno_empregador: cno_caepf.to_string(),
                    razao_social: razao_social.to_string(),
                    id_rep: id_rep.to_string(),
                    data_inicio: NaiveDate::parse_from_str(&data_inicio, "%Y-%m-%d").unwrap(),
                        // SelDate::new_by_str(data_inicio),
                    data_final: NaiveDate::parse_from_str(&data_final, "%Y-%m-%d").unwrap(),
                    geracao_arquivo: geracao_arquivo.to_string(),
                    versao_leiaute_afd: versao_leiaute_afd.to_string(),
                    cnpj_fabricante_rep: cnpj_fabricante_rep.to_string(),
                    modelo_rep: modelo_rep.to_string(),
                    registro_hexa: registro_hexa.to_string(),
                };
                appstate.afd.cabecalho = Some(cabecalho)
            },

            Self::CreateUpdateEmpresa => { //2
                let nsr = &linha[0..9];
                let registro = RegistryType::CreateUpdateEmpresa;
                let date_time = &linha[10..29];
                let cpf_admin = &linha[34..48];
                let tipo_empregador_txt = &linha[48..49];
                let tipo_empregador_int: i8 = tipo_empregador_txt.parse::<i8>().unwrap();
                let tipo_empregador = match tipo_empregador_int{
                    1 => "CNPJ",
                    2 => "CPF",
                    _ => "NAO LISTADO",

                };
                let cnpj_empregador = &linha[49..63];
                let cno = &linha[63..77];
                let razao_social = &linha[77..227];
                let local_servico = &linha[227..327];
                let registro_hexa = &linha[327..332];
                // println!("nsr: {}", nsr);
                // println!("registro: {}", registro);
                // println!("date time: {}", date_time);
                // println!("cpf_admin: {}", cpf_admin);
                // println!("tipo empregador: {}", tipo_empregador);
                // println!("cnpj empregador: {}", cnpj_empregador.trim());
                // println!("cno: {}", cno);
                // println!("razao_social: {}", razao_social);
                // println!("local_servico: {}", local_servico);
                // println!("registro_hexa: {}", registro_hexa);
                let createupdateempresa = CreateUpdateEmpresa{
                    base: AFDBase { nsr: nsr.to_string(), tipo: registro },
                    date_time: NaiveDateTime::parse_from_str(&date_time, "%Y-%m-%dT%H:%M:%S").unwrap(),
                    cpf_admin: cpf_admin.to_string(),
                    tipo_empregador: tipo_empregador.to_string(),
                    cnpj_empregador: cnpj_empregador.to_string(),
                    cno: cno.to_string(),
                    razao_social: razao_social.to_string(),
                    local_servico: local_servico.to_string(),
                    registro_hexa: registro_hexa.to_string(),
                };
                appstate.afd.createupdateempresa.push(createupdateempresa)
            },
            
            Self::MarcacaoPonto => { //3
                let nsr = &linha[0..9];
                let registro = RegistryType::MarcacaoPonto;
                let date_time = &linha[10..29];
                let cpf_empregado = &linha[35..46];
                let registro_hexa = &linha[46..50];
                // println!("nsr: {}", nsr);
                // println!("registro: {}", registro);
                // println!("data/hora: {}", date_time);
                // println!("cpf empregado: {}", cpf_empregado);
                // println!("registro hexa: {}", registro_hexa);
                let marcacaoponto = MarcacaoPonto{
                    base: AFDBase { nsr: nsr.to_string(), tipo: registro },
                    date_time: NaiveDateTime::parse_from_str(&date_time, "%Y-%m-%dT%H:%M:%S").unwrap(),
                    cpf_empregado: cpf_empregado.to_string(),
                    registro_hexa: registro_hexa.to_string(),
                };
                appstate.afd.marcacaoponto.push(marcacaoponto)
            },

            Self::AjusteRelogio => { //4
                let nsr = &linha[0..9];
                let registro = RegistryType::AjusteRelogio;
                let date_time_antes_registro = &linha[10..29];
                let date_time_ajustado = &linha[34..53];
                let cpf_adm = &linha[58..69];
                let registro_hexa = &linha[69..73];
                // println!("nsr: {}", nsr);
                // println!("registro: {}", registro);
                // println!("date_time_antes_registro: {}", date_time_antes_registro);
                // println!("date time ajustado: {}", date_time_ajustado);
                // println!("cpf adm: {}", cpf_adm);
                // println!("registro_hexa: {}", registro_hexa);
                let ajuste_relogio = AjusteRelogio{
                    base: AFDBase { nsr: nsr.to_string(), tipo: registro },
                    date_time_antes_registro: NaiveDateTime::parse_from_str(&date_time_antes_registro, "%Y-%m-%dT%H:%M:%S").unwrap(),
                    date_time_ajustado: NaiveDateTime::parse_from_str(&date_time_ajustado, "%Y-%m-%dT%H:%M:%S").unwrap(),
                    cpf_adm: cpf_adm.to_string(),
                    registro_hexa: registro_hexa.to_string()
                };
                appstate.afd.ajusterelogio.push(ajuste_relogio)
            },
            Self::CreateUpdateDeleteEmpregado => { //5
                let nsr = &linha[0..9];
                let registro = RegistryType::CreateUpdateDeleteEmpregado;
                let date_time = &linha[10..29];
                let tipo_operacao_str = &linha[34..35];
                let tipo_operacao = match tipo_operacao_str{
                    "I" => "Inclusao",
                    "A" => "Alteracao",
                    "E" => "Exclusao",
                    _ => "Evento nao listado"
                };
                let cpf_empregado = &linha[36..47];
                let nome_empregado = &linha[47..99];
                let mais_dados_empregado = &linha[100..104];
                let cpf_adm = &linha[104..115];
                let registro_hexa = &linha[115..118];
                // println!("nsr: {}", nsr);
                // println!("registro: {}", registro);
                // println!("data/hora: {}", date_time);
                // println!("tipo operacao: {}", tipo_operacao);
                // println!("cpf empregado: {}", cpf_empregado);
                // println!("nome empregado: {}", nome_empregado.trim());
                // println!("mais_dados_empregado: {}", mais_dados_empregado);
                // println!("cpf admin: {}", cpf_adm);
                // println!("registro hexa: {}", registro_hexa);
                let createupdatedeleteempregado = CreateUpdateDeleteEmpregado{
                    base: AFDBase { nsr: nsr.to_string(), tipo: registro },
                    date_time: NaiveDateTime::parse_from_str(&date_time, "%Y-%m-%dT%H:%M:%S").unwrap(),
                    tipo_operacao: tipo_operacao.to_string(),
                    cpf_empregado: cpf_empregado.to_string(),
                    nome_empregado: nome_empregado.to_string(),
                    mais_dados_empregado: mais_dados_empregado.to_string(),
                    cpf_adm: cpf_adm.to_string(),
                    registro_hexa: registro_hexa.to_string(),
                };
                appstate.afd.createupdatedeleteempregado.push(createupdatedeleteempregado)

            },

            Self::SensivelREP=> { //6
                let nsr = &linha[0..9];
                let registro = RegistryType::SensivelREP;
                let date_time = &linha[10..29];
                let tipo_evento_str = &linha[34..36];
                let tipo_evento_int: i8 = tipo_evento_str.parse::<i8>().unwrap();
                let evento = match tipo_evento_int{
                    1 => "Abertura/Violacao REP",
                    2 => "Retorno Energia",
                    3 => "Introducao de Pendrive",
                    4 => "Retirada de Pendrive",
                    5 => "Emissao da Relacao de Marcacoes",
                    6 => "Erro de impressão",
                    7 => "Disponibilidade de Servico",
                    8 => "Indisponibilidade de Servico",
                    _ => "Evento nao listado"
                };
                // println!("nsr: {}", nsr);
                // println!("registro: {}", registro);
                // println!("data/hora: {}", date_time);
                // println!("tipo evento: {} -> {}", tipo_evento_str, evento);

                let sensivelrep = SensivelREP{
                    base: AFDBase { nsr: nsr.to_string(), tipo: registro },
                    date_time: NaiveDateTime::parse_from_str(&date_time, "%Y-%m-%dT%H:%M:%S").unwrap(),

                    evento: evento.to_string()
                };
                appstate.afd.sensivelrep.push(sensivelrep)
            },
            
            Self::MarcacaoPontoRepP => println!("TODO"),//7
            Self::Trailer => println!("TODO"),//9
        }

    }
}
