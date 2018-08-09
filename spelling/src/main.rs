use std::fs::File;
use std::io::Result;
use std::io::prelude::*;
// use std::cmp::Ordering;
use std::collections::HashMap;


fn main() -> Result<()> {

	let filename = "foo1.txt";

	let dict = dicts(filename).unwrap();

	// println!("{:?}", dict);

    Ok(())
}

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
