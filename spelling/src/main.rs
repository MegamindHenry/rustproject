// Course:      Efficient Linear Algebra and Machine Learning
// Assignment:  Project for English Word Check
// Author:      Xuefeng Luo
//
// Honor Code:  I pledge that this program represents my own work.
extern crate time;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use time::PreciseTime;

fn main() -> Result<()> {
	let start = PreciseTime::now();


    let filename = "foo1.txt";

    let dict = dicts(filename).unwrap();

    // println!("{:?}", dict);

    let edit_word = candidates("fuc".to_string(), 3);

    // println!("{:?}", edit_word);


    let end = PreciseTime::now();

    println!("{} seconds", start.to(end));

    Ok(())
}

//read from big txt and generate a hashmap with all words
fn dicts(filename: &str) -> Result<HashMap<String, u32>> {
    let mut file = File::open(filename)?;

    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let word = word.to_string();
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    Ok(map)
}

fn edit(word: String) -> Vec<String> {
    let letters = "abcdefghijklmnopqrstuvwxyz";

    let mut words = Vec::new();

    let length = word.len();

    //loop for each position in word
    for x in 0..length + 1 {
        //clone first part and split second part
        let mut word_f = word.clone();
        let mut word_s = word_f.split_off(x);

        //add a new letter
        for y in letters.chars() {
            //clone new string
            let mut word_new = word_f.clone();
            let word_new_s = word_s.as_str();

            //add all parts
            word_new.push(y);
            word_new.push_str(word_new_s);

            //add to words
            words.push(word_new);
        }

        //delete
        //check if it is out of range
        if x < length {
            //clone
            let mut word_new = word.clone();

            //remove at x
            word_new.remove(x);

            //add to words
            words.push(word_new);
        }

        //replace
        //check if it is the frist emply string
        if x != 0 {
            for y in letters.chars() {
                //clone parts
                let mut word_new = word_f.clone();
                let word_new_s = word_s.as_str();

                //add parts
                word_new.pop();
                word_new.push(y);
                word_new.push_str(word_new_s);

                //add to words
                words.push(word_new);
            }
        }

        //exchange
        if x != 0 && x < length {
            //clone parts
            let mut word_new = word_f.clone();
            let mut word_new_s = word_s.clone();

            //get swap chars
            let first_char = word_new.pop().unwrap();
            let second_char = word_new_s.remove(0);

            //add parts
            word_new.push(second_char);
            word_new.push(first_char);
            word_new.push_str(word_new_s.as_str());

            //add to words
            words.push(word_new);
        }
    }

    words
}


//do the edit as dim as times
fn candidates(word: String, dim: u8) -> Vec<String>{
	// let mut words = Vec::new();

	//initialize the words
	let mut words = edit(word);

	//redo the edit function
	for _ in 1..dim {
		for x in words.clone().iter() {
			let mut words_new = edit(x.to_string());
			words.append(&mut words_new);
		}
	}

	words
}
