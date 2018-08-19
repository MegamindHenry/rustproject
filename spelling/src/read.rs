// Course:      Efficient Linear Algebra and Machine Learning
// Assignment:  Project for English Word Check
// Author:      Xuefeng Luo
//
// Honor Code:  I pledge that this program represents my own work.

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

//read from big txt and generate a hashmap with all words
pub fn dicts(filename: &str) -> Result<HashMap<String, u32>> {
    let mut file = File::open(filename)?;

    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let text = text.to_lowercase();

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let word = word.to_string();
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    Ok(map)
}