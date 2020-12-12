use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

const BUILTIN_MANIFEST_STR: &str = include_str!("../repos.yaml");

pub fn get_manifest() -> Manifest {
    serde_yaml::from_str(BUILTIN_MANIFEST_STR).unwrap()
}

pub fn get_config_dir() {
    if let Some(proj_dirs) = ProjectDirs::from("com", "christopherbiscardi", "adventure-time") {
        proj_dirs.config_dir();
        dbg!(proj_dirs.config_dir());
    }
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
