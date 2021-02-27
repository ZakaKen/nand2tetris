use std::env;
use std::fs;
mod Parser;
mod CodeWriter;



fn main(){
    //Get args
    let result = GetArgs();
    let name: String = result.0; //dir or file path
    let n: usize = result.1; //num of .vm files
    let list: Vec<String> = result.2; //list of path to .vm files (Ex [dir/Xxx.vm, dir/Yyy.vm])
    println!("{} {} {}", name, n, list[0]);

    

    //init writer instance
    let mut code_writer = CodeWriter::CodeWriter::new(name + ".asm");

    //sys.init
    code_writer.VMinit(&list);

    //init Parser list = [parser[Aaa.vm], parser[Bbb.vm] ...]
    let mut codes_list = Vec::new();
    for i in 0..n{
	codes_list.push(Parser::CodeReader::new(&list[i]));
    }

    //write asm file.
    for i in 0..n{
	code_writer.setFileName(&list[i]);
	vm2asm(&mut codes_list[i], &mut code_writer);
    }
    
}

fn vm2asm(codes: &mut Parser::CodeReader, code_writer: &mut CodeWriter::CodeWriter) -> (){
    while (codes).hasMoreCommands(){
	codes.advance();
	match &codes.commandType()[..]{
	    "C_PUSH"|"C_POP" => {
		code_writer.writePushPop(codes.commandType(), codes.arg1(), codes.arg2());
	    }

	    "C_ARITHMETIC" => {
		code_writer.writeArithmetic(codes.arg1());
	    }

	    "C_LABEL" => {
		code_writer.writeLabel(format!("{}${}", codes.current_func, codes.arg1()));
	    }

	    "C_GOTO" => {
		code_writer.writeGoto(format!("{}${}", codes.current_func, codes.arg1()));
	    }

	    "C_IF" =>{
		code_writer.writeIf(format!("{}${}", codes.current_func, codes.arg1()));
	    }

	    "C_FUNCTION" => {
		//renew current func
		codes.current_func = codes.arg1().clone();
		//write
		code_writer.writeFunction(codes.arg1(), codes.arg2());
	    }
	    
	    "C_RETURN" => {
		code_writer.writeReturn();
	    }

	    "C_CALL" => {
		code_writer.writeCall(codes.arg1(), codes.arg2());
	    }
	    _ => {
		panic!("main not defined command")
	    }
	}
    }
}


//GetArg
//Xxx.vm ->   ___  1, Xxx.vm
//(Dir)Ddd -> Ddd, n, Xxx.vm, Zzz.vm, ...
fn GetArgs() -> (String, usize, Vec<String>){
    let mut nargs: usize = 0;

    //get arg. arg is Xxx.vm or Xxx(Dir).
    let args: Vec<String> = env::args().collect();
    if  args.len() > 2 {
	panic!("more than 1 args");
    }
    let arg = args[1].trim().to_string();
    

    //if arg is Xxx.vm
    if isVMfile(&arg) {
	let trimed: Vec<&str> = arg.split(".").collect();
	//println!("{}", &trimed[0]);
	return(trimed[0].to_string(), 1, vec![arg]);
    }

    //else if arg is Ddd(DIR)
    else{
	let mut vmlist= Vec::new();
	let paths = fs::read_dir(format!("./{}", arg)).unwrap();
	//push .vm files to vmlist. nargs = length of vmlist.
	for path in paths {
	    let path_string = path.unwrap().path().into_os_string().into_string().unwrap();
	    if isVMfile(&path_string){
		vmlist.push(path_string);
		nargs += 1;
	    }
	}

	//if there are no .vm file in Dir, Error
	if nargs==0{
	    panic!("no .vm file");
	}

	//generate where Dir, Dir/ -> Dir/Dir
	let dir_name: String;
	if arg.contains("/") {
	    let trimed: Vec<&str> = arg.split("/").collect();
	    dir_name = trimed[0].to_string();
	}
	else {
	    dir_name = arg;
	}

	//dir_name = Dir
	let out_file_path = dir_name.clone() + "/" + &dir_name.clone();
	return(out_file_path, nargs, vmlist);
    }
}

fn isVMfile(name: &String) -> bool{
    let trimed: Vec<&str> = name.split(".").collect();
    let n = trimed.len();

    if trimed[n-1] == "vm".to_string(){
	true
    }
    else{
	false
    }
}

