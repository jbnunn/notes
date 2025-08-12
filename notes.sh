#!/bin/bash

# Notes - A simple bash script for managing daily notes and todos
# Usage: notes [--projects | todos | <project_name>]

NOTES_DIR="$HOME/Documents/notes"
DAILY_DIR="$NOTES_DIR/daily"
PROJECTS_DIR="$NOTES_DIR/projects"
EDITOR="nvim"  # Change to your preferred editor

# Create directories if they don't exist
mkdir -p "$DAILY_DIR" "$PROJECTS_DIR"

# Function to open daily note
open_daily_note() {
    local today=$(date +%Y-%m-%d)
    local daily_file="$DAILY_DIR/$today.md"
    
    # Create file with header if it doesn't exist
    if [[ ! -f "$daily_file" ]]; then
        echo "# $(date '+%a %Y-%m-%d')" > "$daily_file"
        echo "" >> "$daily_file"
    fi
    
    $EDITOR "$daily_file"
}

# Function to open project note
open_project_note() {
    local project_name="$1"
    local project_file="$PROJECTS_DIR/${project_name}.md"
    
    # Create file with header if it doesn't exist
    if [[ ! -f "$project_file" ]]; then
        echo "# $project_name" > "$project_file"
        echo "" >> "$project_file"
    fi
    
    $EDITOR "$project_file"
}

# Function to list projects
list_projects() {
    echo "Project notes:"
    if [[ -d "$PROJECTS_DIR" ]]; then
        for file in "$PROJECTS_DIR"/*.md; do
            if [[ -f "$file" ]]; then
                basename "$file" .md
            fi
        done
    else
        echo "No project notes found."
    fi
}

# Function to list todos
list_todos() {
    echo "Open To-Do Items:"
    echo "=================="
    
    # Search for open todos in all markdown files
    find "$NOTES_DIR" -name "*.md" -type f | while read -r file; do
        local todos=$(grep -n "^- \[\] " "$file" 2>/dev/null)
        if [[ -n "$todos" ]]; then
            local relative_path=$(echo "$file" | sed "s|$NOTES_DIR/||")
            echo
            echo "From: $relative_path"
            echo "$todos" | while IFS=: read -r line_num todo; do
                echo "  Line $line_num: $(echo "$todo" | sed 's/^- \[\] //')"
            done
        fi
    done
}

# Main logic
case "$1" in
    --projects)
        list_projects
        ;;
    todos)
        list_todos
        ;;
    "")
        open_daily_note
        ;;
    *)
        open_project_note "$1"
        ;;
esac
