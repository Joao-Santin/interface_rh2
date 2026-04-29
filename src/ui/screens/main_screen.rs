use iced::{Element, Color};
use iced::Alignment::Center;
use iced::Length::{Fill, Fixed};
use iced::widget::{button, column, text, row, container};
use crate::app::state::AppState;
use crate::app::message::Message;
use crate::ui::Screen;
use crate::ui::components::buttons::Buttons;

pub fn view(state: &AppState) -> Element<'_, Message> {
    let button_height = 30.0;
    let button_width = 200.0;
    let button_funcionarios = if state.last_afd_got.is_some() && state.last_add_info_got.is_some() {
        button("FUNCIONARIOS")
            .width(Fixed(button_width))
            .height(Fixed(button_height))
            .on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::Employees)))
    } else {
        button("FUNCIONARIOS")
            .width(Fixed(button_width))
            .height(Fixed(button_height))
    };
    let button_company_day_off = if state.last_afd_got.is_some() && state.last_add_info_got.is_some() {
        button("FERIADOS E FÉRIAS")
            .width(Fixed(button_width))
            .height(Fixed(button_height))
            .on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::CompanyDayOff)))
    } else {
        button("FERIADOS E FÉRIAS")
            .width(Fixed(button_width))
            .height(Fixed(button_height))
    };
    container(
        column![
            text("APURADOR DE PONTO")
                    .width(Fill)
                    .size(60)
                    .color([0.5, 0.5, 0.5])
                    .align_x(Center),
            // if let Some(data) = &state.last_afd_got{
            //     text(format!("Ultimo AFD: {}", data.to_string()))
            // }else{
            //     text("AFD PENDENTE!")
            // },
            // if let Some(data_add) = &state.last_add_info_got{
            //     text(format!("Ultimo Info Add: {}", data_add.to_string()))
            // }else{
            //     text("INFO ADD PENDENTE!")
            // },
            text("BUSCAR DADOS").width(Fill).size(20).color([0.5, 0.5, 0.5]).align_x(Center),
            row![
                button(text("BUSCAR DADOS AFD").color(if state.last_afd_got.is_some(){Color::from_rgb(0.0, 1.0, 0.0)}else{Color::from_rgb(1.0, 0.0, 0.0)})).width(Fixed(button_width)).height(Fixed(button_height))
                    .on_press(Message::ButtonPressed(Buttons::GetAFDFile)),
                button(text("BUSCAR INFO ADD").color(if state.last_add_info_got.is_some(){Color::from_rgb(0.0, 1.0, 0.0)}else{Color::from_rgb(1.0, 0.0, 0.0)})).width(Fixed(button_width)).height(Fixed(button_height))
                    .on_press(Message::ButtonPressed(Buttons::GetInfoAdd)),

            ].spacing(5.0),
            text("SALVAR DADOS").width(Fill).size(20).color([0.5, 0.5, 0.5]).align_x(Center),
            button(text("SALVAR INFO ADD")).width(Fixed(button_width)).height(Fixed(button_height))
                .on_press(Message::ButtonPressed(Buttons::SaveInfoAdd)),
            // button("ACONTECIMENTOS")
            //     .on_press(Message::ButtonPressed(Buttons::SwitchScreen(Screen::AFDEvents))),
            text("MUDAR PAGINA").width(Fill).size(20).color([0.5, 0.5, 0.5]).align_x(Center),

            button_funcionarios,
            button_company_day_off,
        ].spacing(10.0).width(Fill).height(Fill).align_x(Center)
    ).width(Fill).height(Fill).center_x(Fill).center_y(Fill).into()
}
