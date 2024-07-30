// Imports, for information on Walk and tokio, please check the Cargo.toml file.
// This is Version 2 of the FilenameSearcher, it includes multithreading using tokio, to streamline processes. When searching large amounts of directories, the change is unnoticeable, but for smaller amounts, it is a lot faster

use std::io;
use std::io::Write;
use std::path::Path;
use std::time::Instant;
use tokio::{fs, task};

use ignore::Walk;

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
        if found_repos.is_empty() {
            println!(
                "The file you specified could not be found! Check if it lies in a git repository"
            )
        }
        for repo in found_repos {
            println!("{}", repo);
        }
        println!("Search completed in {:.2} seconds", duration.as_secs_f64());
    }
}

/** This function reads user input and prints an in the main defined predefined message to the terminal.
* Input: A string reference for the message to be printed.
* Output: The String the user has entered into the console.
*/
fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/** This function checks the directories, going out from the root.
* It checks if the directory contains a ".git" file (required to check if it's a repository)
* It checks if that repository contains the required file
    -> If the file is found, it gets added to a list, containing all the Paths that lead to the file (or files with the same name)
* A counter is updated each time a repository is checked (fun)
* Returns: A list containing all the directories, in which the filename was found and a counter
*/
async fn search_files(root_dir: &str, search_term: &str) -> (usize, Vec<String>) {
    let mut checked_dirs = 0;
    let mut found_repos = Vec::new();

    let entries = Walk::new(root_dir)
        .filter_map(|e| e.ok())
        .collect::<Vec<_>>();

    let tasks: Vec<_> = entries
        .into_iter()
        .map(|entry| {
            let search_term = search_term.to_string();
            let path_str = entry
                .path()
                .to_string_lossy()
                .replace("\\", "/")
                .to_string();
            task::spawn(async move {
                let mut local_checked_dirs = 0;
                let mut local_found_repos = Vec::new();

                local_checked_dirs += 1;

                if Path::new(&path_str).join(".git").exists() {
                    let file_entries = Walk::new(&path_str)
                        .filter_map(Result::ok)
                        .collect::<Vec<_>>();

                    for file_entry in file_entries {
                        if let Some(filename) = file_entry.file_name().to_str() {
                            if filename.to_lowercase() == search_term.to_lowercase() {
                                local_found_repos.push(file_entry.path().display().to_string());
                                break;
                            }
                        }
                    }
                }
                (local_checked_dirs, local_found_repos)
            })
        })
        .collect();

    for task in tasks {
        let (local_checked_dirs, local_found_repos) = task.await.unwrap();
        checked_dirs += local_checked_dirs;
        found_repos.extend(local_found_repos);
    }
    (checked_dirs, found_repos)
}
