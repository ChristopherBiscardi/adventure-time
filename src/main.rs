use adventure_time::{config::get_manifest, init};
use clap::{crate_description, crate_version, App, AppSettings, Arg};
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Adventure Time")
        .version(crate_version!())
        .author("Chris Biscardi <chris@christopherbiscardi.com>")
        .about(crate_description!())
        .setting(AppSettings::ColoredHelp)
        .subcommand(App::new("init"))
        .subcommand(
            App::new("watch")
                .about("Run all the tests for a course and watch for changes.")
                .arg(
                    Arg::new("debug")
                        .short('d')
                        .about("print debug information verbosely"),
                ),
        )
        .subcommand(
            App::new("list").about("list courses").arg(
                Arg::new("course")
                    .short('c')
                    .about("List the lessons in a course"),
            ),
        )
        .subcommand(
            App::new("init")
                .about("start working on a new course in a new directory")
                .arg(
                    Arg::new("course")
                        .short('c')
                        .about("The course id you want to start"),
                ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("init") => init(matches.subcommand_matches("init").unwrap()),
        Some("list") => {}
        Some("watch") => {}
        _ => {}
    };
    Ok(())
}
