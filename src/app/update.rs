use super::state::{AppState};
use iced::{Task as Command};
use super::message::{Message};
use crate::ui::screens::Screen;
use crate::ui::components::buttons::Buttons;

pub fn update(state: &mut AppState, message:Message) -> Command<Message>{
    match message{
        Message::ButtonPressed(buttons)=>{
            match buttons{
                Buttons::SwitchScreen(screen) =>{
                    match screen{
                        Screen::Main => {
                            println!("To Main")

                        }
                        Screen::AFDEvents => {
                            println!("To AFDEvents")
                        }
                        Screen::Employees => {
                            println!("Employees")
                        }
                    }
                    state.current_screen = screen
                }
            }
        }
    }
    Command::none()
}
