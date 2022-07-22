use dialoguer::{theme::ColorfulTheme, Input, Select};
use rusqlite::{Connection, Result};
use suru::get_proj_paths;

struct Todo {
    title: String,
    description: String,
    priority: String,
}

pub fn add_command() -> Result<()> {
    let title_input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Todo title (keep it brief)")
        .interact_text()
        .unwrap();

    let description_input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Todo description")
        .interact_text()
        .unwrap();

    let priority_selections = ["low", "medium", "high", "urgent"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick the priority of the todo")
        .default(0)
        .items(&priority_selections)
        .interact()
        .unwrap();

    let todo = Todo {
        title: title_input,
        description: description_input,
        priority: priority_selections[selection].to_string(),
    };

    let (path, db_path) = get_proj_paths();
    if path.exists() {
        let conn = Connection::open(&db_path)?;
        conn.execute(
            "INSERT INTO todos (title, description, priority) VALUES (?1, ?2, ?3)",
            (&todo.title, &todo.description, &todo.priority),
        )?;
    } else {
        println!("Error: unable to open database at {:?}", db_path)
    }
    Ok(())
}
