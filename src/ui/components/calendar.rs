use chrono::{NaiveDate, Local, Weekday, Datelike};
use std::collections::HashMap;
use iced::widget::{button, column, row, text, Column};
use iced::Alignment::Center;
use iced::Length::{Fixed, Fill};
use iced::{Color, Element};
use std::fmt;

use crate::extensions::chrono_ext::NaiveDateExt;
use crate::ui::components::buttons::Buttons;
use crate::ui::Screen;
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CalendarMessage{
    SelectDay(CalendarType, u32),
    SelectMonth(CalendarType, u32),
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
            if let Some(date) = self.selected_date.get(&calendar_type){
                let day_button: Element<Message> = if date.day() == day{
                    button(text(format!("{} *", day.to_string())).color(Color::from_rgb(1.0, 0.0, 0.0))).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, day)))).into()
                }else{
                    button(text(format!("{}", day.to_string()))).width(Fixed(60.0)).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectDay(calendar_type, day)))).into()
                };
                match weekday{
                    Weekday::Sun => sun = sun.push(day_button),
                    Weekday::Mon => mon = mon.push(day_button),
                    Weekday::Tue => tue = tue.push(day_button),
                    Weekday::Wed => wed = wed.push(day_button),
                    Weekday::Thu => thu = thu.push(day_button),
                    Weekday::Fri => fri = fri.push(day_button),
                    Weekday::Sat => sat = sat.push(day_button)
                }
            }
        }
        let month_button_width: f32 = 40.0;
        let monthnow = self.selected_date.get(&calendar_type).unwrap().month();
        let yearnow = self.selected_date.get(&calendar_type).unwrap().year();
        column![
            text("CALENDARIO"),
            text(format!("{}", calendar_type.to_string())),
            row![
                button("<-").on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::PreviousYear(calendar_type)))),
                text(format!("{}", yearnow.to_string())),
                button("->").on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::NextYear(calendar_type)))),
            ],
            row![
                button(text(if monthnow == 1{"Jan*"}else{"Jan"}).color(if monthnow ==1{Color::from_rgb(1.0, 0.0, 0.0)}else{Color::from_rgb(1.0,1.0,1.0)})).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectMonth(calendar_type, 1)))).width(Fixed(month_button_width)),
                button(text(if monthnow == 2{"Fev*"}else{"Fev"}).color(if monthnow ==2{Color::from_rgb(1.0, 0.0, 0.0)}else{Color::from_rgb(1.0,1.0,1.0)})).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectMonth(calendar_type, 2)))).width(Fixed(month_button_width)),
                button(text(if monthnow == 3{"Mar*"}else{"Mar"}).color(if monthnow ==3{Color::from_rgb(1.0, 0.0, 0.0)}else{Color::from_rgb(1.0,1.0,1.0)})).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectMonth(calendar_type, 3)))).width(Fixed(month_button_width)),
                button(text(if monthnow == 4{"Abr*"}else{"Abr"}).color(if monthnow ==4{Color::from_rgb(1.0, 0.0, 0.0)}else{Color::from_rgb(1.0,1.0,1.0)})).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectMonth(calendar_type, 4)))).width(Fixed(month_button_width)),
                button(text(if monthnow == 5{"Mai*"}else{"Mai"}).color(if monthnow ==5{Color::from_rgb(1.0, 0.0, 0.0)}else{Color::from_rgb(1.0,1.0,1.0)})).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectMonth(calendar_type, 5)))).width(Fixed(month_button_width)),
                button(text(if monthnow == 6{"Jun*"}else{"Jun"}).color(if monthnow ==6{Color::from_rgb(1.0, 0.0, 0.0)}else{Color::from_rgb(1.0,1.0,1.0)})).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectMonth(calendar_type, 6)))).width(Fixed(month_button_width)),
                button(text(if monthnow == 7{"Jul*"}else{"Jul"}).color(if monthnow ==7{Color::from_rgb(1.0, 0.0, 0.0)}else{Color::from_rgb(1.0,1.0,1.0)})).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectMonth(calendar_type, 7)))).width(Fixed(month_button_width)),
                button(text(if monthnow == 8{"Ago*"}else{"Ago"}).color(if monthnow ==8{Color::from_rgb(1.0, 0.0, 0.0)}else{Color::from_rgb(1.0,1.0,1.0)})).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectMonth(calendar_type, 8)))).width(Fixed(month_button_width)),
                button(text(if monthnow == 9{"Set*"}else{"Set"}).color(if monthnow ==9{Color::from_rgb(1.0, 0.0, 0.0)}else{Color::from_rgb(1.0,1.0,1.0)})).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectMonth(calendar_type, 9)))).width(Fixed(month_button_width)),
                button(text(if monthnow == 10{"Out*"}else{"Out"}).color(if monthnow ==10{Color::from_rgb(1.0, 0.0, 0.0)}else{Color::from_rgb(1.0,1.0,1.0)})).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectMonth(calendar_type, 10)))).width(Fixed(month_button_width)),
                button(text(if monthnow == 11{"Nov*"}else{"Nov"}).color(if monthnow ==11{Color::from_rgb(1.0, 0.0, 0.0)}else{Color::from_rgb(1.0,1.0,1.0)})).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectMonth(calendar_type, 11)))).width(Fixed(month_button_width)),
                button(text(if monthnow == 12{"Dez*"}else{"Dez"}).color(if monthnow ==12{Color::from_rgb(1.0, 0.0, 0.0)}else{Color::from_rgb(1.0,1.0,1.0)})).on_press(Message::ButtonPressed(Buttons::CalendarButton(CalendarMessage::SelectMonth(calendar_type, 12)))).width(Fixed(month_button_width)),
            ],
            row![
                sun,
                mon,
                tue,
                wed,
                thu,
                fri,
                sat
            ].spacing(10),
            button(text("SELECIONAR DIA!")).on_press(match calendar_type {
                CalendarType::DailyEvents => Message::ButtonPressed(Buttons::SwitchScreen(Screen::Main)),
                CalendarType::StartFilter => Message::ButtonPressed(Buttons::SwitchScreen(Screen::Main)),
                CalendarType::EndFilter => Message::ButtonPressed(Buttons::SwitchScreen(Screen::Main)),
        }),
        ].spacing(15).width(Fill).height(Fill).align_x(Center).into()
    }
}
