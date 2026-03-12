use chrono::{NaiveDate, Local, Weekday};
use std::collections::HashMap;
use iced::widget::{button, column, row, text, Column};
use iced::Alignment::Center;
use iced::Length::{Fixed};
use iced::Element;
use std::fmt;

use crate::extensions::chrono_ext::NaiveDateExt;
use crate::ui::components::buttons::Buttons;
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
#[derive(Debug, Clone)]
pub enum CalendarMessage{
    SelectDay(CalendarType, u32),
    PreviousMonth(CalendarType),
    NextMonth(CalendarType),
    PreviousYear(CalendarType),
    NextYear(CalendarType)
}
impl Calendar{
    pub fn view(&self, calendar_type: CalendarType) -> Element<'_, Message>{

        let mut sun: Column<Message> = column![text("Dom")].spacing(5).align_x(Center);//sun
        let mut mon: Column<Message> = column![text("Seg")].spacing(5).align_x(Center);//mon
        let mut tue: Column<Message> = column![text("Ter")].spacing(5).align_x(Center);//tue
        let mut wed: Column<Message> = column![text("Qua")].spacing(5).align_x(Center);//wed
        let mut thu: Column<Message> = column![text("Qui")].spacing(5).align_x(Center);//thu
        let mut fri: Column<Message> = column![text("Sex")].spacing(5).align_x(Center);//fri
        let mut sat: Column<Message> = column![text("Sab")].spacing(5).align_x(Center);//sat
        let mut days: Vec<(chrono::Weekday, u32)> = Vec::new();
        if let Some(sel_date) = self.selected_date.get(&calendar_type){
            days = sel_date.get_month_day_weekday();
        }
        let first_weekday = days.first().map(|(weekday, _)| *weekday);
        match first_weekday{
                Some(Weekday::Sun) => (),
                Some(Weekday::Mon) => {
                    sun = sun.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, 0)))));
                },
                Some(Weekday::Tue) => {
                    sun = sun.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, 0)))));
                    mon = mon.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, 0)))))
                },
                Some(Weekday::Wed) => {
                    sun = sun.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, 0)))));
                    mon = mon.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, 0)))));
                    tue = tue.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, 0)))));
                },
                Some(Weekday::Thu) => {
                    sun = sun.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, 0)))));
                    mon = mon.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, 0)))));
                    tue = tue.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, 0)))));
                    wed = wed.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, 0)))));
                },
                Some(Weekday::Fri) => {
                    sun = sun.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, 0)))));
                    mon = mon.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, 0)))));
                    tue = tue.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, 0)))));
                    wed = wed.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, 0)))));
                    thu = thu.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, 0)))));
                },
                Some(Weekday::Sat) => {
                    sun = sun.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, 0)))));
                    mon = mon.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, 0)))));
                    tue = tue.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, 0)))));
                    wed = wed.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, 0)))));
                    thu = thu.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, 0)))));
                    fri = fri.push(button(text("")).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, 0)))));
                },
                None => println!("Não funcionou")
                }
        for (weekday, day) in days{
            //nao vai precisar desse match, vou mijar, escrevendo caso eu esqueça zkkkkkkkkkkkk
            match calendar_type{
                CalendarType::DailyEvents =>{
                    println!("DailyEvents");
                },
                CalendarType::StartFilter =>println!("StartFilter"),
                CalendarType::EndFilter =>println!("EndFilter"),
            }
        }
        column![
            text("CALENDARIO"),
            text(format!("{}", calendar_type.to_string()))
        ].into()
    }
}
