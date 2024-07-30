// Imports, for information on WalkDir, please check the Cargo.toml file.

use std::io;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

use walkdir::WalkDir;

// Use of constants for IO Operations to save memory
const EXIT_COMMAND: &str = "exit";
const PROMPT_TERM: &str = "Please Input the file you are searching for (!IMPORTANT! Filenames have to include the filetype (example: .java, .py) and are *not* case sensitive), if you wish to exit the application, simply type 'exit'";
const PROMPT_DIR: &str = "Enter your Root directory to start the search (format: C:/user/...): ";

// Main function, just loops so the process does not end after one search is completed.
#[tokio::main]
async fn main() {
    loop {
        let search_term = read_input(PROMPT_TERM);
        if search_term.eq_ignore_ascii_case(EXIT_COMMAND) {
            break;
        }

        let root_dir = read_input(PROMPT_DIR);
        let start_time = Instant::now();
        let (checked_dirs, found_repos) = search_files(&root_dir, &search_term).await;
        let duration = start_time.elapsed();
        println!("Checked {} directories", checked_dirs);
        for repo in found_repos {
            println!("{}", repo);
        }
        println!("Search completed in {:.2} seconds", duration.as_secs_f64());
    }
}

// This function reads user input and prints an in the main defined predefined message to the terminal.

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

async fn search_files(root_dir: &str, search_term: &str) -> (usize, Vec<String>) {
    let mut checked_dirs = 0;
    let mut found_repos = Vec::new();
    
    let mut entries = WalkDir::new(root_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok());
    
    while let Some(entry) = entries.next() {
        let path_str = entry.path().to_string_lossy().replace("\\", "/");
        checked_dirs += 1;
        
        if Path::new(&path_str).join(".git").exists() {
            let mut file_entries = WalkDir::new(entry.path())
                .follow_links(true)
                .into_iter()
                .filter_map(Result::ok);
            
            while let Some(file_entry) = file_entries.next() {
                if let Some(filename) = file_entry.file_name().to_str() {
                    if filename.to_lowercase() == search_term.to_lowercase() {
                        if !found_repos.contains(&path_str) {
                            found_repos.push(file_entry.path().display().to_string())
                        }
                        break;
                    }
                }
            }
        }
    }
    (checked_dirs, found_repos)
}
