use std::path::PathBuf;

use bin::ls::LsOption;
pub mod bin;

pub fn ls(path: Option<PathBuf>) {
    bin::ls::Ls::new(path, LsOption::new().build()).handle();
}
