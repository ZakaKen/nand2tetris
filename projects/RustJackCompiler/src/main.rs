use std::env;
use std::fs;
mod JackTokenizer;
mod CompilationEngine;
mod SymbolTable;
mod VMWriter;



fn main(){
    //Get args
    let result = GetArgs();
    let n: usize = result.1; //num of .jack files
    let list: Vec<String> = result.2; //list of path to .jack files (Ex [dir/Xxx.jack, dir/Yyy.jack])

    //init writer instance

    //init JackTokenizer list = [tokenizer[Aaa.jack], tokenizer[Bbb.jack] ...]
    let mut tokens_list = Vec::new();
    let mut comp_engine_list = Vec::new();
    for i in 0..n{
	tokens_list.push(JackTokenizer::JackTokenizer::new(&list[i]));
	let out_file_path = list[i].replace(".jack", ".vm");
	comp_engine_list.push(CompilationEngine::CompilationEngine::new(tokens_list[i].clone(), out_file_path));
    }

    //write xml file.
    for i in 0..n{
	comp_engine_list[i].startCompile();
    }
}


//GetArg
//Xxx.jack ->   ___  1, Xxx.jack
//(Dir)Ddd -> Ddd, n, Xxx.jack, Zzz.jack, ...
fn GetArgs() -> (String, usize, Vec<String>){
    let mut nargs: usize = 0;

    //get arg. arg is Xxx.jack or Xxx(Dir).
    let args: Vec<String> = env::args().collect();
    if  args.len() > 2 {
	panic!("more than 1 args");
    }
    let arg = args[1].trim().to_string();
    

    //if arg is Xxx.jack
    if isJACKfile(&arg) {
	let trimed: Vec<&str> = arg.split(".").collect();
	//println!("{}", &trimed[0]);
	return(trimed[0].to_string(), 1, vec![arg]);
    }

    //else if arg is Ddd(DIR)
    else{
	let mut jacklist= Vec::new();
	let paths = fs::read_dir(format!("./{}", arg)).unwrap();
	//push .jack files to jacklist. nargs = length of jacklist.
	for path in paths {
	    let path_string = path.unwrap().path().into_os_string().into_string().unwrap();
	    if isJACKfile(&path_string){
		jacklist.push(path_string);
		nargs += 1;
	    }
	}

	//if there are no .jack file in Dir, Error
	if nargs==0{
	    panic!("no .jack file");
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
	return(out_file_path, nargs, jacklist);
    }
}

fn isJACKfile(name: &String) -> bool{
    let trimed: Vec<&str> = name.split(".").collect();
    let n = trimed.len();

    if trimed[n-1] == "jack".to_string(){
	true
    }
    else{
	false
    }
}

