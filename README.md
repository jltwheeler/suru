# 🗒️ Suru

CLI Todo app written in Rust 🦀

# Features

- Uses sqlite (`rusqlite`) to manage data
- Use `clap` for parsing cli arguments, `ansi_term` for coloring terminal and
`dialoguer` for interactive terminal prompt
- Commands:
  - suru init
  - suru --help
  - suru --version
  - suru add  - this could be an interactive prompt to get todo task title,
  desc, due date, category
  - suru rm [id]
  - suru close [id]
  - suru open [id]
  - suru list - this can have option flags for filtering the ordered list
  - suru import - import todos from csv or json
  - suru export - export todos to csv or json
  - suru destroy - removes local db instance
- Can view todos in browser via a local server
  - Create a local api
  - Yew for front end?
- Make all the cli viewing a nice experience:
  - 
