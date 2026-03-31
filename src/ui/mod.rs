use iced::Element;
use crate::ui::screens::{main_screen, afdevents_screen, employees_screen, datepicker_screen, employee_screen};
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
        Screen::DatePicker(calendar_type) => {
            datepicker_screen::view(state, *calendar_type)

        }
    }
}

