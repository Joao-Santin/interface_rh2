use iced::Element;
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
        button("BUSCAR DADOS")
            .on_press(Message::ButtonPressed(Buttons::GetAFDFile)),
        button("ACONTECIMENTOS")
            .on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::AFDEvents))),
        button("FUNCIONARIOS")
            .on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::Employees))),
        button("TESTE CALENDARIO")
            .on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::DatePicker(CalendarType::DailyEvents))))

    ].width(Fill).height(Fill).align_x(Center).into()
}

