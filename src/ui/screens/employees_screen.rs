use iced::Element;
use iced::widget::{row, button, column, text, Column, scrollable, text_input};
use iced::Alignment::Center;
use iced::Length::{Fixed};

use crate::ui::components::buttons::Buttons;
use crate::ui::Screen;
use crate::app::state::AppState;
use crate::app::message::Message;
use crate::ui::components::textinputs::{TextInputsEnum};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let funcionarios: Vec<Element<Message>> = state.employees
        .iter()
        .filter(|(k, _)| {
            let filtro = state.text_inputs.filtro_funcionario_employee_screen.trim();
            if state.text_inputs.filtro_funcionario_employee_screen.trim() == ""{
                true
            }else{
                state.employees.get(k.as_str()).map_or(false, |name| name.contains(&filtro.to_uppercase()))
            }
        })
        .map(|(k, v)|{
        row![
            column![
                text(v)
            ].width(Fixed(150.0)).align_x(Center),
            column![
                text(k)
            ].width(Fixed(150.0)).align_x(Center),
            column![
                text("*")
            ].width(Fixed(150.0)).align_x(Center),
            column![
                button("config").on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::Employee(k.clone()))))
            ].width(Fixed(150.0)).align_x(Center),
        ].spacing(15).into()
    }).collect();
    let coluna_funcionarios = Column::with_children(funcionarios);
    let cabecalho = row![
        column![
            text("NOME").size(20),
        ].width(Fixed(150.0)).align_x(Center),
        column![
            text("CPF").size(20),
        ].width(Fixed(150.0)).align_x(Center),
        column![
            text("BANCO HORAS").size(20),
        ].width(Fixed(150.0)).align_x(Center),
        column![
            text("CONFIG").size(20),
        ].width(Fixed(150.0)).align_x(Center),
    ].spacing(15);
    column![
        text("Employees Screen"),
        text_input("Procurando quem?", &state.text_inputs.filtro_funcionario_employee_screen).on_input(|v| Message::TextInputChanged(TextInputsEnum::FiltroFuncionarioEmployeesScreen, v)),
        button("To Main").on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::Main))),
        cabecalho,
        scrollable(coluna_funcionarios)
    ].into()
}
