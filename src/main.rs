use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
use std::collections::HashMap;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() == 1 {
		println!("Format: ./hcomp <filename>");
	}

	let filepath = &args[1];

    let file = File::open(filepath).unwrap();

    // Create a BufReader
    let reader = BufReader::new(file);

    let mut char_freqs: HashMap<char, usize> = HashMap::new();

    // Read the file line by line
    for line in reader.lines() {
        for c in line.expect("lines failed").chars() {
            //println!("Character: {}", c);
            char_freqs.entry(c)
            	.and_modify(|e| *e += 1)
            	.or_insert(1);
        }
    }
    println!("{:?}", char_freqs);
}
