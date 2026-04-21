use iced::{Element};
use iced::widget::{button, column, row, text};

use crate::ui::components::calendar::CalendarType;
use crate::app::state::AppState;
use crate::app::message::Message;
use crate::ui::components::buttons::Buttons;
use crate::ui::Screen;

pub fn view(state: &AppState) -> Element<'_, Message> {
    column![
        row![
            text("Folgas e Férias Coletivas"),
        ],
        row![
            text("Do dia "),
            button(text(format!("{:?}", state.sel_dates.selected_date.get(&CalendarType::StartDayOffCompany).map(|d| d.to_string()).unwrap_or("Sem data".to_string())))).on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::DatePicker(CalendarType::StartDayOffCompany, None)))),
            text("ao dia "),
            button(text(format!("{:?}", state.sel_dates.selected_date.get(&CalendarType::EndDayOffCompany).map(|d| d.to_string()).unwrap_or("Sem data".to_string())))).on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::DatePicker(CalendarType::EndDayOffCompany, None)))),
        ],
        row![
            button("voltar").on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::Main)))
        ]
    ].into()
}
