// Course:      Efficient Linear Algebra and Machine Learning
// Assignment:  Project for English Word Check
// Author:      Xuefeng Luo
//
// Honor Code:  I pledge that this program represents my own work.
extern crate difference;
extern crate time;

use std::io::{stdin, stdout, Write};
use time::PreciseTime;

//read file and form dict
mod read;
use read::dicts;

//find out all possible words
mod candidate;

//grade the words and find out the correction
mod grade;
use grade::correction;

fn main() {
    loop {
        let s = menu();

        let start = PreciseTime::now();

        let filename = "big.txt";

        let dict = dicts(filename).unwrap();

        let correction = correction(s, dict);

        println!("Correction: {}", correction);

        let end = PreciseTime::now();

        println!("{} seconds", start.to(end));
    }
}

//print menu
//from: https://users.rust-lang.org/t/how-to-get-user-input/5176/2
fn menu() -> String {
    let mut s = String::new();
    print!("Word(Ctrl+C to exit): ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    let s = s.to_lowercase();

    s
}
