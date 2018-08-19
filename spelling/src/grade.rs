// Course:      Efficient Linear Algebra and Machine Learning
// Assignment:  Project for English Word Check
// Author:      Xuefeng Luo
//
// Honor Code:  I pledge that this program represents my own work.
use candidate::candidates;
use difference::diff;
use difference::Difference;
use std::collections::HashMap;

//find out he most possible word for correction
pub fn correction(word: String, dicts: HashMap<String, u32>) -> String {
    //get all candidates
    let words = candidates(word.clone(), 2);

    //find out the most common words
    let candidates_grade = grade(words, dicts, word.clone());

    let word_max = top_grade(candidates_grade);

    word_max
}

//grade system for the correction
fn grade(
    words: Vec<String>,
    dicts: HashMap<String, u32>,
    word_original: String,
) -> HashMap<String, u32> {
    let mut candidates_grade = HashMap::new();

    for x in words.iter() {
        let mut grade: u32;

        let y = dicts.get(&x.to_string());
        if y != None {
            let z = y.unwrap();
            grade = *z;

            if x.eq(&word_original) {
                grade += 2000000;
            }
        } else {
            grade = 0;

            if x.eq(&word_original) {
                grade = 1;
            }
        }

        if grade > 0 {
            let word = x.clone();

            grade += grade_diff(x.clone(), word_original.clone());
            candidates_grade.insert(word, grade);
        }
    }

    println!("{:?}", candidates_grade);

    candidates_grade
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

//grade the words depends on the difference of words
fn grade_diff(word1: String, word2: String) -> u32 {
    let mut grade = 0;
    let mut last_diff = Difference::Same("".to_string());
    let (dist, changeset) = diff(&word1, &word2, "");
    if dist > 0 {
        grade = (4 - dist as u32) * 1000;
    }

    for x in changeset.iter() {
        match x {
            Difference::Same(s) => {
                last_diff = Difference::Same(s.to_string());
            }
            Difference::Add(s) => {
                if last_diff.eq(&Difference::Rem("e".to_string())) && s.eq("a") {
                    grade += 5000;
                }

                if last_diff.eq(&Difference::Rem("a".to_string())) && s.eq("e") {
                    grade += 5000;
                }

                if last_diff.eq(&Difference::Rem("r".to_string())) && s.eq("o") {
                    grade += 1000;
                }

                if last_diff.eq(&Difference::Rem("o".to_string())) && s.eq("r") {
                    grade += 1000;
                }

                if last_diff.eq(&Difference::Rem("y".to_string())) && s.eq("z") {
                    grade += 1000;
                }

                if last_diff.eq(&Difference::Rem("z".to_string())) && s.eq("y") {
                    grade += 1000;
                }

                if s.eq("a") {
                    grade += 1000;
                }
                if s.eq("e") {
                    grade += 1000;
                }
                if s.eq("i") {
                    grade += 1000;
                }
                if s.eq("o") {
                    grade += 1000;
                }
                if s.eq("u") {
                    grade += 1000;
                }

                last_diff = Difference::Add(s.to_string());
            }
            Difference::Rem(s) => {
                if last_diff.eq(&Difference::Add("e".to_string())) && s.eq("a") {
                    grade += 5000;
                }

                if last_diff.eq(&Difference::Add("a".to_string())) && s.eq("e") {
                    grade += 5000;
                }

                if last_diff.eq(&Difference::Add("r".to_string())) && s.eq("o") {
                    grade += 1000;
                }

                if last_diff.eq(&Difference::Add("o".to_string())) && s.eq("r") {
                    grade += 1000;
                }

                if last_diff.eq(&Difference::Add("y".to_string())) && s.eq("z") {
                    grade += 1000;
                }

                if last_diff.eq(&Difference::Add("z".to_string())) && s.eq("y") {
                    grade += 1000;
                }

                if s.eq("a") {
                    grade += 1000;
                }
                if s.eq("e") {
                    grade += 1000;
                }
                if s.eq("i") {
                    grade += 1000;
                }
                if s.eq("o") {
                    grade += 1000;
                }
                if s.eq("u") {
                    grade += 1000;
                }

                last_diff = Difference::Rem(s.to_string());
            }
        }
    }

    grade
}

#[cfg(test)]
mod tests {
    use super::{correction, grade, grade_diff, top_grade};
    use std::collections::HashMap;

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
        test_grade.insert("this".to_string(), 3010);
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

    #[test]
    fn grade_diff_test() {
        assert_eq!(grade_diff("a".to_string(), "e".to_string()), 9000);
    }
}
