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
impl RegistryType{
    fn get_registry_type_by_number(number: i8)->Option<Self>{
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
}

pub struct AFDBase{
    nsr: String,
    tipo: RegistryType
}
pub struct AFD{
}

impl Default for AFD{
    fn default() -> Self {
        Self{
        }
    }
}
