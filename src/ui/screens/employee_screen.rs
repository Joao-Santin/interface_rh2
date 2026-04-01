
use iced::Element;
use iced::widget::{row, button, column, text, text_input};

use crate::ui::components::buttons::Buttons;
use crate::ui::components::textinputs::TextInputsEnum;
use crate::ui::Screen;
use crate::app::state::AppState;
use crate::app::message::Message;

pub fn view(state: &AppState, cpf: String) -> Element<'_, Message> {
    column![
        text("Employee Screen"),
        text(format!("CPF:{} NOME:{}",
            &cpf,
            state
                .employees
                .get(&cpf)
                .map(|s| s.as_str())
                .unwrap_or("Nao encontrado")
        )),
        row![
            text("Adicionar dia."),
            text_input("dia", &state.text_inputs.dia_adicionando_employee_screen).on_input(|v| Message::TextInputChanged(TextInputsEnum::DiaAdicionandoEmployeeScreen, v)),
        ],
        button("To Employees").on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::Employees))),
    ].into()
}
