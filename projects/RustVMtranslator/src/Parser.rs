use std::fs::File;
use std::io::{BufRead, BufReader};

const ARITHMETIC_LIST: &str = "add,sub,neg,eq,gt,lt,and,or,not";

pub struct CodeReader {
    pub lines: Vec<String>,
    pub current_command: String,
    pub current_arg1: String,
    pub current_arg2: String,
    pub current_func: String,
    pub n: usize,
    pub l: usize,
}

impl CodeReader{

    
    pub fn new(name: &String) -> CodeReader{
	//open file and prepare
	let f_name = &name;
	let f = File::open(f_name).expect("file not found");
	let reader = BufReader::new(f);
	let mut codes : Vec<String> =Vec::new();

	//read file > trime space and // >push line to codes.
	for line in reader.lines(){
	    let mut line = line.unwrap().trim().to_string();
	
	    //trim "//" and space
	    if line.contains("//") {
		let trimed: Vec<&str> = line.split("//").collect();
		line=trimed[0].trim().to_string();
	    }
	    //ignore vacant line
	    if line.len() > 0 {
		codes.push(line);
	    }
	    
	    
	}

	//instance init
	CodeReader{
	    lines: codes.clone(),
	    current_command: "".to_string(),
	    current_arg1: "".to_string(),
	    current_arg2: "".to_string(),
	    current_func: "null".to_string(),
	    n: codes.len(),
	    l: 0,
	}
    }

    pub fn hasMoreCommands(&self) -> bool{
	self.l < self.n
    }

    pub fn advance(&mut self) -> (){
	let trimed: Vec<&str> = self.lines[self.l].split(" ").collect();
	let args_len: usize = trimed.len();
	match args_len {
	    1 => {self.current_command=trimed[0].to_string();
     		  self.current_arg1="".to_string();
		  self.current_arg2="".to_string();}
	    
	    2 => {self.current_command=trimed[0].to_string();
       		  self.current_arg1=trimed[1].to_string();
		  self.current_arg2="".to_string();}
	    
	    3 => {self.current_command=trimed[0].to_string();
       		  self.current_arg1=trimed[1].to_string();
		  self.current_arg2=trimed[2].to_string();}
	    
	    _ => {panic!("more than 3 argments")}
	}
	self.l += 1;
    }

    pub fn commandType(&self) -> String{
	if ARITHMETIC_LIST.contains(&self.current_command){
	    "C_ARITHMETIC".to_string()
	}
	else if self.current_command == "push".to_string(){
	    "C_PUSH".to_string()
	}
	else if self.current_command == "pop".to_string(){
	    "C_POP".to_string()
	}
	else if self.current_command == "label".to_string(){
	    "C_LABEL".to_string()
	}
	else if self.current_command == "goto".to_string(){
	    "C_GOTO".to_string()
	}
	else if self.current_command == "if-goto".to_string(){
	    "C_IF".to_string()
	}
	else if self.current_command == "function".to_string(){
	    "C_FUNCTION".to_string()
	}
	else if self.current_command == "return".to_string(){
	    "C_RETURN".to_string()
	}
	else if self.current_command == "call".to_string(){
	    "C_CALL".to_string()
	}
	else{
	    //"commandType non".to_string()
	    panic!("Parser commandType Error");
	}
    }

    pub fn arg1(&self) -> String{
	if self.commandType() == "C_ARITHMETIC".to_string(){
	    self.current_command.clone()
	}
	else{
	    self.current_arg1.clone()
	}
    }

    pub fn arg2(&self) -> u16{
	self.current_arg2.parse().unwrap()
    }

    pub fn reset_pos(&mut self) -> (){
	self.l = 0;
    }

    

}

