use super::state::{AppState};
use iced::{Task as Command};
use super::message::{Message};
use crate::ui::screens::Screen;

pub fn update(state: &mut AppState, message:Message) -> Command<Message>{
    match message{
        Message::SwitchScreen(screen)=>{
            match screen{
                Screen::Main =>{
                    println!("To Main")
                }
                Screen::Teste =>{
                    println!("To Teste")
                }
            }
            state.current_screen = screen
        }
    }
    Command::none()
}
