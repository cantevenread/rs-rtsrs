use lazy_static::lazy_static;

mod config;
mod error;

lazy_static! {
    pub static ref CONFIG: config::Config = config::init_config().unwrap();
}

fn main() {
    let h = &CONFIG;
    println!("{}, {}", h.token, h.owner_id);
}
