use rusqlite::{Connection, Result};
use std::fs::create_dir;
use std::process::Command;
use suru::get_proj_paths;

pub fn init_command() -> Result<()> {
    let (path, db_path) = get_proj_paths();

    if !path.exists() {
        Command::new("sqlite3")
            .arg("--version")
            .output()
            .expect("sqlite3 is required to be installed.");

        create_dir(&path).ok();

        let conn = Connection::open(&db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS todos (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             title TEXT NOT NULL UNIQUE,
             description TEXT NOT NULL,
             priority TEXT NOT NULL,
             created_at Timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
         )",
            [],
        )?;

        println!(
            "Initialised a database at the following path: {:?}",
            &db_path
        );
    } else {
        println!("The path: {:?} already exists, exiting command...", path);
    }

    Ok(())
}
