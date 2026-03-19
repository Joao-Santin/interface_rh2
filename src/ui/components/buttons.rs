use crate::ui::{components::calendar::CalendarMessage, screens::Screen};

#[derive(Debug, Clone)]
pub enum Buttons{
    SwitchScreen(Screen),
    GetAFDFile,
    GetTally,
    CalendarButton(CalendarMessage)
}
