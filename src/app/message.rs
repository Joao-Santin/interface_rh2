use crate::ui::components::{buttons::Buttons, textinputs::TextInputsEnum, checkboxes::CheckBoxes};
use crate::domain::info_add::info_add::DayOffType;

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed(Buttons),
    TextInputChanged(TextInputsEnum, String),
    DayOffCompanyTypePicked(DayOffType),
    DayOffEmployeeTypePicked(DayOffType),
    CheckBoxToggled(CheckBoxes, bool),
}
