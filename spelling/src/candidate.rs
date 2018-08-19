// Course:      Efficient Linear Algebra and Machine Learning
// Assignment:  Project for English Word Check
// Author:      Xuefeng Luo
//
// Honor Code:  I pledge that this program represents my own work.

//do the edit as dim as times
pub fn candidates(word: String, dim: u8) -> Vec<String>{
	// let mut words = Vec::new();
	let word_original = word.clone();

	//initialize the words
	let mut words = edit(word);

	//redo the edit function
	for _ in 1..dim {
		for x in words.clone().iter() {
			let mut words_new = edit(x.to_string());
			words.append(&mut words_new);
		}
	}

	words.push(word_original);

    //delete duplicates
    words.sort();

    words.dedup();

	words
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