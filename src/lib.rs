use home::home_dir;
use std::path::PathBuf;

pub fn get_proj_paths() -> (PathBuf, PathBuf) {
    let path = home_dir()
        .expect("Could not locate home directory.")
        .join(".suru");
    let db_path = path.join("data.db");

    (path, db_path)
}
