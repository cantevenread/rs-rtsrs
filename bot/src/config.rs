use dotenv::dotenv;
use std::env;

use crate::error::BotError;

#[derive(Debug, Clone)]
pub struct Config {
    pub token: String,
    pub owner_id: String,
}

pub fn init_config() -> Result<Config, BotError> {
    dotenv().ok();

    let token =
        env::var("TOKEN").expect("Error: Cannot find the corresponding environment variable");
    let owner_id =
        env::var("OWNER_ID").expect("Error: Cannot find the corresponding environment variable");

    let config = Config { token, owner_id };

    if config.token.is_empty() || config.owner_id.is_empty() {
        return Err(
            BotError::new("Error: Cannot find the corresponding environment variable".to_string())
        );
    } else {
        return Ok(config);
    }
}
