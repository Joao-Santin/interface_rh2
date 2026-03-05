use iced::Element;
use iced::widget::{button, column, text};

use crate::app::state::AppState;
use crate::app::message::Message;
use crate::ui::Screen;

pub fn view(_state: &AppState) -> Element<'_, Message> {
    column![
        text("Main Screen"),
        button("To Teste").on_press(Message::SwitchScreen(Screen::Teste))
    ].into()
}
