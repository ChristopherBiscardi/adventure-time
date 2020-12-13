use config::CONFIG_DIR;

pub mod config;
mod runner;

pub fn run() {}

pub fn init() {
    println!("{}", CONFIG_DIR.display());
}
