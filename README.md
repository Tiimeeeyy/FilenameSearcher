# FilenameSearcher

## This project is created using Rust. It is a searcher, that searches for a file in all directories, going from a root directory. It checks if the directory contains a .git file and then looks for the file needed.

# Dependencies
* If you want to compile the binaries yourself, Rust is required: https://www.rust-lang.org
* Pre compiled binaries is the filenameSearcher.exe file (Version 1) and filenameSearcherV2.exe (Version 2)
* These binaries *only* work on Windows machines! If you want to compile them to macOS / Linux, you need Rust installed and need to compile to that target. For more refer to: https://rust-lang.github.io/rustup/cross-compilation.html

Have fun with this little project :) Hope it is helpful

# Update 1:
I "upgraded" the code by using multithreading with the tokio.rs package (https://tokio.rs). The program is quite fast now, if you know the general area of directories where the files you are looking for are located. I added a timer to the app just for fun and to compare runtimes when searching (for optimization purposes). 

# Update 2:
I changed the Directory traversal from WalkDir to ignore::Walk (https://crates.io/crates/ignore). This yields far better results and is a lot faster (from almost 40 seconds down to 2). Multithreading was changed a bit, since now threads are being spawned for each directory to be checked. Looking at the time it takes to run the code, I don't think optimizing for overhead will yield any good results, but I might implement it in the future.
