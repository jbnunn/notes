# Notes

Notes is a simple tool to manage notes and to-do lists. Running this tool as `notes` opens your IDE and creates a new Markdown file with today's date (or opens the file for you if it already exists). You can take notes and create to-do's for yourself with the following syntax:

```
## Tues 2025-08-12

I don't know why it took me so long to realize this didn't need to be Python or Rust ... a simple Bash script does the trick. Now <90 lines of code with no dependencies.
 
## Thu 2024-07-18

Today I moved my original `notes` code from Python to Rust. Benefits:

* Cleaner code
* Fewer dependencies
* Rust

- [] This is an open to-do
- [] Review documentation after switching to Bash
- [X] Migrate `notes` from Python to Rust
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

`notes [--projects | todos | <project_name>]`

- Run without arguments to create/open the daily note.
- `--projects` to list all project note files.
- `todos` to display a list of all to-do items from your notes.
- `<project_name>` to create/open a specific project note file.

## Installation

1. Clone the repository or download the source code.
2. Update the `EDITOR` in [./notes.sh](notes.sh), eg "nano", "code", "nvim" etc
3. Create a symlink:

```
ln -s /path/to/notes/notes.sh /usr/local/bin/notes
```

## Configuration

By default, the application expects your notes to be stored in the `~/Documents/notes` directory. If you want to change this location, you'll need to modify the code accordingly.

## About

This is a Bash rewrite of my original Python code, which I've used since 2020 for daily notes and managing my to-do's. After experimenting with Rust, I realized a simple Bash script was the most practical solution with no dependencies.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.
