use chrono::prelude::*;
use colored::Colorize;
use regex::Regex;
use std::fs::{read_dir, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match &args[1][..] {
            "--projects" => list_projects(),
            "todos" => {
                let mut path = dirs::home_dir().expect("Failed to get home directory");
                path.push("Documents/notes");
                let total_todo_count = search_todos(Path::new(&path));
                if total_todo_count > 0 {
                    println!(
                        "{}",
                        format!(
                            "You have {} to-do{}",
                            total_todo_count,
                            if total_todo_count == 1 { "" } else { "s" }
                        )
                        .green()
                        .bold()
                    );
                } else {
                    println!("{}", "You have no to-do's".green().bold());
                }
            }
            "--help" => print_help(),
            project_name => open_or_create_project(project_name),
        }
    } else {
        open_or_create_daily_note();
    }
}

fn open_or_create_daily_note() {
    let mut path = dirs::home_dir().expect("Failed to get home directory");
    path.push("Documents/notes/daily");
    std::fs::create_dir_all(&path)
        .unwrap_or_else(|_| panic!("Failed to create directory: {:?}", path));

    let today = Local::now();
    let file_name = format!("{}.md", today.format("%Y-%m-%d"));
    let mut file_path = path;
    file_path.push(file_name);

    if file_path.exists() {
        open_with_vscode(&file_path);
    } else {
        let mut file = File::create(file_path.clone())
            .unwrap_or_else(|_| panic!("Failed to create file: {:?}", file_path));

        let header = format!(
            "# {} {}\n\n",
            today.format("%a").to_string(),
            today.format("%Y-%m-%d")
        );
        file.write_all(header.as_bytes())
            .unwrap_or_else(|_| panic!("Failed to write to file: {:?}", file_path));

        println!("Created file: {:?}", file_path);
        open_with_vscode(&file_path);
    }
}

fn open_or_create_project(project_name: &str) {
    let mut path = dirs::home_dir().expect("Failed to get home directory");
    path.push("Documents/notes/projects");
    std::fs::create_dir_all(&path)
        .unwrap_or_else(|_| panic!("Failed to create directory: {:?}", path));

    let file_name = format!("{}.md", project_name);
    let mut file_path = path;
    file_path.push(file_name);

    if file_path.exists() {
        open_with_vscode(&file_path);
    } else {
        File::create(file_path.clone())
            .unwrap_or_else(|_| panic!("Failed to create file: {:?}", file_path));

        println!("Created file: {:?}", file_path);
        open_with_vscode(&file_path);
    }
}

fn list_projects() {
    let mut path = dirs::home_dir().expect("Failed to get home directory");
    path.push("Documents/notes/projects");

    let mut projects: Vec<_> = read_dir(&path)
        .unwrap_or_else(|_| panic!("Failed to read directory: {:?}", path))
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let file_name = entry.file_name().into_string().ok()?;
            if file_name.ends_with(".md") {
                Some(file_name[..file_name.len() - 3].to_owned())
            } else {
                None
            }
        })
        .collect();

    projects.sort_unstable();
    projects.dedup();

    for project in projects {
        println!("{}", project);
    }
}

fn search_todos(folder_path: &Path) -> i32 {
    let todo_regex = Regex::new(r"(?m)^(.*)-\s*\[\s*\](.*)$").unwrap();
    let completed_regex = Regex::new(r"(?m)-\s*\[x\]").unwrap();
    let mut total_todo_count = 0;
    let mut file_todos = Vec::new();

    if folder_path.is_dir() {
        for entry in std::fs::read_dir(folder_path).unwrap() {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    total_todo_count += search_todos(&path);
                } else if path.is_file() && path.extension().unwrap_or_default() == "md" {
                    if let Ok(contents) = std::fs::read_to_string(&path) {
                        let mut file_todo_count = 0;
                        let mut file_todos_str = String::new();

                        for capture in todo_regex.captures_iter(&contents) {
                            let todo = capture.get(0).map_or("", |m| m.as_str());
                            if !completed_regex.is_match(todo) {
                                file_todo_count += 1;
                                total_todo_count += 1;
                                file_todos_str.push_str(&format!("{}\n", todo.trim()));
                            }
                        }

                        if file_todo_count > 0 {
                            file_todos.push((
                                path.file_name().unwrap().to_str().unwrap().to_string(),
                                file_todos_str,
                            ));
                        }
                    }
                }
            }
        }
    }

    for (file_name, file_todos_str) in &file_todos {
        println!("{}", file_name);
        print!("{}", file_todos_str);
    }

    total_todo_count
}

fn print_help() {
    println!("USAGE:");
    println!("    notes [OPTIONS] [PROJECT_NAME]");
    println!();
    println!("DESCRIPTION:");
    println!(
        "    Notes is a command-line application that helps you manage your notes and to-do lists."
    );
    println!();
    println!("OPTIONS:");
    println!("    --projects      List all existing project note files");
    println!("    todos           Display a list of all to-do items from your notes");
    println!("    --help          Print this help information");
    println!();
    println!("ARGUMENTS:");
    println!("    PROJECT_NAME    Name of the project note file to create or open");
    println!();
    println!("EXAMPLES:");
    println!("    notes                       Create or open the daily note file");
    println!("    notes --projects            List all project note files");
    println!("    notes todos                 Show all to-do items from your notes");
    println!("    notes my-project            Create or open the \"my-project\" note file");
    println!();
    println!("By default, notes are stored in the ~/Documents/notes directory.");
    println!("Daily notes are saved in ~/Documents/notes/daily with the format YYYY-MM-DD.md.");
    println!(
        "Project notes are saved in ~/Documents/notes/projects with the format project_name.md."
    );
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn open_with_vscode(path: &PathBuf) {
    let mut working_dir = path.clone();
    working_dir.pop(); // Remove the file name to get the directory path

    let mut child = Command::new("code")
        .arg("--new-window")
        .arg("--reuse-window")
        .arg("--folder-uri")
        .arg(format!("file://{}", working_dir.to_str().unwrap()))
        .arg(path)
        .spawn()
        .expect("Failed to open file with VS Code");

    let exit_status = child.wait().expect("Failed to wait for VS Code process");

    if !exit_status.success() {
        eprintln!("Failed to open file with VS Code");
    }
}

#[cfg(target_os = "windows")]
fn open_with_vscode(path: &PathBuf) {
    let mut working_dir = path.clone();
    working_dir.pop(); // Remove the file name to get the directory path

    let mut child = Command::new("cmd")
        .args(&[
            "/C",
            "start",
            "code",
            "--new-window",
            "--reuse-window",
            "--folder-uri",
            format!("file:///{}", working_dir.to_str().unwrap()).as_str(),
            path.to_str().unwrap(),
        ])
        .spawn()
        .expect("Failed to open file with VS Code");

    let exit_status = child.wait().expect("Failed to wait for VS Code process");

    if !exit_status.success() {
        eprintln!("Failed to open file with VS Code");
    }
}
