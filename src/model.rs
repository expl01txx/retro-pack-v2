pub enum InputMode {
    Editing,
}

pub struct App {
    pub filename: String,
    pub password: String,
    pub row: i32,
    pub input_mode: InputMode,
    pub messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            filename: String::new(),
            password: String::new(),
            row: 0,
            input_mode: InputMode::Editing,
            messages: Vec::new(),
        }
    }
}
