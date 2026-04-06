
use chrono::{NaiveDate, NaiveDateTime};
use iced::Element;
use iced::widget::{row, button, column, text, text_input, Column};

use crate::ui::components::buttons::Buttons;
use crate::ui::components::textinputs::TextInputsEnum;
use crate::ui::Screen;
use crate::app::state::AppState;
use crate::app::message::Message;
use crate::domain::tally::tally::Tally;
use crate::ui::components::calendar::CalendarType;

pub fn view(state: &AppState, cpf: String) -> Element<'_, Message> {
    let search_period = state.sel_dates.selected_date.get(&CalendarType::StartFilter)..=state.sel_dates.selected_date.get(&CalendarType::EndFilter);
    let mut marcacoes: Vec<(NaiveDateTime, Tally)> = state.tally
        .iter()
        .filter(|(_, tally)| tally.cpf == cpf)
        .filter(|(dt, _)| search_period.contains(&Some(&dt.date())))
        .map(|(dt, tally)| (*dt, tally.clone()))
        .collect();
    marcacoes.sort_by_key(|(dt, _)| *dt);
    let mut grouped: Vec<(NaiveDate, Vec<(NaiveDateTime, Tally)>)> = Vec::new();
    for (dt, tally) in marcacoes {
        if let Some((last_date, vec)) = grouped.last_mut() {
            if *last_date == dt.date() {
                vec.push((dt, tally));
                continue;
            }
        }
        grouped.push((dt.date(), vec![(dt, tally)]));
    }
    grouped.sort_by_key(|(date, _)| *date);
    let elementmarcacoes: Vec<Element<Message>> = grouped
        .iter()
        .map(|(date, registros)| {
            let horarios = registros.iter().enumerate().map(|(count, (dt, tall))| {
                if tall.entry_manu.is_some() && tall.entry_marca.is_some(){
                    text(format!("ponto n.{}: {}(corre.)", count+1, dt.format("%H:%M").to_string())).into()

                }else if tall.entry_manu.is_some(){
                    text(format!("ponto n.{}: {}(cria.)", count+1, dt.format("%H:%M").to_string())).into()
                }else{
                    text(format!("ponto n.{}: {}(auto.)", count+1, dt.format("%H:%M").to_string())).into()
                }

            });

            row![
                text(date.format("%d/%m/%Y").to_string()),
                row(horarios.collect::<Vec<_>>()).spacing(10.0)
            ]
            .spacing(10.0).into()
        })
        .collect();
    let coluna_marcacoes = Column::with_children(elementmarcacoes);
    column![
        text("Employee Screen"),
        text(format!("CPF:{} NOME:{}",
            &cpf,
            state
                .employees
                .get(&cpf)
                .map(|s| s.as_str())
                .unwrap_or("Nao encontrado")
        )),
        row![
            text("Do dia "),
            button(text(format!("{:?}", state.sel_dates.selected_date.get(&CalendarType::StartFilter).map(|d| d.to_string()).unwrap_or("Sem data".to_string())))).on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::DatePicker(CalendarType::StartFilter, Some(cpf.clone()))))),
            text("ao dia "),
            button(text(format!("{:?}", state.sel_dates.selected_date.get(&CalendarType::EndFilter).map(|d| d.to_string()).unwrap_or("Sem data".to_string())))).on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::DatePicker(CalendarType::EndFilter, Some(cpf))))),
        ],
        row![
            text("Adicionar dia."),
            text_input("dia", &state.text_inputs.dia_adicionando_employee_screen).on_input(|v| Message::TextInputChanged(TextInputsEnum::DiaAdicionandoEmployeeScreen, v)),
        ],
        coluna_marcacoes,
        button("To Employees").on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::Employees))),
    ].spacing(10.0).into()
}
