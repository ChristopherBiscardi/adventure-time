use directories::ProjectDirs;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const BUILTIN_MANIFEST_STR: &str = include_str!("../repos.yaml");

pub fn get_manifest() -> Manifest {
    serde_yaml::from_str(BUILTIN_MANIFEST_STR).unwrap()
}

lazy_static! {
    /// This is an example for using doc comment attributes
    pub static ref CONFIG_DIR: PathBuf = {
let p =        ProjectDirs::from("com", "christopherbiscardi", "adventure-time").expect("expected to get config dir");
p    .config_dir().to_path_buf()
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    courses: Vec<Course>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    name: String,
    description: String,
    repo: String,
    tags: Vec<String>,
}
