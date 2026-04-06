#[derive(Debug, Clone, Default)]
pub struct TextInputs{
    pub dia_alterando_employee_screen: String,
    pub dia_adicionando_employee_screen: String,
    pub filtro_funcionario_employee_screen: String,
}
#[derive(Debug, Clone)]
pub enum TextInputsEnum{
    DiaAlterandoEmployeeScreen,
    DiaAdicionandoEmployeeScreen,
    FiltroFuncionarioEmployeesScreen,
}
