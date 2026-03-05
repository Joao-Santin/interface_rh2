use crate::ui::screens::Screen;

#[derive(Debug, Clone)]
pub enum Message {
    SwitchScreen(Screen),
}
