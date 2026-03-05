use iced::Element;
use crate::ui::screens::{main_screen, teste_screen};
use crate::Message;
use crate::AppState;
use screens::screen::Screen;
pub mod screens;

pub fn view(state: &AppState) -> Element<'_, Message>{
    match state.current_screen{
        Screen::Main => {
            main_screen::view(state)
        }
        Screen::Teste => {
            teste_screen::view(state)
        }
    }
}

