// Imports, for information on WalkDir, please check the Cargo.toml file.

use std::io;
use std::io::{Read, Write};
use std::path::Path;
use walkdir::WalkDir;

/** This function searches the directories, starting from the root_directory variable.
* It searches for a term, which has to be a full filename and not just a file extension (for example .py would not work, but filename.py would)
* Searched directories are counted and the number is displayed after the search process is over.
*/
fn main() {
    // Loop, so the program can run multiple times.
    loop {
        // Here we read the user input from the console and assign it to the search term variable. If the suer types 'exit' we exit the application.
        println!("Please Input the file you are searching for (!IMPORTANT! Filenames have to include the filetype (example: .java, .py) and are *not* case sensitive), if you wish to exit the application, simply type 'exit':");
        io::stdout().flush().unwrap();
        let mut search_term = String::new();
        io::stdin().read_line(&mut search_term).unwrap();
        let search_term = search_term.trim();
        if search_term.eq_ignore_ascii_case("exit") {
            break;
        }
        // We do the same thing for the root directory.
        print!("Enter your Root directory to start the search (format: C:/user/...): ");
        io::stdout().flush().unwrap();
        let mut root_directory = String::new();
        io::stdin().read_line(&mut root_directory).unwrap();
        let root_directory = root_directory.trim();

        // Helper variable to store the number of checked directories (not needed but fun).
        let mut checked_dirs = 0;
        // Helper List to store all of the
        let mut found_repos: Vec<String> = Vec::new();

        // WalkDir "walks" through the directories from the root specified. We iterate over each entry
        // to find the entry relevant to the search. Each subdirectory is checked (follow_links),
        // if you wish to disable the functionality, set .follow_links to false.
        for entry in WalkDir::new(root_directory)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            // We replace "\\" with a normal forward slash, since backslashes are considered escape keys in Rust.
            // This helps us make sure the algorithm is functioning as intended.
            let path_str = entry.path().to_string_lossy().replace("\\", "/");
            // At this point, a directory is already in the process of being so the counter is being updated here.
            checked_dirs += 1;
            // We search if our current path contains a .git file.
            if Path::new(&path_str).join(".git").exists() {
                // We use WalkDir again, to check all the subfolders of our entry path
                // The procedure is the same as before, we just use it again to search more extensively.
                for file_entry in WalkDir::new(entry.path())
                    .follow_links(true)
                    .into_iter()
                    .filter_map(Result::ok)
                {
                    // We create a variable filename and set its value to be the file entry in the current path
                    // Then the filename is compared to the search term. If they are the same it gets added to the found_repos list.
                    // We do this, so we can check that no directory is displayed / checked twice.
                    // Then the found path is printed to the console, and the loop gets closed.
                    if let Some(filename) = file_entry.file_name().to_str() {
                        if filename.to_lowercase() == search_term.to_lowercase() {
                            if !found_repos.contains(&path_str) {
                                found_repos.push(path_str.clone());
                                println!("{}", file_entry.path().display());
                            }
                            break;
                        }
                    }
                }
            }
        }
        // Finally, the amount of checked directories is printed (again, this is for fun only
        println!("Checked {checked_dirs} directories")
    }
}
