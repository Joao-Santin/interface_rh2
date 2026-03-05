#[derive(Debug, Clone, Copy)]
pub enum Screen{
    Main,
    Teste
}
impl Default for Screen {
    fn default() -> Self {
        Screen::Main
    }
}
