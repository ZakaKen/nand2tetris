use std::env;
mod Parser;
mod CodeWriter;


fn main(){
    //Get args and new instance:codes
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut codes = Parser::CodeReader::new(filename.to_string());

    //init code_writer
    let trimed: Vec<&str> = filename.split(".").collect();
    let of_name: String = trimed[0].to_string() + ".asm";
    let mut code_writer = CodeWriter::CodeWriter::new(of_name);

    //VMinit
    code_writer.VMinit();


    //check
    codes.reset_pos();
    while codes.hasMoreCommands(){
	codes.advance();
	if codes.commandType()=="C_PUSH"||codes.commandType()=="C_POP" {
	    code_writer.writePushPop(codes.commandType(), codes.arg1(), codes.arg2());
	}

	else if codes.commandType()=="C_ARITHMETIC"{
	    code_writer.writeArithmetic(codes.arg1());
	}

	else{
	    panic!("main not defined command")
	}
	
    }
}

	
