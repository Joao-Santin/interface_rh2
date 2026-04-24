use super::super::components::calendar::CalendarType;

#[derive(Debug, Clone)]
pub enum Screen{
    Main,
    AFDEvents,
    Employees,
    Employee(String),
    DatePicker(CalendarType, Option<String>),
    CompanyDayOff,
    EmployeeDayOff(String)
}
impl Default for Screen {
    fn default() -> Self {
        Screen::Main
    }
}
