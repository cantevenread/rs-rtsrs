#[derive(Debug)]
pub struct BotError {
    pub message: String,
}

impl std::fmt::Display for BotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for BotError {}

impl BotError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
