use super::state::{AppState};
use iced::{Task as Command};
use super::message::{Message};
use rfd::FileDialog;
use chrono::{Datelike, Months, Local};

use crate::ui::screens::Screen;
use crate::ui::components::buttons::Buttons;
use crate::ui::components::calendar::{CalendarMessage};
use crate::ui::components::textinputs::TextInputsEnum;

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
                        Screen::DatePicker(calendar_type, cpf)=>{
                            println!("DatePicker");
                            state.current_screen = Screen::DatePicker(calendar_type, cpf)
                        }
                        Screen::CompanyDayOff => {
                            println!("CompanyDayOff");
                            state.current_screen = screen
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
                Buttons::GetInfoAdd => {
                    if let Some(path) = FileDialog::new()
                        .add_filter("json", &["json"])
                        .set_title("SELECIONE A APURACAO")
                        .pick_file()
                    {
                        state.load_info_add(path);
                    }else{
                        println!("Nenhum arquivo selecionado")
                    }
                },
                Buttons::SaveInfoAdd =>{
                    let now = Local::now();
                    let filename = format!(
                        "infoadd_{}.json",
                        now.format("%d-%m-%Y_%H-%M-%S")
                );
                    if let Some(path) = FileDialog::new()
                        .set_file_name(&filename)
                        .add_filter("json", &["json"])
                        .set_title("SALVAR INFOADD")
                        .save_file()
                    {
                        state.save_info_add(path);
                    }else{
                        println!("SALVAMENTO CANCELADO")
                    }
                },
                Buttons::TallyData => {
                    state.load_tally()
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
                Buttons::CreateManualPonto(manualponto)=>{
                    state.info_add.create_manual_ponto(manualponto);
                    state.load_tally();
                    println!("{:?}", state.info_add)
                }
                Buttons::EditManualPonto(manualponto)=>{
                    state.info_add.edit_manual_ponto(manualponto);
                    state.load_tally();
                    println!("{:?}", state.info_add)
                }
                Buttons::DeleteManualPonto(manualponto)=>{
                    state.info_add.delete_manual_ponto(manualponto);
                    state.load_tally();
                    println!("{:?}", state.info_add)
                }
            }
        }
        Message::TextInputChanged(textinput, valor)=>{
            match textinput{
                TextInputsEnum::DiaAlterandoEmployeeScreen =>{
                    state.text_inputs.dia_alterando_employee_screen = valor
                },
                TextInputsEnum::DiaAdicionandoEmployeeScreen=>{
                    state.text_inputs.dia_adicionando_employee_screen = valor
                },
                TextInputsEnum::FiltroFuncionarioEmployeesScreen=>{
                    state.text_inputs.filtro_funcionario_employee_screen = valor
                }
                TextInputsEnum::MoreInfoCompanyDayOffScreen=>{
                    state.text_inputs.more_info_company_day_off_screen = valor
                }
            }
        }
    }

    Command::none()
}
