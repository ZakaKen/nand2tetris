use std::fs::File;
use std::io::{BufRead, BufReader};

const KEYWORD_LIST:[&str; 22] = ["class", "constructor", "function", "method", "field",
				 "static", "var", "int", "char", "boolean", "void",
				 "true", "false", "null", "this", "let", "do", "if",
				 "else", "while", "return", "this"];
const SYMBOL_LIST:[&str; 19] = ["{", "}", "(", ")", "[", "]", ".", ",", ";", "+",
				  "-", "*", "/", "&", "|", "<", ">", "=", "~"];
    

#[derive(Clone)]
pub struct JackTokenizer {
    pub tokens: Vec<String>,
    pub current_token: String,
    pub n: usize,
    pub l: usize,
}

impl JackTokenizer {

    
    pub fn new(name: &String) -> JackTokenizer{
	//open file and prepare
	let f_name = &name;
	let f = File::open(f_name).expect("file not found");
	let reader = BufReader::new(f);
	let codes : Vec<String> = do_comment_out(reader.lines());
	let tokens = tokenize(codes.clone());

	//instance init
	JackTokenizer{
	    tokens: tokens.clone(),
	    current_token: "".to_string(),
	    n: tokens.len(),
	    l: 0,
	}
    }

    pub fn hasMoreTokens(&self) -> bool{
	self.l < self.n 
    }

    pub fn advance(&mut self) -> (){
	if self.hasMoreTokens(){
	    self.current_token = self.tokens[self.l].clone();
	    self.l += 1;
	}
	else{
	    //panic!("JackTokenizer:: no more tokens")
	}
    }

    pub fn tokenType(&self) -> String{
	if KEYWORD_LIST.iter().find(|&&x| x == &self.current_token).is_some(){
	    "KEYWORD".to_string()
	}
	else if SYMBOL_LIST.iter().find(|&&x| x == &self.current_token).is_some(){
	    "SYMBOL".to_string()
	}
	else if is_int_const(&self.current_token){
	    "INT_CONST".to_string()
	}
	else if is_string_const(&self.current_token){
	    "STRING_CONST".to_string()
	}
	else{
	    //it should be panic if not identified IDENTIFER
	    "IDENTIFIER".to_string()
	}
    }

    pub fn keyWord(&self) -> String{
	match &self.current_token[..]{
	    "class" => {"CLASS".to_string()}
	    "method" => {"METHOD".to_string()}
	    "function" => {"FUNCTION".to_string()}
	    "constructor" => {"CONSTRUCTOR".to_string()}
	    "int" => {"INT".to_string()}
	    "boolean" => {"BOOLEAN".to_string()}
	    "char" => {"CHAR".to_string()}
	    "void" => {"VOID".to_string()}
	    "var" => {"VAR".to_string()}
	    "static" => {"STATIC".to_string()}
	    "field" => {"FIELD".to_string()}
	    "let" => {"LET".to_string()}
	    "do" => {"DO".to_string()}
	    "if" => {"IF".to_string()}
	    "else" => {"ELSE".to_string()}
	    "while" => {"WHILE".to_string()}
	    "return" => {"RETURN".to_string()}
	    "true" => {"TRUE".to_string()}
	    "false" => {"FALSE".to_string()}
	    "null" => {"NULL".to_string()}
	    "this" => {"THIS".to_string()}
	    _ => panic!("JackTokenizer:: undefined keyword")
	}
    }

    pub fn symbol(&self) -> String{
	self.current_token.clone()
    }

    pub fn identifier(&self) -> String{
	self.current_token.clone()
    }

    pub fn intVal(&self) -> u16{
	self.current_token.parse::<u16>().unwrap() 
    }

    pub fn stringVal(&self) -> String{
	let begin = self.current_token.char_indices().nth(1).unwrap().0;
	let end = self.current_token.char_indices().last().unwrap().0;
	let val = &self.current_token[begin..end];
	val.to_string()
    }
}

