use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;

fn main(){
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file {}", filename);

    let f = File::open(filename).expect("file not found");
    let reader = BufReader::new(f);
    let mut codes : Vec<String> =Vec::new();

    /*
    let mut counter = 0;
    reader.lines().for_each(|_| counter += 1);
    println!("{}", counter);
    */
    
    //read and push datas to vector
    for line in reader.lines(){
	let line = line.unwrap();

	//if vacant line
	if line.len() >0 {
	    if line.contains("//") {
		println!("{}", "comment out !!");
	    }
	    codes.push(line);
	}
    }

    //check!
    for line in codes.iter(){
	println!("{}", line);
    }

    //check2
    let n = codes.len();
    println!("{}", n);
    println!("{}", codes[1]);
}
