use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
mod Parser;


fn main(){
    //Get args and new instance:codes
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut codes = Parser::CodeReader::new(filename.to_string());

    //create out file
    let trimed: Vec<&str> = filename.split(".").collect();
    let of_name: String = trimed[0].to_string() + ".asm";
    let mut writer = BufWriter::new(File::create(of_name).unwrap());

    //check
    codes.reset_pos();
    while codes.hasMoreCommands(){
	let mut bin_line: u16 = 0;
	codes.advance();

	if codes.commandType()=="C_COMMAND"{
	    ()
	}

	else{
	    ()
	}

	println!("{}", codes.current_line);
	let outline:String = format!("{:0>16}", format!("{:b}", bin_line));
        writer.write(&*outline.as_bytes()).unwrap();
        writer.write(b"\n").unwrap();
	
    }
}

	
