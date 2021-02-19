use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
mod Parser;
mod Code;


fn main(){
    //Get args and new instance:codes
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut codes = Parser::CodeReader::new(filename.to_string());

    //create out file
    let trimed: Vec<&str> = filename.split(".").collect();
    let of_name: String = trimed[0].to_string() + ".hack";
    let mut writer = BufWriter::new(File::create(of_name).unwrap());

    //check!
    while codes.hasMoreCommands(){
	let mut bin_line: u16 = 0;
	codes.advance();

	if codes.commandType()=="C_COMMAND"{
	    bin_line += 0b111 << 13;
	    bin_line += Code::comp(codes.comp()) << 6;
	    bin_line += Code::dest(codes.dest()) << 3;
	    bin_line += Code::jump(codes.jump());				 
	}

        else if codes.commandType()=="A_COMMAND"{
	    //it is not supported SYMBOL
	    bin_line += codes.symbol().parse::<u16>().unwrap();
	}

	else{
	    panic!("undefined COMMAND");
	}
	//debug
        println!("{}", codes.current_line);
	println!("{:0>16}", format!("{:b}", bin_line));
	//
			       
        let outline:String = format!("{:0>16}", format!("{:b}", bin_line));
	writer.write(&*outline.as_bytes()).unwrap();
	writer.write(b"\n").unwrap();
    }
	    
}
