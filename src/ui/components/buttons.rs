use crate::ui::{components::calendar::CalendarMessage, screens::Screen};
use crate::domain::info_add::info_add::{CompanyDayOff, ManualPonto};
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub enum Buttons{
    SwitchScreen(Screen),
    GetAFDFile,
    GetInfoAdd,
    SaveInfoAdd,
    TallyData,
    CalendarButton(CalendarMessage),
    CreateManualPonto(ManualPonto),//criando e alterando infoadd do state.
    EditManualPonto(ManualPonto),
    DeleteManualPonto(ManualPonto),
    CreateCompanyDayOff(CompanyDayOff),
}
