use crate::ui::components::{buttons::Buttons, textinputs::TextInputsEnum};

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed(Buttons),
    TextInputChanged(TextInputsEnum, String),
}
