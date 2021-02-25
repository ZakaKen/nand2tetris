use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
mod Parser;
mod Code;
mod SymbolTable;


fn main(){
    //Get args and new instance:codes
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut codes = Parser::CodeReader::new(filename.to_string());

    //create out file
    let trimed: Vec<&str> = filename.split(".").collect();
    let of_name: String = trimed[0].to_string() + ".hack";
    let mut writer = BufWriter::new(File::create(of_name).unwrap());

    //init symbol talbe
    let mut symbol_table = SymbolTable::SymbolTable::new();

    /*First Pass*/
    //Add Labels to Symbol Table
    let mut ROMaddr: u16 = 0;
    while codes.hasMoreCommands(){
	codes.advance();
	if codes.commandType()=="C_COMMAND"{
	    ROMaddr += 1;
	}

	else if codes.commandType()=="A_COMMAND"{
	    ROMaddr += 1;
	}

	else if codes.commandType()=="L_COMMAND"{
	    if !symbol_table.contains(codes.symbol()){
		symbol_table.addEntry(codes.symbol(), ROMaddr);
	    }
	    else{
		panic!("This symbol has already defined");
	    }
	}

	else{
	    panic!("First Pass codes error")
	}
    }

    /*Second Pass*/
    //Code Creation
    codes.reset_pos();
    let mut RAMaddr: u16 = 16;
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
	    //matsh @numbers or @symbol
	    match codes.symbol().parse::<u16>(){
		//@(numbers)
		Ok(n) => {
		    bin_line += codes.symbol().parse::<u16>().unwrap();
		}
		//@SYMBOLS
		Err(err) =>{
		    if symbol_table.contains(codes.symbol()){
			bin_line += symbol_table.getAddress(codes.symbol());
		    }
		    else {
			symbol_table.addEntry(codes.symbol(), RAMaddr);
			bin_line += RAMaddr;
			RAMaddr += 1;
		    }
		}
	    }
	}


	else if codes.commandType()=="L_COMMAND"{
	    ()
        }

	else{
	    panic!("First pass undefined COMMAND");
	}

        //output binary code
        if(codes.commandType()=="C_COMMAND"||codes.commandType()=="A_COMMAND"){ 
	    //debug
	    //println!("{}", codes.current_line);
	    println!("{:0>16}", format!("{:b}", bin_line));
	    //
			       
            let outline:String = format!("{:0>16}", format!("{:b}", bin_line));
	    writer.write(&*outline.as_bytes()).unwrap();
	    writer.write(b"\n").unwrap();
	}
    }
}

	
