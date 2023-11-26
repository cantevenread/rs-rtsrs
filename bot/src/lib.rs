use lazy_static::lazy_static;

mod config;
mod error;

lazy_static! {
    #[derive(Debug, Clone)]
    pub static ref CONFIG: config::Config = config::init_config().unwrap();
}
