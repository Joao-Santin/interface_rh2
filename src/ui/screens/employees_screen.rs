use iced::Element;
use iced::widget::{row, button, column, text, Column, scrollable, text_input};
use iced::Alignment::Center;
use iced::Length::{Fixed, Fill};

use crate::ui::components::buttons::Buttons;
use crate::ui::Screen;
use crate::app::state::AppState;
use crate::app::message::Message;
use crate::ui::components::textinputs::{TextInputsEnum};
use crate::domain::day_result::day_result::{total_balance_by_cpf, total_balance_by_cpf_with_dates};
use crate::ui::components::calendar::CalendarType;

pub fn view(state: &AppState) -> Element<'_, Message> {
    let mut employees: Vec<(&String, &String)> = state.employees.iter().collect();
    employees.sort_by(|a, b| a.1.to_lowercase().cmp(&b.1.to_lowercase()));
    let funcionarios: Vec<Element<Message>> = employees
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
                text(*v)
            ].width(Fixed(150.0)).align_x(Center),
            column![
                text(*k)
            ].width(Fixed(150.0)).align_x(Center),
            column![
                text(format!("{}H{}M", ((total_balance_by_cpf_with_dates(*state.sel_dates.selected_date.get(&CalendarType::StartFilterEmployees).unwrap(), *state.sel_dates.selected_date.get(&CalendarType::EndFilterEmployees).unwrap(),&state.day_result, &k) + state.info_add.bh_legado.get(*k).map(|m| chrono::Duration::minutes(*m as i64)).unwrap_or(chrono::Duration::zero())).num_minutes() as i32 / 60).to_string(), ((total_balance_by_cpf_with_dates(*state.sel_dates.selected_date.get(&CalendarType::StartFilterEmployees).unwrap(), *state.sel_dates.selected_date.get(&CalendarType::EndFilterEmployees).unwrap(),&state.day_result, &k) + state.info_add.bh_legado.get(*k).map(|m| chrono::Duration::minutes(*m as i64)).unwrap_or(chrono::Duration::zero())).num_minutes() as u32 % 60).to_string())),
            ].width(Fixed(150.0)).align_x(Center),
            column![
                text(format!("{}", (total_balance_by_cpf(&state.day_result, &k).num_minutes() as f32 / 60.0).to_string())),
            ].width(Fixed(150.0)).align_x(Center),
            column![
                button("config").on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::Employee(k.to_string()))))
            ].width(Fixed(150.0)).align_x(Center),
        ].spacing(15).into()
    }).collect();
    let coluna_funcionarios = Column::with_children(funcionarios);
    let cabecalho = row![
        column![
            text("NOME").size(20).color([0.5, 0.5, 0.5]),
        ].width(Fixed(150.0)).align_x(Center),
        column![
            text("CPF").size(20).color([0.5, 0.5, 0.5]),
        ].width(Fixed(150.0)).align_x(Center),
        column![
            text("BH PERIODO").size(20).color([0.5, 0.5, 0.5]),
        ].width(Fixed(150.0)).align_x(Center),
        column![
            text("BENEFICIADO").size(20).color([0.5, 0.5, 0.5]),
        ].width(Fixed(150.0)).align_x(Center),
        column![
            text("CONFIG").size(20).color([0.5, 0.5, 0.5]),
        ].width(Fixed(150.0)).align_x(Center),
    ].spacing(15);
    column![
        text("FUNCIONARIOS")
            .width(Fill)
            .size(60)
            .color([0.5, 0.5, 0.5])
            .align_x(Center),
        button("VOLTAR TELA").on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::Main))),
        row![
            text("Do dia "),
            button(text(format!("{:?}", state.sel_dates.selected_date.get(&CalendarType::StartFilterEmployees).map(|d| d.to_string()).unwrap_or("Sem data".to_string())))).on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::DatePicker(CalendarType::StartFilterEmployees, None)))),
            text(" ao dia "),
            button(text(format!("{:?}", state.sel_dates.selected_date.get(&CalendarType::EndFilterEmployees).map(|d| d.to_string()).unwrap_or("Sem data".to_string())))).on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::DatePicker(CalendarType::EndFilterEmployees, None)))),
        ],
        text_input("Procurando quem?", &state.text_inputs.filtro_funcionario_employee_screen).on_input(|v| Message::TextInputChanged(TextInputsEnum::FiltroFuncionarioEmployeesScreen, v)).width(Fixed(300.0)),
        cabecalho,
        scrollable(coluna_funcionarios.spacing(5.0))
    ].align_x(Center).spacing(10.0).into()
}
