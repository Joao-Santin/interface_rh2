use iced::{Element};
use iced::widget::{button, column, row, text, text_input, Column, pick_list, container};
use iced::Alignment::Center;
use iced::Length::{Fill, Fixed};

use crate::ui::components::calendar::CalendarType;
use crate::app::state::AppState;
use crate::app::message::Message;
use crate::ui::components::buttons::Buttons;
use crate::ui::Screen;
use crate::domain::info_add::info_add::{CompanyDayOff, DayOffType};
use crate::ui::components::textinputs::{TextInputsEnum};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let start_search_period = state.sel_dates.selected_date.get(&CalendarType::StartDayOffCompany).unwrap();
    let end_search_period = state.sel_dates.selected_date.get(&CalendarType::EndDayOffCompany).unwrap();
    let dayoffs: Vec<&CompanyDayOff> = state.info_add.company_day_off
        .iter()
        .filter(|cdo| {
            cdo.start <= *end_search_period && cdo.end >= *start_search_period
        })
        .collect();
    let element_dayoffs: Vec<Element<Message>> = dayoffs
        .iter()
        .cloned()
        .map(|cdo| {
            row![
            text(format!("{} | {} à {} | OBS: {}",cdo.typ.to_string(), cdo.start.to_string(), cdo.end.to_string(), cdo.more_info.to_string())),
            button("DELETAR").on_press(Message::ButtonPressed(Buttons::DeleteCompanyDayOff(cdo.clone())))
            ].into()
        })
        .collect();
    let motivos = [
        DayOffType::CollectiveLeave,
        DayOffType::Holiday
    ];
    let column_dayoffs = Column::with_children(element_dayoffs);
    container(
        column![
            row![
                text("FERIADOS E FERIAS COLETIVAS")
                            .width(Fill)
                            .size(60)
                            .color([0.5, 0.5, 0.5])
                            .align_x(Center),
            ],
            button("VOLTAR").on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::Main))),
            container(
                column![
                    text("CRIAR").width(Fill).size(30).color([0.5, 0.5, 0.5]).align_x(Center),
                    row![
                        pick_list(motivos, state.picked_dayoff_company_creating, Message::DayOffCompanyTypePicked).placeholder("selecione o motivo!")
                    ],
                    row![
                        text("DO DIA"),
                        button(text(format!("{:?}", state.sel_dates.selected_date.get(&CalendarType::StartDayOffCompanyCreating).map(|d| d.to_string()).unwrap_or("Sem data".to_string())))).on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::DatePicker(CalendarType::StartDayOffCompanyCreating, None)))),
                        text("AO DIA"),
                        button(text(format!("{:?}", state.sel_dates.selected_date.get(&CalendarType::EndDayOffCompanyCreating).map(|d| d.to_string()).unwrap_or("Sem data".to_string())))).on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::DatePicker(CalendarType::EndDayOffCompanyCreating, None)))),
                    ],
                    row![
                        text_input("O QUE ACONTECEU?", &state.text_inputs.more_info_company_day_off_screen).width(Fixed(300.0)).on_input(|v| Message::TextInputChanged(TextInputsEnum::MoreInfoCompanyDayOffScreen, v))
                    ],
                    row![
                        if let (Some (typ), Some(start), Some(end)) = (
                    state.picked_dayoff_company_creating.clone(),
                    state.sel_dates.selected_date.get(&CalendarType::StartDayOffCompanyCreating).copied(),
                    state.sel_dates.selected_date.get(&CalendarType::EndDayOffCompanyCreating).copied(),

                ){
                    button("CRIAR DIA!").on_press(Message::ButtonPressed(Buttons::CreateCompanyDayOff(CompanyDayOff{
                        typ,
                        start,
                        end,
                        more_info: state.text_inputs.more_info_company_day_off_screen.clone()
                    })))

                    }else{
                            button("PREENCHA O QUE FALTA!!!")
                        }
                    ]
                ].width(Fill).align_x(Center).spacing(10.0),
            ).width(Fill).center_x(Fill),
            column![
                text("PROCURAR").width(Fill).size(30).color([0.5, 0.5, 0.5]).align_x(Center),
                row![
                    text("DO DIA"),
                    button(text(format!("{:?}", state.sel_dates.selected_date.get(&CalendarType::StartDayOffCompany).map(|d| d.to_string()).unwrap_or("Sem data".to_string())))).on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::DatePicker(CalendarType::StartDayOffCompany, None)))),
                    text("AO DIA"),
                    button(text(format!("{:?}", state.sel_dates.selected_date.get(&CalendarType::EndDayOffCompany).map(|d| d.to_string()).unwrap_or("Sem data".to_string())))).on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::DatePicker(CalendarType::EndDayOffCompany, None))))
                ].spacing(10),
            ].width(Fill).align_x(Center).spacing(10),
            column_dayoffs,
        ].spacing(30.0)
    ).width(Fill).height(Fill).center_x(Fill).into()
}
