use iced::{Task as Command, Element, Theme};

mod ui;
mod app;
mod domain;
mod extensions;

use app::state::AppState;
use app::message::Message;
fn boot() -> (AppState, Command<Message>){
    (AppState::default(), Command::none())
}
fn update(state: &mut AppState, message: Message)-> Command<Message> {
    app::update(state, message)
}
fn view(state: &AppState) -> Element<'_, Message> {
    ui::view(state)
}

fn main()-> iced::Result {
    iced::application(boot, update, view)
        .title("ERP")
        .theme(|_: &AppState| Theme::Dark)
        .run()
}
