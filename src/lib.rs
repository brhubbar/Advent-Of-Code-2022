// Common functions for advent of code 2022.
use std::fs;


/// Read the contents of a file directly into a String.
///
/// Parameters
/// ----------
/// file_path : &str
///     Path to read from.
///
pub fn read_file(file_path: &str) -> String {
    // source: https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}
