use std::fs::File;
use std::io::Result;
use std::io::prelude::*;
use std::cmp::Ordering;

#[derive(Eq)]
struct DictWord {
    pub content: String,
    pub count: u32,
}

impl PartialOrd for DictWord {
    fn partial_cmp(&self, other: &DictWord) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DictWord {
    fn cmp(&self, other: &DictWord) -> Ordering {
        self.content.cmp(&other.content)
    }
}

impl PartialEq for DictWord {
    fn eq(&self, other: &DictWord) -> bool {
        self.content == other.content
    }
}

fn main() -> Result<()> {
    let mut contents = String::new();
    contents = read_file("foo.txt").unwrap();

    let dict = form_dict(contents);

    // println!("{:?}", dict[11].content);

    Ok(())
}

//read the file and return a String
fn read_file(filename: &str) -> Result<String> {
    let mut file = File::open(filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

//create a dictionary with a String
fn form_dict(contents: String) -> Vec<DictWord> {
    let contents = contents.to_lowercase();

    let iter = contents.split_whitespace();

    let mut dict: Vec<DictWord> = Vec::new();
    let mut count = 0;

    for content in iter {

    	// let find = dict.binary_search(&content);

    	count += 1;
    	println!("{:?}", count);
    	// println!("{:?}", content);

    	let mut find = find(&dict, &content);
    	// let find = -1;

    	// println!("{:?}", find);
    	// if find {println!("{:?}", &content)};

    	if find == -1 {
	        let word = DictWord {
	            content: content.to_string(),
	            count: 1u32,
	        };
	        dict.push(word);
	    }else {
	    	let location: usize = find as usize;
	    	// println!("{:?}", location);
	    	dict[location].count+=1;

	    	// let x: () = location;
	    }
    }
    // println!("{:?}", dict.len());

    // for x in dict.iter() {
    // 	if x.count > 1 {
	   //  	println!("{:?}   {:?}", x.content, x.count);
	   //  }
    // }

    // println!("{:?}", 1111);

    dict
}

fn find(ref dict: &Vec<DictWord>, target: &str) -> i32 {
	let mut location = 0;

	for x in dict.iter() {
		if x.content == target {
			return location;
		}
		location+=1;
	}

	-1
}
