use iced::widget::{column, text, row, button, text_input};
use iced::Length::{Fixed, Fill};
use iced::Element;
use iced::Alignment::Center;

use crate::app::message::Message;
use crate::ui::components::textinputs::{TextInputsEnum};
use crate::app::state::AppState;
use crate::ui::components::calendar::CalendarType;
use crate::ui::components::buttons::Buttons;
use crate::ui::Screen;
use crate::domain::info_add::info_add::{EmployeeDayOff, DayOffType};

pub fn view(state: &AppState, cpf: String) -> Element<'_, Message>{
    column![
        text("ATESTADO MEDICO P/ FILHO"),
        button("VOLTAR").on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::EmployeeDayOff(cpf.clone())))),
        row![
            text("Inicio:"),
            button(text(format!("{:?}", state.sel_dates.selected_date.get(&CalendarType::StartDayOffEmployeeCreating).map(|d| d.to_string()).unwrap_or("Sem data".to_string())))).on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::DatePicker(CalendarType::StartDayOffEmployeeCreating, Some(cpf.clone()))))),
            text("Fim:"),
            button(text(format!("{:?}", state.sel_dates.selected_date.get(&CalendarType::EndDayOffEmployeeCreating).map(|d| d.to_string()).unwrap_or("Sem data".to_string())))).on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::DatePicker(CalendarType::EndDayOffEmployeeCreating, Some(cpf.clone()))))),
        ].spacing(5.0),
        row![
            text_input("O QUE ACONTECEU?", &state.text_inputs.more_info_employee_day_off_screen).width(Fixed(300.0)).on_input(|v| Message::TextInputChanged(TextInputsEnum::MoreInfoEmployeeDayOffScreen, v))
        ],
        if let (Some(start), Some(end)) = (
        state.sel_dates.selected_date.get(&CalendarType::StartDayOffEmployeeCreating).copied(),
        state.sel_dates.selected_date.get(&CalendarType::EndDayOffEmployeeCreating).copied()
    ){
        button("CRIAR").on_press(Message::ButtonPressed(Buttons::CreateEmployeeDayOff(EmployeeDayOff{
            cpf: cpf.clone(),
            typ: DayOffType::SonsSickLeave,
            start,
            end,
            more_info: state.text_inputs.more_info_employee_day_off_screen.clone(),
            uses_time_off_balance: false
        })))
            }else{
            button("PREENCHA O QUE FALTA")
        }
    ].width(Fill).height(Fill).align_x(Center).into()

}
