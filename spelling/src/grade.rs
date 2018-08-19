// Course:      Efficient Linear Algebra and Machine Learning
// Assignment:  Project for English Word Check
// Author:      Xuefeng Luo
//
// Honor Code:  I pledge that this program represents my own work.
use std::collections::HashMap;
use candidate::candidates;


//find out he most possible word for correction
pub fn correction(word: String, dicts: HashMap<String, u32>) -> String  {
	//get all candidates
	let words = candidates(word.clone(), 2);

	//find out the most common words
    let candidates_grade = grade(words, dicts, word.clone());

    let word_max = top_grade(candidates_grade);

	word_max
}


//grade system for the correction
fn grade(words: Vec<String>, dicts: HashMap<String, u32>, word_original: String) -> HashMap<String, u32> {
    let mut candidates_grade = HashMap::new();

    for x in words.iter() {
        let mut grade: u32;

        let y = dicts.get(&x.to_string());
        if y != None{
            let z = y.unwrap();
            grade = *z;

            if x.eq(&word_original) {
                grade += 2000;
            }
        } else {
            grade = 0;

            if x.eq(&word_original) {
            	grade = 1;
            }
        }

        let word = x.clone();

        if grade > 0 {
            candidates_grade.insert(word, grade);
        }
    }

    return candidates_grade;
}

//find out the top grade word
fn top_grade(candidates_grade: HashMap<String, u32>) -> String {
    let mut max = 0;
    let mut max_string = String::new();

    for (x, y) in &candidates_grade {
        if *y > max {
            max = *y;
            max_string = x.clone();
        }
    }

    max_string
}

#[cfg(test)]
mod tests {
	use std::collections::HashMap;
	use super::{correction, grade, top_grade};

    #[test]
    fn correction_test() {
    	let mut test_dicts = HashMap::new();
    	test_dicts.insert("this".to_string(), 10);
    	test_dicts.insert("file".to_string(), 2);
    	test_dicts.insert("is".to_string(), 1);
    	test_dicts.insert("a".to_string(), 1);
    	test_dicts.insert("test".to_string(), 1);

        assert_eq!(correction("thi".to_string(), test_dicts), "this");
    }

    #[test]
    fn grade_test() {
    	let mut test_dicts = HashMap::new();
    	test_dicts.insert("this".to_string(), 10);
    	test_dicts.insert("file".to_string(), 2);
    	test_dicts.insert("is".to_string(), 1);
    	test_dicts.insert("a".to_string(), 1);
    	test_dicts.insert("test".to_string(), 1);

    	let mut test_words = Vec::new();
    	test_words.push("thi".to_string());
    	test_words.push("thb".to_string());
    	test_words.push("tha".to_string());
    	test_words.push("this".to_string());

    	let word_original = "thi".to_string();

    	let mut test_grade = HashMap::new();
    	test_grade.insert("this".to_string(), 10);
    	test_grade.insert("thi".to_string(), 1);

    	assert_eq!(grade(test_words, test_dicts, word_original), test_grade);
    }

    #[test]
    fn top_grade_test() {
    	let mut test_grade = HashMap::new();
    	test_grade.insert("this".to_string(), 10);
    	test_grade.insert("thi".to_string(), 1);

    	assert_eq!(top_grade(test_grade), "this");
    }
}