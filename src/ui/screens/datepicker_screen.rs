use iced::Element;
use iced::Alignment::Center;
use iced::widget::{text, column};

use crate::app::state::AppState;
use crate::app::message::Message;
use crate::ui::components::calendar::{CalendarType};

pub fn view(state: &AppState, calendar_type: CalendarType) -> Element<'_, Message>{
    column![
        text("DatePicker"),
        state.sel_dates.view(calendar_type),
    ].into()
}
