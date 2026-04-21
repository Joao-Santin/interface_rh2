use iced::{Element, Color};
use iced::Alignment::Center;
use iced::Length::Fill;
use iced::widget::{button, column, text};

use crate::ui::components::calendar::CalendarType;
use crate::app::state::AppState;
use crate::app::message::Message;
use crate::ui::Screen;
use crate::ui::components::buttons::Buttons;

pub fn view(state: &AppState) -> Element<'_, Message> {
    column![
        if let Some(data) = &state.last_afd_got{
            text(data.to_string())
        }else{
            text("PEGAR AFD!")
        },
        if let Some(data_add) = &state.last_add_info_got{
            text(data_add.to_string())
        }else{
            text("PEGAR AFD!")
        },

        button(text("BUSCAR DADOS AFD").color(if state.last_afd_got.is_some(){Color::from_rgb(0.0, 1.0, 0.0)}else{Color::from_rgb(1.0, 0.0, 0.0)}))
            .on_press(Message::ButtonPressed(Buttons::GetAFDFile)),
        button(text("BUSCAR INFO ADD").color(if state.last_add_info_got.is_some(){Color::from_rgb(0.0, 1.0, 0.0)}else{Color::from_rgb(1.0, 0.0, 0.0)}))
            .on_press(Message::ButtonPressed(Buttons::GetInfoAdd)),
        button(text("SALVAR INFO ADD"))
            .on_press(Message::ButtonPressed(Buttons::SaveInfoAdd)),
        // button("ACONTECIMENTOS")
        //     .on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::AFDEvents))),
        button("FUNCIONARIOS")
            .on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::Employees))),
        button("FERIADOS E FERIAS COLETIVAS")
            .on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::CompanyDayOff)))
    ].width(Fill).height(Fill).align_x(Center).into()
}

