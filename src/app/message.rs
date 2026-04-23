use crate::ui::components::{buttons::Buttons, textinputs::TextInputsEnum};
use crate::domain::info_add::info_add::DayOffType;

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed(Buttons),
    TextInputChanged(TextInputsEnum, String),
    DayOffTypePicked(DayOffType)
}
