use chrono::{NaiveDate, Local};
use std::collections::HashMap;
use iced::widget::{button, column, text, Column};
use iced::Alignment::Center;
use iced::Element;
use std::fmt;

use crate::extensions::chrono_ext::NaiveDateExt;
use crate::app::message::Message;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum CalendarType{
    DailyEvents,
    StartFilter,
    EndFilter,
}
impl fmt::Display for CalendarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            CalendarType::DailyEvents => "DailyEvents",
            CalendarType::StartFilter => "StartFilter",
            CalendarType::EndFilter => "EndFilter"
        };
        write!(f, "{}", text)
    }
}
pub struct Calendar{
    pub selected_date: HashMap<CalendarType, NaiveDate>
}
impl Default for Calendar{
    fn default() -> Self {
        let mut selected_date = HashMap::new();
        let time_now = Local::now().date_naive();
        selected_date.insert(CalendarType::DailyEvents, time_now);
        selected_date.insert(CalendarType::StartFilter, time_now);
        selected_date.insert(CalendarType::EndFilter, time_now);

        Self{
            selected_date
        }
    }
}
pub enum CalendarMessage{
    SelectDay(u32),
    PreviousMonth,
    NextMonth,
    PreviousYear,
    NextYear
}
impl Calendar{
    pub fn view(&self, calendar_type: CalendarType) -> Element<'_, Message>{

        let mut sun: Column<Message> = column![text("Dom")].spacing(5).align_x(Center);
        let mut mon: Column<Message> = column![text("Seg")].spacing(5).align_x(Center);
        let mut tue: Column<Message> = column![text("Ter")].spacing(5).align_x(Center);
        let mut wed: Column<Message> = column![text("Qua")].spacing(5).align_x(Center);
        let mut thu: Column<Message> = column![text("Qui")].spacing(5).align_x(Center);
        let mut fri: Column<Message> = column![text("Sex")].spacing(5).align_x(Center);
        let mut sat: Column<Message> = column![text("Sab")].spacing(5).align_x(Center);
        let mut days: Vec<(chrono::Weekday, u32)> = Vec::new();
        if let Some(sel_date) = self.selected_date.get(&calendar_type){
            days = sel_date.get_month_day_weekday();
        }
        println!("days:{:?}", days);
        //voltar aqui!!!!!
        match calendar_type{
            CalendarType::DailyEvents =>{
                println!("DailyEvents");
            },
            CalendarType::StartFilter =>println!("StartFilter"),
            CalendarType::EndFilter =>println!("EndFilter"),
        }
        column![
            text("CALENDARIO"),
            text(format!("{}", calendar_type.to_string()))
        ].into()
    }
}
