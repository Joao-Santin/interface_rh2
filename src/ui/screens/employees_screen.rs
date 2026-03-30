use iced::Element;
use iced::widget::{row, button, column, text};
use iced::Alignment::Center;
use iced::Length::{Fixed};

use crate::ui::components::buttons::Buttons;
use crate::ui::Screen;
use crate::app::state::AppState;
use crate::app::message::Message;

pub fn view(_state: &AppState) -> Element<'_, Message> {
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
        button("To Main").on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::Main))),
        cabecalho
    ].into()
}
