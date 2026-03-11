use super::state::{AppState};
use iced::{Task as Command};
use super::message::{Message};
use crate::ui::screens::Screen;
use crate::ui::components::buttons::Buttons;
use rfd::FileDialog;

pub fn update(state: &mut AppState, message:Message) -> Command<Message>{
    match message{
        Message::ButtonPressed(buttons)=>{
            match buttons{
                Buttons::SwitchScreen(screen) =>{
                    match screen{
                        Screen::Main => {
                            println!("To Main");
                            state.current_screen = screen

                        }
                        Screen::AFDEvents => {
                            println!("To AFDEvents");
                            state.current_screen = screen
                        }
                        Screen::Employees => {
                            println!("Employees");
                            state.current_screen = screen
                        }
                        Screen::DatePicker(calendar_type)=>{
                            println!("DatePicker");
                            state.current_screen = Screen::DatePicker(calendar_type)
                        }
                    }
                }
                Buttons::GetAFDFile => {
                    if let Some(path) = FileDialog::new()
                        .add_filter("Arquivos de texto", &["txt"])
                        .set_title("SELECIONE O ARQUIVO DE PONTO!")
                        .pick_file()
                    {
                        state.load_afd(path);
                    } else {
                        println!("Nenhum arquivo selecionado.");
                    }
                }
            }
        }
    }
    Command::none()
}
