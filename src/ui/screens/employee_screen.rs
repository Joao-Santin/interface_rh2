use iced::Color;
use chrono::{NaiveDateTime, Weekday, Datelike};
use iced::Element;
use iced::widget::{row, button, column, text, text_input, Column, scrollable};
use iced::Alignment::Center;
use iced::Length::{Fixed, Fill};

use crate::domain::info_add::info_add::{ManualPonto, TypeOrigin};
use crate::extensions::chrono_ext::{is_valid_naivedatetime, NaiveDateTimePtBr};
use crate::ui::components::buttons::Buttons;
use crate::ui::components::textinputs::TextInputsEnum;
use crate::ui::Screen;
use crate::app::state::AppState;
use crate::app::message::Message;
use crate::domain::tally::tally::{Tally, group_tally_by_day};
use crate::domain::day_result::day_result::{DayResult, total_balance_by_cpf_with_dates};
use crate::ui::components::calendar::CalendarType;

pub fn view(state: &AppState, cpf: String) -> Element<'_, Message> {
    let start = state.sel_dates.selected_date.get(&CalendarType::StartFilter);
    let end = state.sel_dates.selected_date.get(&CalendarType::EndFilter);
    let in_period = |date: chrono::NaiveDate| {
        match (start, end) {
            (Some(start), Some(end)) => date >= *start && date <= *end,
            (Some(start), None) => date >= *start,
            (None, Some(end)) => date <= *end,
            (None, None) => true,
        }
    };

    let mut marcacoes: Vec<(NaiveDateTime, Tally)> = state.tally
        .iter()
        .filter(|(_, tally)| tally.cpf.trim() == cpf.trim())
        .filter(|(dt, _)| in_period(dt.date()))
        // search_period.contains(&Some(&dt.date()))
        .map(|(dt, tally)| (*dt, tally.clone()))
        .collect();
    marcacoes.sort_by_key(|(dt, _)| *dt);
    println!("marcacoes: {:?}", marcacoes);
    let grouped = group_tally_by_day(&marcacoes);
    let spacing_hour = 180.0;
    let day_results: Vec<&DayResult> = state.day_result
        .iter()
        .filter(|dr| in_period(dr.date))
        .filter(|dr| dr.cpf.trim() == cpf.trim())
        .collect();
    println!("Grouped: {:?}", grouped);
    println!("DayResults: {:?}", day_results);
    let elementmarcacoes2: Vec<Element<Message>> = day_results
        .iter()
        .map(|dr| {
            println!("Procurando: {}", dr.date);
            let empty: Vec<(NaiveDateTime, Tally)> = Vec::new();
            let registros = grouped
                .iter()
                .find(|(dt, _)| {
                    *dt == dr.date
                })
                .map(|(_, registros)| registros)
                .unwrap_or(&empty);

            let horarios = registros
                .iter()
                .enumerate()
                .map(|(count, (dt, tall))| {

                    if tall.entry_manu.is_some() && tall.entry_marca.is_some(){
                        if let TypeOrigin::Correcao(ref original) = tall.origin{
                            text(format!(
                                "ponto n.{}: {}({})",
                                count+1,
                                dt.format("%H:%M"),
                                original.date_time.time().format("%H:%M")
                            ))
                            .width(Fixed(spacing_hour))
                            .style(|_| iced::widget::text::Style{
                                color: Some(Color::from_rgb(0.0, 1.0, 0.0))
                            })
                            .into()

                        } else {
                            text(format!("ponto n.{}: {}(manu.)", count+1, dt.format("%H:%M")))
                                .width(Fixed(spacing_hour))
                                .into()
                        }

                    } else if tall.entry_manu.is_some(){
                        text(format!("ponto n.{}: {}(cria.)", count+1, dt.format("%H:%M")))
                            .width(Fixed(spacing_hour))
                            .style(|_| iced::widget::text::Style{
                                color: Some(Color::from_rgb(1.0, 1.0, 0.0))
                            })
                            .into()
                    } else {
                        text(format!("ponto n.{}: {}(auto.)", count+1, dt.format("%H:%M")))
                            .width(Fixed(spacing_hour))
                            .into()
                    }

                });
            let saldo = dr.balance.num_minutes() as f32 / 60.0;

            let saldo_color = if saldo < 0.0 {
                Color::from_rgb(1.0, 0.0, 0.0) // vermelho
            } else if saldo > 0.0 {
                Color::from_rgb(0.0, 1.0, 0.0) // verde
            } else {
                Color::from_rgb(1.0, 1.0, 1.0) // branco
            };

            let resumo =column![
                row![
                    text(format!(
                        "Trab: {:.2}h | Esp: {:.2}h | Saldo: ",
                        dr.worked.num_minutes() as f32 / 60.0,
                        dr.expected.num_minutes() as f32 / 60.0,
                    )),
                    text(format!("{:.2}h", saldo))
                        .style(move |_| iced::widget::text::Style {
                            color: Some(saldo_color)
                        })
                ]

            ]; 
            let status = if registros.is_empty() {
                if dr.date.weekday() == Weekday::Sat || dr.date.weekday() == Weekday::Sun{
                    row![
                        text("FINAL DE SEMANA").style(|_| iced::widget::text::Style{
                            color: Some(Color::from_rgb(1.0, 0.5, 0.0))
                        })
                    ].spacing(10.0)

                }else if let Some(d) = state.info_add.company_day_off.iter().find(|d| dr.date >= d.start && dr.date <= d.end){
                    row![
                        text("DAY OFF EMPRESA").style(|_| iced::widget::text::Style{
                            color: Some(Color::from_rgb(1.0, 0.5, 0.0))
                        }),
                        text(format!("{}", d.typ.to_string())),
                        text(format!("OBS:{}", d.more_info))
                    ].spacing(10.0)
                    

                }else if let Some(d) = state.info_add.employee_day_off.iter().filter(|d| d.cpf == cpf).find(|d| dr.date>=d.start && dr.date <=d.end){
                    row![
                        text("DAY OFF FUNCIONARIO").style(|_| iced::widget::text::Style{
                            color: Some(Color::from_rgb(1.0, 0.5, 0.0))
                        }),
                        text(format!("{}", d.typ.to_string())),
                        text(format!("OBS:{}", d.more_info))
                    ].spacing(10.0)

                }else{
                    row![
                        text("FALTA").style(|_| iced::widget::text::Style{
                            color: Some(Color::from_rgb(1.0, 0.0, 0.0))
                        })
                    ].spacing(10.0)

                }
            } else {
                row![
                    text("")
                ]
            };

            row![
                text(format!(
                    "{} {}",
                    dr.date.format("%d/%m/%Y"),
                    dr.date.weekdayname_ptbr()
                ))
                .width(Fixed(210.0)),

                row(horarios.collect::<Vec<_>>()).spacing(10.0),

                status,
                resumo,
            ]
            .spacing(10.0)
            .into()
        })
    .collect();

    // let elementmarcacoes: Vec<Element<Message>> = grouped
    //     .iter()
    //     .map(|(date, registros)| {
    //         let horarios = registros.iter().enumerate().map(|(count, (dt, tall))| {
    //             if tall.entry_manu.is_some() && tall.entry_marca.is_some(){
    //                 if let TypeOrigin::Correcao(ref original) = tall.origin{
    //                     text(format!("ponto n.{}: {}({})", count+1, dt.format("%H:%M").to_string(), original.date_time.time().format("%H:%M").to_string())).width(Fixed(spacing_hour)).style(|_| iced::widget::text::Style{
    //                         color: Some(Color::from_rgb(0.0, 1.0, 0.0))
    //                     }).into()
    //
    //                 }else{
    //                     text(format!("ponto n.{}: {}(manu.)", count+1, dt.format("%H:%M").to_string())).width(Fixed(spacing_hour)).into()
    //
    //                 }
    //
    //             }else if tall.entry_manu.is_some(){
    //                 text(format!("ponto n.{}: {}(cria.)", count+1, dt.format("%H:%M").to_string())).width(Fixed(spacing_hour)).style(|_| iced::widget::text::Style{
    //                         color: Some(Color::from_rgb(1.0, 1.0, 0.0))
    //                     }).into()
    //             }else{
    //                 text(format!("ponto n.{}: {}(auto.)", count+1, dt.format("%H:%M").to_string())).width(Fixed(spacing_hour)).into()
    //             }
    //
    //         });
    //
    //         row![
    //             text(format!("{} {}", date.format("%d/%m/%Y").to_string(), date.weekdayname_ptbr())).width(Fixed(210.0)),
    //             row(horarios.collect::<Vec<_>>()).spacing(10.0)
    //         ].spacing(10.0)
    //         .spacing(10.0).into()
    //     })
    //     .collect();
    let coluna_marcacoes = Column::with_children(elementmarcacoes2);
    let button_confirmar: Element<Message> =
    if is_valid_naivedatetime(&state.text_inputs.dia_adicionando_employee_screen)
        && is_valid_naivedatetime(&state.text_inputs.dia_alterando_employee_screen)
    {
        if let (Ok(old_dt), Ok(new_dt)) = (
            NaiveDateTime::parse_from_str(
                &state.text_inputs.dia_alterando_employee_screen,
                "%d-%m-%Y %H:%M",
            ),
            NaiveDateTime::parse_from_str(
                &state.text_inputs.dia_adicionando_employee_screen,
                "%d-%m-%Y %H:%M",
            ),
        ) {
            if let Some(marcacao) = state.afd.marcacaoponto.iter().find(|m| {
                m.cpf_empregado == cpf && m.date_time == old_dt
            }) {
                button("Editar!")
                    .on_press(Message::ButtonPressed(
                        Buttons::EditManualPonto(ManualPonto {
                            typemanual: TypeOrigin::Correcao(marcacao.clone()),
                            date_time: new_dt,
                            cpf_empregado: cpf.clone(),
                        }),
                    ))
                    .into()
            } else {
                button("Editar!").into()
            }
        } else {
            button("Editar!").into()
        }
    } else if is_valid_naivedatetime(&state.text_inputs.dia_adicionando_employee_screen) {
        if let Ok(new_dt) = NaiveDateTime::parse_from_str(
            &state.text_inputs.dia_adicionando_employee_screen,
            "%d-%m-%Y %H:%M",
        ) {
            button("Criar!")
                .on_press(Message::ButtonPressed(
                    Buttons::CreateManualPonto(ManualPonto {
                        typemanual: TypeOrigin::Criacao,
                        date_time: new_dt,
                        cpf_empregado: cpf.clone(),
                    }),
                ))
                .into()
        } else {
            button("Criar!").into()
        }
    } else if is_valid_naivedatetime(&state.text_inputs.dia_alterando_employee_screen) {
        if let Ok(old_dt) = NaiveDateTime::parse_from_str(
            &state.text_inputs.dia_alterando_employee_screen,
            "%d-%m-%Y %H:%M",
        ) {
            button("Deletar!")
                .on_press(Message::ButtonPressed(
                    Buttons::DeleteManualPonto(ManualPonto{
                        typemanual: TypeOrigin::AFD,
                        date_time: old_dt,
                        cpf_empregado: cpf.clone()
                    },
                )))
                .into()
        } else {
            button("Deletar!").into()
        }
    } else {
        text("").into()
    };
    column![
        text("PONTOS FUNCIONARIO")
                    .width(Fill)
                    .size(60)
                    .color([0.5, 0.5, 0.5])
                    .align_x(Center),
        button("VOLTAR PARA ULTIMA TELA").on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::Employees))),
        text(format!("CPF: {} || NOME: {}",
            &cpf,
            state
                .employees
                .get(&cpf)
                .map(|s| s.as_str())
                .unwrap_or("Nao encontrado")
        )).width(Fill).size(20).color([0.5, 0.5, 0.5]).align_x(Center),
        row![
            button("FERIAS E FALTAS").on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::EmployeeDayOff(cpf.clone()))))
        ],
        row![
            column![
                row![
                    text("MANIPULAR DADOS").width(Fill).size(20).color([0.5, 0.5, 0.5]).align_x(Center),
                ],
                row![
                    text("Alterar:"),
                    text_input("dd-mm-aaaa hh:mm", &state.text_inputs.dia_alterando_employee_screen).on_input(|v| Message::TextInputChanged(TextInputsEnum::DiaAlterandoEmployeeScreen, v)).width(Fixed(200.0)).style(move |theme, status|{
                        let mut style = text_input::default(theme, status);
                        style.value = if is_valid_naivedatetime(&state.text_inputs.dia_alterando_employee_screen){
                            Color::from_rgb(0.0, 0.7, 0.0)
                        }else{
                                Color::from_rgb(0.7, 0.0, 0.0)
                            };
                        style
                    }),
                    text("Nova Data:"),
                    text_input("dd-mm-aaaa hh:mm", &state.text_inputs.dia_adicionando_employee_screen).on_input(|v| Message::TextInputChanged(TextInputsEnum::DiaAdicionandoEmployeeScreen, v)).width(Fixed(200.0)).width(Fixed(200.0)).style(move |theme, status|{
                        let mut style = text_input::default(theme, status);
                        style.value = if is_valid_naivedatetime(&state.text_inputs.dia_adicionando_employee_screen){
                            Color::from_rgb(0.0, 0.7, 0.0)
                        }else{
                                Color::from_rgb(0.7, 0.0, 0.0)
                            };
                        style
                    }),
                    button_confirmar,
                    // button("Confirmar!").on_press(Message::ButtonPressed(Buttons::CreateModifyDate(None, NaiveDateTime::parse_from_str(&state.text_inputs.dia_adicionando_employee_screen.trim(), "%d-%m-%Y %H:%M").unwrap())))

                ].spacing(5)
            ].width(Fill).align_x(Center),
        ].spacing(10),
        text("BUSCA DADOS").width(Fill).size(20).color([0.5, 0.5, 0.5]).align_x(Center),
        row![
            text("Do dia "),
            button(text(format!("{:?}", state.sel_dates.selected_date.get(&CalendarType::StartFilter).map(|d| d.to_string()).unwrap_or("Sem data".to_string())))).on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::DatePicker(CalendarType::StartFilter, Some(cpf.clone()))))),
            text(" ao dia "),
            button(text(format!("{:?}", state.sel_dates.selected_date.get(&CalendarType::EndFilter).map(|d| d.to_string()).unwrap_or("Sem data".to_string())))).on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::DatePicker(CalendarType::EndFilter, Some(cpf.clone()))))),
        ],
        row![
            text("BANCO HORAS PERIODO:"),
            text(format!("{}H", total_balance_by_cpf_with_dates(*state.sel_dates.selected_date.get(&CalendarType::StartFilter).unwrap(), *state.sel_dates.selected_date.get(&CalendarType::EndFilter).unwrap(), &state.day_result, &cpf.clone()).num_minutes() as f32 / 60.0))

        ],
        scrollable(coluna_marcacoes),
    ].width(Fill).height(Fill).align_x(Center).spacing(10.0).into()
}
