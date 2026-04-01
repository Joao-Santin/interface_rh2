use crate::ui::{components::calendar::CalendarMessage, screens::Screen};

#[derive(Debug, Clone)]
pub enum Buttons{
    SwitchScreen(Screen),
    GetAFDFile,
    GetInfoAdd,
    TallyData,
    CalendarButton(CalendarMessage)
}
