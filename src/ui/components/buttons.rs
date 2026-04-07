use crate::ui::{components::calendar::CalendarMessage, screens::Screen};
use crate::domain::info_add::info_add::ManualPonto;
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub enum Buttons{
    SwitchScreen(Screen),
    GetAFDFile,
    GetInfoAdd,
    TallyData,
    CalendarButton(CalendarMessage),
    CreateManualPonto(ManualPonto),//criando e alterando infoadd do state.
    UpdateManualPonto(NaiveDateTime, NaiveDateTime),
    DeleteManualPonto(NaiveDateTime),
}
