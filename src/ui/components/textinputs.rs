#[derive(Debug, Clone, Default)]
pub struct TextInputs{
    pub dia_alterando_employee_screen: String,
    pub dia_adicionando_employee_screen: String,
    pub filtro_funcionario_employee_screen: String,
    pub more_info_company_day_off_screen: String,
    pub more_info_employee_day_off_screen: String,
    pub bh_legado_employee_screen: String,
}
#[derive(Debug, Clone)]
pub enum TextInputsEnum{
    DiaAlterandoEmployeeScreen,
    DiaAdicionandoEmployeeScreen,
    FiltroFuncionarioEmployeesScreen,
    MoreInfoCompanyDayOffScreen,
    MoreInfoEmployeeDayOffScreen,
    BHLegadoEmployeeScreen,
}
