use super::super::components::calendar::CalendarType;

#[derive(Debug, Clone, Copy)]
pub enum Screen{
    Main,
    AFDEvents,
    Employees,
    DatePicker(CalendarType)
}
impl Default for Screen {
    fn default() -> Self {
        Screen::Main
    }
}
