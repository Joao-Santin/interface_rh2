use iced::{Element};
use iced::widget::{button, column, pick_list, row, text, text_input, Column, checkbox, container};
use iced::Length::{Fill, Fixed};
use iced::Alignment::Center;

use crate::ui::components::textinputs::{TextInputsEnum};
use crate::ui::components::buttons::Buttons;
use crate::ui::Screen;
use crate::ui::components::calendar::CalendarType;
use crate::ui::components::checkboxes::CheckBoxes;
use crate::app::state::AppState;
use crate::app::message::Message;
use crate::domain::info_add::info_add::{EmployeeDayOff, DayOffType};

pub fn view(state: &AppState, cpf: String) -> Element<'_, Message> {
    let start_search_period = state.sel_dates.selected_date.get(&CalendarType::StartDayOffEmployee).unwrap();
    let end_search_period = state.sel_dates.selected_date.get(&CalendarType::EndDayOffEmployee).unwrap();
    let dayoffs: Vec<&EmployeeDayOff> = state.info_add.employee_day_off
        .iter()
        .filter(|edo|{
            edo.cpf == cpf
        })
        .filter(|edo| {
            edo.start <= *end_search_period && edo.end >= *start_search_period
        }).collect();
    let element_dayoffs: Vec<Element<Message>> = dayoffs
        .iter()
        .cloned()
        .map(|edo| {
            row![
                text(format!("{} | {} à {} | OBS: {} | UsaBanco: {}", edo.typ.to_string(), edo.start.to_string(), edo.end.to_string(), edo.more_info.to_string(), edo.uses_time_off_balance.to_string())),
                button("DELETAR").on_press(Message::ButtonPressed(Buttons::DeleteEmployeeDayOff(edo.clone())))
            ].into()
        })
        .collect();
    let column_dayoffs = Column::with_children(element_dayoffs);
    let motivos = [
        DayOffType::SickLeave,
        DayOffType::Vacation,
        DayOffType::ProgrammedLeave,
        DayOffType::MedicalLeave,
    ];
    container(
        column![
            text("FALTAS E FOLGAS FUNCIONARIO")
                        .width(Fill)
                        .size(60)
                        .color([0.5, 0.5, 0.5])
                        .align_x(Center),
            row![
                button("VOLTAR").on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::Employee(cpf.clone()))))
            ],
            container(
                column![
                    row![
                        text("CRIAR").width(Fill).size(30).color([0.5, 0.5, 0.5]).align_x(Center),
                    ],
                    row![
                        pick_list(
                            motivos,
                            state.picked_dayoff_employee_creating,
                            Message::DayOffEmployeeTypePicked
                        ).placeholder("Qual Motivo da Falta?")
                    ],
                    row![
                        text("Inicio:"),
                        button(text(format!("{:?}", state.sel_dates.selected_date.get(&CalendarType::StartDayOffEmployeeCreating).map(|d| d.to_string()).unwrap_or("Sem data".to_string())))).on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::DatePicker(CalendarType::StartDayOffEmployeeCreating, Some(cpf.clone()))))),
                        text("Fim:"),
                        button(text(format!("{:?}", state.sel_dates.selected_date.get(&CalendarType::EndDayOffEmployeeCreating).map(|d| d.to_string()).unwrap_or("Sem data".to_string())))).on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::DatePicker(CalendarType::EndDayOffEmployeeCreating, Some(cpf.clone()))))),
                    ].spacing(5.0),
                    row![
                        text_input("O QUE ACONTECEU?", &state.text_inputs.more_info_employee_day_off_screen).width(Fixed(300.0)).on_input(|v| Message::TextInputChanged(TextInputsEnum::MoreInfoEmployeeDayOffScreen, v))
                    ],
                    row![
                        checkbox(state.uses_time_off_balance_dayoff_employee_creating)
                            .label("UTILIZAR BANCO DE HORAS")
                            .on_toggle(|is_checked| Message::CheckBoxToggled(CheckBoxes::UsesTimeOffBalance, is_checked))
                    ],
                    row![
                        if let (Some (typ), Some(start), Some(end)) = (
                    state.picked_dayoff_employee_creating.clone(),
                    state.sel_dates.selected_date.get(&CalendarType::StartDayOffEmployeeCreating).copied(),
                    state.sel_dates.selected_date.get(&CalendarType::EndDayOffEmployeeCreating).copied(),
                ){
                            button("CRIAR!").on_press(Message::ButtonPressed(Buttons::CreateEmployeeDayOff(EmployeeDayOff{
                                cpf: cpf.clone(),
                                typ,
                                start,
                                end,
                                more_info: state.text_inputs.more_info_employee_day_off_screen.clone(),
                                uses_time_off_balance: state.uses_time_off_balance_dayoff_employee_creating,
                            })))
                        }else{
                            button("PREENCHA O QUE FALTA")
                        }
                    ],
                ].width(Fill).align_x(Center).spacing(10.0)
            ).width(Fill).center_x(Fill),
            column![
                text("BUSCAR").width(Fill).size(30).color([0.5, 0.5, 0.5]).align_x(Center),
                row![
                    text("do dia"),
                    button(text(format!("{:?}", state.sel_dates.selected_date.get(&CalendarType::StartDayOffEmployee).map(|d| d.to_string()).unwrap_or("Sem data".to_string())))).on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::DatePicker(CalendarType::StartDayOffEmployee, Some(cpf.clone()))))),
                    text("ao dia"),
                    button(text(format!("{:?}", state.sel_dates.selected_date.get(&CalendarType::EndDayOffEmployee).map(|d| d.to_string()).unwrap_or("Sem data".to_string())))).on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::DatePicker(CalendarType::EndDayOffEmployee, Some(cpf.clone())))))
                ].spacing(10),
            ].width(Fill).align_x(Center).spacing(10),
            column_dayoffs,
        ].width(Fill).spacing(30.0).align_x(Center)
    ).width(Fill).height(Fill).center_x(Fill).into()
}
