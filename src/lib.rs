use config::{get_manifest, CONFIG_DIR};

pub mod config;
mod runner;

pub fn run() {}

pub fn init(args: &clap::ArgMatches) {
    // println!("{}", CONFIG_DIR.display());
    // dbg!(args);
    let manifest = get_manifest();
    dbg!(manifest);
}

// async fn download(url: String) {
//     let body = reqwest::get("https://github.com/ChristopherBiscardi/advent-of-code/archive/master.zip")
//     .await?
//     .text()
//     .await?;
//     println!("body = {:?}", body);

// }