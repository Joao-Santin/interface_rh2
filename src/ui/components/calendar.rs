use chrono::NaiveDate;
use std::collections::HashMap;
use iced::widget::{button, column, text};
use iced::Element;

use crate::app::message::Message;

pub enum CalendarType{
    DailyEvents,
    StartFilter,
    EndFilter,
}
pub struct Calendar{
    pub selected_date: HashMap<CalendarType, NaiveDate>
}
pub enum CalendarMessage{
    SelectDay(u32),
    PreviousMonth,
    NextMonth,
    PreviousYear,
    NextYear
}
impl Calendar{
    pub fn view(&mut self, calendar_type: CalendarType) -> Element<'_, Message>{
        column![
            text("CALENDAR!")

        ].into()
    }
}
