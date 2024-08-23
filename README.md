# Notes (A Rust CLI Tool)

Notes is a simple tool to manage notes and to-do lists. Running this tool as `notes` opens your IDE and creates a new Markdown file with today's date (or opens the file for you if it already exists). You can take notes and create to-do's for yourself with the following syntax:

```
# Thu 2024-07-18

Today I moved my original `notes` code from Python to Rust. Benefits:

* Cleaner code
* Fewer dependencies
* Rust

- [] This is an open to-do
- [] Migrate `notes` from Python to Rust
- [X] This is a to-do that has been completed
```

The CLI can also list of all your open to-do items from the command line with `notes todos`:

![notes-help](/notes-todos.png)

## Features

- **Create Daily Notes**: If you run the application without any arguments, it will create a new daily note file (or open an existing one) in the `~/Documents/notes/daily` directory. The file name is in the format `YYYY-MM-DD.md`.

- **Create Project Notes**: If you provide a project name as an argument, the application will create a new project note file (or open an existing one) in the `~/Documents/notes/projects` directory. The file name is in the format `project_name.md`.

- **List Projects**: If you provide the `--projects` argument, the application will list all your existing project note files.

- **View To-Do List**: If you provide the `todos` argument, the application will scan all your note files and display a list of all your to-do items. To-do items are detected using the format `- [] task description`.

## Usage

![notes-help](/notes-help.png)


`notes [--projects | todos | <project_name>]`

- Run without arguments to create/open the daily note.
- `--projects` to list all project note files.
- `todos` to display a list of all to-do items from your notes.
- `<project_name>` to create/open a specific project note file.

## Requirements

- [Rust toolchain](https://www.rust-lang.org/tools/install)
- An IDE. This script is configured to open [Visual Studio Code](https://code.visualstudio.com/) but can be changed to your IDE of choice.

## Installation

1. Clone the repository or download the source code.
2. Navigate to the project directory in your terminal.
3. Build the application using the Rust toolchain: `cargo build --release`.
4. The compiled binary will be located in the `target/release` directory.
5. (Optional) Copy the binary to a location in your PATH, e.g., `sudo cp target/release/notes /usr/local/bin`. Then, run `notes` from anywhere in your command line.

## Configuration

By default, the application expects your notes to be stored in the `~/Documents/notes` directory. If you want to change this location, you'll need to modify the code accordingly.

## About

This is a rewrite of my original Python code, which I've used since 2020 for daily notes and managing my to-do's.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.