pub fn do_comment_out(lines: std::io::Lines<BufReader<File>>) -> Vec<String>{
    let mut codes : Vec<String> = Vec::new();
    let mut comment_out_flag : bool = false;
    for line in lines{
	let mut line = line.unwrap().to_string();

	/*comment out processing*/

	//trim /*xxx*/
	// not after '/*'
	if !comment_out_flag {
	    //if '/* xxx */ in the line'
	    if line.contains("/*")&&line.contains("*/"){
		comment_out_flag = false;
		let trimed: Vec<&str> = line.split("/*").collect();
		let trimed2:Vec<&str> = line.split("*/").collect();
		let header: String = trimed[0].trim().to_string();
		let tail:String = trimed2[1].trim().to_string();
		line = header + &tail;
	    }

	    // '/*'  start
	    if line.contains("/*"){
		comment_out_flag = true;
		let trimed: Vec<&str> = line.split("/*").collect();
		line = trimed[0].trim().to_string();
	    }

	}
	//after '/*'
	else{
	    if line.contains("*/"){
		comment_out_flag = false;
		let trimed: Vec<&str> = line.split("*/").collect();
		line = trimed[1].trim().to_string();
	    }
	    else{
		line = "".to_string();
	    }
	}
	
	//trim "//" and space
	if line.contains("//") {
	    let trimed: Vec<&str> = line.split("//").collect();
	    line=trimed[0].trim().to_string();
	}
	
	//ignore vacant line and push
	if line.len() > 0 {
	    codes.push(line.trim().to_string());
	}
    }

    return codes
}

pub fn tokenize(lines: Vec::<String>) -> Vec<String>{
    let mut tokens = Vec::<String>::new();

    for line in lines{
	tokens.extend_from_slice(&line_to_tokens(&line));
    }

    tokens
}

pub fn split_symbol(word: &str) -> Vec<String>{
    let mut buf_string: String = "".to_string();
    let mut words_splited_symb = Vec::<String>::new();

    //read by 1 character
    for c in word.chars(){
	//if SYMBOL_LIST.contains(c){
	if  SYMBOL_LIST.iter().find(|&&x| x == &c.to_string()).is_some(){
	    //push token
	    if buf_string.len() > 0{
		words_splited_symb.push(buf_string.clone());
	    }
	    //push symbol to tokens
	    words_splited_symb.push(c.to_string());
	    //reset buf
	    buf_string = "".to_string();
	}
	//if c is not symbol, push c to buf strings
	else {
	    buf_string.push(c);
	}
    }
    //push token
    if buf_string.len() > 0{
	words_splited_symb.push(buf_string.clone());
    }

    //return
    words_splited_symb
}

pub fn line_to_tokens(line: &str) -> Vec<String>{
    let mut buf_string: String = "".to_string();
    let mut splited_line = Vec::<String>::new();
    let mut is_string: bool = false;

    //read by 1 character
    for c in line.chars(){
	//if whitespace
	if c.to_string() == " ".to_string() && !is_string{
	    //push token
	    if buf_string.len() > 0{
		splited_line.push(buf_string.clone());
	    }
	    //reset buf
	    buf_string = "".to_string();
	}

	
	//if SYMBOL_LIST.contains(c){
	else if  SYMBOL_LIST.iter().find(|&&x| x == &c.to_string()).is_some() && !is_string{
	    //push token
	    if buf_string.len() > 0{
		splited_line.push(buf_string.clone().trim().to_string());
	    }
	    //push symbol to tokens
	    splited_line.push(c.to_string());
	    //reset buf
	    buf_string = "".to_string();
	}

	//if string start
	else if c.to_string() == '"'.to_string() && !is_string{
	    //push token
	    if buf_string.len() > 0{
		splited_line.push(buf_string.clone().trim().to_string());
	    }
	    //reset buf
	    buf_string = "".to_string();
	    //push symbol to buf_string
	    buf_string.push(c);

	    is_string = true;
	}

	//if string end
	else if c.to_string() == '"'.to_string() && is_string{
	    //push symbol to buf_string
	    buf_string.push(c);
	    //push token
	    if buf_string.len() > 0{
		splited_line.push(buf_string.clone().trim().to_string());
	    }
	    //reset buf
	    buf_string = "".to_string();

	    is_string = false;
	}
	
	//push c to buf strings
	else {
	    buf_string.push(c);
	}
    }
    
    //push token
    if buf_string.len() > 0{
	splited_line.push(buf_string.clone());
    }

    //return
    splited_line
}

pub fn is_int_const(token: &String) -> bool {
    match token.parse::<u16>() {
	Ok(_n) => {true}
	Err(_err) => {false}
    }
}

pub fn is_string_const(token: &String) -> bool{
    if (*token).chars().nth(0).unwrap()=='"' && (*token).chars().last().unwrap()=='"'{
	true
    }
    else{
	false
    }
}
