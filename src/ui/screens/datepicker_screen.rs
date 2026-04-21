use iced::Element;
use iced::widget::{text, column, button};
use iced::Length::Fill;
use iced::Alignment::Center;

use crate::app::state::AppState;
use crate::app::message::Message;
use crate::ui::components::calendar::{CalendarType};
use crate::ui::screens::Screen;
use crate::ui::components::buttons::Buttons;

pub fn view(state: &AppState, calendar_type: CalendarType, cpf: Option<String>) -> Element<'_, Message>{
    let screen_retorno = match calendar_type{
        CalendarType::StartFilter => Screen::Employee(cpf.unwrap()),
        CalendarType::EndFilter => Screen::Employee(cpf.unwrap()),
        CalendarType::DailyEvents => Screen::Main,
        CalendarType::StartDayOffCompany => Screen::CompanyDayOff,
        CalendarType::EndDayOffCompany => Screen::CompanyDayOff,
    };
    column![
        text("DatePicker"),
        state.sel_dates.view(calendar_type),
        button("Confirmar").on_press(Message::ButtonPressed(Buttons::SwitchScreen(screen_retorno))),
    ].spacing(15).width(Fill).height(Fill).align_x(Center).into()
}
