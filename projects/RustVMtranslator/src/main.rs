use std::env;
use std::fs;
mod Parser;
mod CodeWriter;



fn main(){
    //Get args
    let name: String;
    let n: usize;
    let list: Vec<String>;
    let result = GetArgs();
    name = result.0;
    n = result.1;
    list = result.2;
    println!("{} {} {}", name, n, list[0]);
    let of_name: String = name.clone() + ".asm";

    //init writer instance
    let mut code_writer = CodeWriter::CodeWriter::new(of_name);

    //sys.init dummy
    code_writer.VMinit();

    //init Parser list
    let mut codes_list = Vec::new();
    for i in 0..n{
	codes_list.push(Parser::CodeReader::new(list[i].clone()));
    }

    //write asm file. This clone() is ugly
    for i in 0..n{
	vm2asm(codes_list[i].clone(), &mut code_writer);
    }
    
}

fn vm2asm(mut codes: Parser::CodeReader, code_writer: &mut CodeWriter::CodeWriter) -> (){
    while (codes).hasMoreCommands(){
	codes.advance();
	if codes.commandType()=="C_PUSH"||codes.commandType()=="C_POP" {
	    code_writer.writePushPop(codes.commandType(), codes.arg1(), codes.arg2());
	}

	else if codes.commandType()=="C_ARITHMETIC"{
	    code_writer.writeArithmetic(codes.arg1());
	}

	else if codes.commandType()=="C_LABEL"{
	    code_writer.writeLabel(codes.arg1());
	}

	else if codes.commandType()=="C_GOTO"{
	    code_writer.writeGoto(codes.arg1());
	}

	else if codes.commandType()=="C_IF"{
	    code_writer.writeIf(codes.arg1());
	}

	else if codes.commandType()=="C_FUNCTION"{
	    code_writer.writeFunction(codes.arg1(), codes.arg2());
	}

	else if codes.commandType()=="C_RETURN"{
	    code_writer.writeReturn();
	}

	else if codes.commandType()=="C_CALL"{
	    code_writer.writeCall(codes.arg1(), codes.arg2());
	}
	else{
	    panic!("main not defined command")
	}
    }
}


//GetArg
//Xxx.vm ->   ___  1, Xxx.vm
//(Dir)Ddd -> Ddd, n, Xxx.vm, Zzz.vm, ...
fn GetArgs() -> (String, usize, Vec<String>){
    let mut name: String;
    let mut nargs: usize = 0;
    
    let args: Vec<String> = env::args().collect();
    if  args.len() > 2 {
	panic!("more than 1 args");
    }
    let input = args[1].trim().to_string();

    //if input is Xxx.vm
    if isVMfile(input.clone()) {
	let trimed: Vec<&str> = input.split(".").collect();
	return(trimed[0].to_string(), 1, vec![input]);
    }

    //else if input is Ddd(DIR)
    else{
	let mut vmlist= Vec::new();
	let paths = fs::read_dir(format!("./{}", input)).unwrap();
	for path in paths {
	    let path_string = path.unwrap().path().into_os_string().into_string().unwrap();
	    if isVMfile(path_string.clone()){
		vmlist.push(path_string);
		nargs += 1;
	    }
	}
	if nargs==0{
	    panic!("no .vm file");
	}
	
	return(input, nargs, vmlist);
    }
}

fn isVMfile(name: String) -> bool{
    let trimed: Vec<&str> = name.split(".").collect();
    let n = trimed.len();

    if trimed[n-1] == "vm".to_string(){
	true
    }
    else{
	false
    }
}

