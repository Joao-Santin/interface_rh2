use iced::Element;
use crate::ui::screens::{main_screen, afdevents_screen, employees_screen, datepicker_screen, employee_screen, company_day_off_screen, employee_day_off_screen};
use crate::Message;
use crate::AppState;
use screens::screen::Screen;
pub mod screens;
pub mod components;

pub fn view(state: &AppState) -> Element<'_, Message>{
    match &state.current_screen{
        Screen::Main => {
            main_screen::view(state)
        }
        Screen::AFDEvents => {
            afdevents_screen::view(state)
        }
        Screen::Employees => {
            employees_screen::view(state)
        }
        Screen::Employee(cpf) =>{
            employee_screen::view(state, cpf.clone())
        }
        Screen::DatePicker(calendar_type, cpf) => {
            datepicker_screen::view(state, *calendar_type, cpf.clone())
        }
        Screen::CompanyDayOff => {
            company_day_off_screen::view(state)
        }
        Screen::EmployeeDayOff(cpf) => {
            employee_day_off_screen::view(state, cpf.clone())
        }

    }
}

