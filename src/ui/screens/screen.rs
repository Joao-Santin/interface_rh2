#[derive(Debug, Clone, Copy)]
pub enum Screen{
    Main,
    AFDEvents,
    Employees
}
impl Default for Screen {
    fn default() -> Self {
        Screen::Main
    }
}
