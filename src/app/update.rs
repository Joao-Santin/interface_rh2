use super::state::{AppState};
use iced::{Task as Command};
use super::message::{Message};
use rfd::FileDialog;
use chrono::{Datelike, Months};

use crate::ui::screens::Screen;
use crate::ui::components::buttons::Buttons;
use crate::ui::components::calendar::{CalendarMessage};

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
                        Screen::Employee(cpf) => {
                            println!("Employee");
                            state.current_screen = Screen::Employee(cpf)
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
                Buttons::GetTally => {
                    if let Some(path) = FileDialog::new()
                        .add_filter("json", &["json"])
                        .set_title("SELECIONE A APURACAO")
                        .pick_file()
                    {
                        state.load_tally(path);
                    }else{
                        println!("Nenhum arquivo selecionado")
                    }
                },
                Buttons::CalendarButton(calendarmessage) => {
                    match calendarmessage{
                        CalendarMessage::SelectDay(calendartype, day)=>{
                            if day>0{
                                if let Some(date) = state.sel_dates.selected_date.get_mut(&calendartype){
                                    if let Some(valid_date) = date.with_day(day){
                                        *date = valid_date
                                    }
                                }
                            }
                        },
                        CalendarMessage::SelectMonth(calendartype, month)=>{
                            if let Some(date) = state.sel_dates.selected_date.get_mut(&calendartype){
                                if let Some(valid_date) = date.with_month(month){
                                    *date = valid_date
                                }
                            }
                        }
                        CalendarMessage::PreviousYear(calendartype)=>{
                            if let Some(date) = state.sel_dates.selected_date.get_mut(&calendartype){
                                let mut year = date.year();
                                year -= 1;
                                if let Some(new_date) = date.with_year(year){
                                    *date = new_date
                                };
                            }
                        },
                        CalendarMessage::NextYear(calendartype)=>{
                            if let Some(date) = state.sel_dates.selected_date.get_mut(&calendartype){
                                let mut year = date.year();
                                year += 1;
                                if let Some(new_date) = date.with_year(year){
                                    *date = new_date
                                };
                            }
                        },
                        CalendarMessage::PreviousMonth(calendartype)=>{
                            if let Some(date) = state.sel_dates.selected_date.get_mut(&calendartype){
                                if let Some(new_date) = date.checked_sub_months(Months::new(1)){
                                *date = new_date
                                }
                            }
                        }
                        CalendarMessage::NextMonth(calendartype)=>{
                            if let Some(date) = state.sel_dates.selected_date.get_mut(&calendartype){
                                if let Some(new_date) = date.checked_add_months(Months::new(1)){
                                *date = new_date
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Command::none()
}
