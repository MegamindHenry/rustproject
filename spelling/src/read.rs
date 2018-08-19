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

#[cfg(test)]
mod tests {
	use std::collections::HashMap;
	use super::dicts;

    #[test]
    fn dicts_test() {
    	let mut test_map = HashMap::new();
    	test_map.insert("this".to_string(), 1);
    	test_map.insert("file".to_string(), 2);
    	test_map.insert("is".to_string(), 1);
    	test_map.insert("a".to_string(), 1);
    	test_map.insert("test".to_string(), 1);

        assert_eq!(dicts("test.txt").unwrap(), test_map);
    }
}