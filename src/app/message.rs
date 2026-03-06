use crate::ui::components::buttons::Buttons;

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed(Buttons),
}
