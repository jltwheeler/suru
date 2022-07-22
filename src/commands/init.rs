use home::home_dir;
use rusqlite::{Connection, Result};
use std::fs::create_dir;
use std::process::Command;

pub fn init_command() -> Result<()> {
    let path = home_dir()
        .expect("Could not locate home directory.")
        .join(".suru");
    let db_path = &path.join("data.db");

    if !path.exists() {
        Command::new("sqlite3")
            .arg("--version")
            .output()
            .expect("sqlite3 is required to be installed.");

        create_dir(&path).ok();

        let conn = Connection::open(db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS todos (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             name TEXT NOT NULL UNIQUE
         )",
            [],
        )?;

        println!(
            "Initialised a database at the following path: {:?}",
            db_path
        );
    } else {
        println!("The path: {:?} already exists", path);
    }

    Ok(())
}
