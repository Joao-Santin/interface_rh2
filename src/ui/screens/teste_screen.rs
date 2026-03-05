use iced::Element;
use iced::widget::{button, column, text};

use crate::ui::Screen;
use crate::app::state::AppState;
use crate::app::message::Message;

pub fn view(_state: &AppState) -> Element<'_, Message> {
    column![
        text("Teste Screen"),
        button("To Main").on_press(Message::SwitchScreen(Screen::Main))
    ].into()
}
