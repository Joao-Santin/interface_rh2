use iced::Color;
use chrono::{NaiveDate, NaiveDateTime};
use iced::Element;
use iced::widget::{row, button, column, text, text_input, Column};
use iced::Alignment::Center;
use iced::Length::{Fixed, Fill};

use crate::domain::info_add::info_add::{ManualPonto, TypeOrigin};
use crate::extensions::chrono_ext::is_valid_naivedatetime;
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
    let spacing_hour = 180.0;
    let elementmarcacoes: Vec<Element<Message>> = grouped
        .iter()
        .map(|(date, registros)| {
            let horarios = registros.iter().enumerate().map(|(count, (dt, tall))| {
                if tall.entry_manu.is_some() && tall.entry_marca.is_some(){
                    if let TypeOrigin::Correcao(ref original) = tall.origin{
                        text(format!("ponto n.{}: {}({})", count+1, dt.format("%H:%M").to_string(), original.date_time.time().format("%H:%M").to_string())).width(Fixed(spacing_hour)).style(|_| iced::widget::text::Style{
                            color: Some(Color::from_rgb(0.0, 1.0, 0.0))
                        }).into()

                    }else{
                        text(format!("ponto n.{}: {}(manu.)", count+1, dt.format("%H:%M").to_string())).width(Fixed(spacing_hour)).into()

                    }

                }else if tall.entry_manu.is_some(){
                    text(format!("ponto n.{}: {}(cria.)", count+1, dt.format("%H:%M").to_string())).width(Fixed(spacing_hour)).style(|_| iced::widget::text::Style{
                            color: Some(Color::from_rgb(1.0, 1.0, 0.0))
                        }).into()
                }else{
                    text(format!("ponto n.{}: {}(auto.)", count+1, dt.format("%H:%M").to_string())).width(Fixed(spacing_hour)).into()
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
            button("FERIAS E FALTAS!").on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::EmployeeDayOff(cpf.clone()))))
        ],
        row![
            text("Do dia "),
            button(text(format!("{:?}", state.sel_dates.selected_date.get(&CalendarType::StartFilter).map(|d| d.to_string()).unwrap_or("Sem data".to_string())))).on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::DatePicker(CalendarType::StartFilter, Some(cpf.clone()))))),
            text(" ao "),
            button(text(format!("{:?}", state.sel_dates.selected_date.get(&CalendarType::EndFilter).map(|d| d.to_string()).unwrap_or("Sem data".to_string())))).on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::DatePicker(CalendarType::EndFilter, Some(cpf))))),
        ],
        row![
            column![
                row![
                    text("MANIPULAR DATA"),
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
        coluna_marcacoes,
        button("To Employees").on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::Employees))),
    ].width(Fill).height(Fill).align_x(Center).spacing(10.0).into()
}
