use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct CodeReader {
    pub filename: String,
    pub lines: Vec<String>,
    pub current_line: String,
    pub n: usize,
    pub l: usize,
}

impl CodeReader{
    pub fn new(name: String) -> CodeReader{
	//prepare
	let f_name = &name;
	let f = File::open(f_name).expect("file not found");
	let reader = BufReader::new(f);
	let mut codes : Vec<String> =Vec::new();

	//read + processing + push datas to vector
	for line in reader.lines(){
	    let mut line = line.unwrap().trim().to_string();
	
	    //trim "//" 
	    if line.contains("//") {
		let trimed: Vec<&str> = line.split("//").collect();
		line=trimed[0].to_string();
	    }
	    //ignore vacant line
	    if line.len() > 0 {
		codes.push(line);
	    }
	}

	//instance init
	CodeReader{
	    filename: f_name.to_string(),
	    lines: codes.clone(),
	    current_line: "".to_string(),
	    n: codes.len(),
	    l: 0,
	}
    }

    pub fn hasMoreCommands(&self) -> bool{
	self.l < self.n
    }

    pub fn advance(&mut self) -> (){
	self.current_line = self.lines[self.l].clone();
	self.l += 1;
    }

    pub fn commandType(&self) -> String{
	if self.current_line.contains("@"){
	    "A_COMMAND".to_string()
	}
	else if self.current_line.contains("=")||self.current_line.contains(";"){
	    "C_COMMAND".to_string()
	}
	else if self.current_line.contains("(")&&self.current_line.contains(")"){
	    "L_COMMAND".to_string()
	}
	else{
	    panic!("Parser commandType Error");
	}
    }

    pub fn symbol(&self) -> String{
	if self.commandType()=="A_COMMAND"{
	    let trimed: Vec<&str> = self.current_line.split("@").collect();
	    return trimed[1].trim().to_string();
	}
	else if self.commandType()=="L_COMMAND"{
	    let mut trimed: Vec<&str> = self.current_line.split("(").collect();
	    trimed = trimed[1].split(")").collect();
	    return trimed[0].trim().to_string();	    
	}
	else{
	    panic!("Parser Symbol Error")
	}
    }

    pub fn dest(&self) -> String{
	if self.current_line.contains("="){
	    //extraction dest part
	    let trimed: Vec<&str> = self.current_line.split("=").collect();
	    let dest = trimed[0].trim().to_string();
	    
	    if dest=="A"{
		"A".to_string()
	    }
	    else if dest=="D"{
		"D".to_string()
	    }
	    else if dest=="M"{
		"M".to_string()
	    }
		
	    else if dest=="MD"{
		 "MD".to_string()
	    }
	    else if dest=="DM"{
		 "MD".to_string()
	    }
		
	    else if dest=="AM"{
		 "AM".to_string()
	    }
	    else if dest=="MA"{
		 "AM".to_string()
	    }
		
	    else if dest=="AD"{
		 "AD".to_string()
	    }
	    else if dest=="DA"{
		 "AD".to_string()
	    }

	    else if dest=="ADM"{
		 "ADM".to_string()
	    }
	    else if dest=="AMD"{
		 "ADM".to_string()
	    }
	    else if dest=="DAM"{
		 "ADM".to_string()
	    }
	    else if dest=="DMA"{
		 "ADM".to_string()
	    }
	    else if dest=="MAD"{
		 "ADM".to_string()
	    }
	    else if dest=="MDA"{
		 "ADM".to_string()
	    }

	    else{
		panic!("Parser Dest Error");
	    }
	}

	else{
	    "null".to_string()
	}
    }


    pub fn comp(&self) -> String{
	//extraction comp part
	let mut comp:String = self.current_line.clone();
	if comp.contains("="){	    
	    let trimed: Vec<&str> = comp.split("=").collect();
	    comp = trimed[1].trim().to_string();
	}
	if comp.contains(";"){
	    let trimed: Vec<&str> = comp.split(";").collect();
	    comp = trimed[0].trim().to_string();	    
	}

	if comp == "0"{
	    "0".to_string()
	}
	else if comp == "1"{
	    "1".to_string()
	}
	else if comp == "-1"{
	    "-1".to_string()
	}
	else if comp == "D"{
	    "D".to_string()
	}
	else if comp == "A"{
	    "A".to_string()
	}
	else if comp == "!D"{
	    "!D".to_string()
	}
	else if comp == "!A"{
	    "!A".to_string()
	}
	else if comp == "-D"{
	    "-D".to_string()
	}
	else if comp == "-A"{
	    "-A".to_string()
	}
	
	else if comp == "D+1"{
	    "D+1".to_string()
	}
	else if comp == "1+D"{
	    "D+1".to_string()
	}

	else if comp == "A+1"{
	    "A+1".to_string()
	}
	else if comp == "1+A"{
	    "A+1".to_string()
	}
	
	else if comp == "D-1"{
	    "D-1".to_string()
	}
	else if comp == "-1+D"{
	    "D-1".to_string()
	}

	else if comp == "A-1"{
	    "A-1".to_string()
	}
	else if comp == "-1+A"{
	    "A-1".to_string()
	}

	else if comp == "D+A"{
	    "D+A".to_string()
	}
	else if comp == "A+D"{
	    "D+A".to_string()
	}

	else if comp == "D-A"{
	    "D-A".to_string()
	}
	else if comp == "-A+D"{
	    "-A+D".to_string()
	}

	else if comp == "A-D"{
	    "A-D".to_string()
	}
	else if comp == "-A+D"{
	    "-A+D".to_string()
	}

	else if comp == "D&A"{
	    "D&A".to_string()
	}
	else if comp == "A&D"{
	    "D&A".to_string()
	}

	else if comp == "D|A"{
	    "D|A".to_string()
	}
	else if comp == "A|D"{
	    "D|A".to_string()
	}

	else if comp == "M"{
	    "M".to_string()
	}
	else if comp == "!M"{
	    "!M".to_string()
	}
	else if comp == "-M"{
	    "-M".to_string()
	}
	
	else if comp == "M+1"{
	    "M+1".to_string()
	}
	else if comp == "1+M"{
	    "M+1".to_string()
	}

	else if comp == "M-1"{
	    "M-1".to_string()
	}
	else if comp == "-1+M"{
	    "M-1".to_string()
	}
	
	else if comp == "D+M"{
	    "D+M".to_string()
	}
	else if comp == "M+D"{
	    "D+M".to_string()
	}

	else if comp == "D-M"{
	    "D-M".to_string()
	}
	else if comp == "-M+D"{
	    "D-M".to_string()
	}

	else if comp == "M-D"{
	    "M-D".to_string()
	}
	else if comp == "-D+M"{
	    "M-D".to_string()
	}

	else if comp == "D&M"{
	    "D&M".to_string()
	}
	else if comp == "M&D"{
	    "D&M".to_string()
	}

	else if comp == "D|M"{
	    "D|M".to_string()
	}
	else if comp == "M|D"{
	    "D|M".to_string()
	}
	else{
	    panic!("Parser Comp Error");
	}
    }

    pub fn jump(&self) -> String{
	//extraction comp part
	let mut jump:String = self.current_line.clone();
	if jump.contains(";"){
	    let trimed: Vec<&str> = jump.split(";").collect();
	    jump = trimed[1].trim().to_string();

	    if jump=="JGT"{
		"JGT".to_string()
	    }
	    else if jump=="JEQ"{
		"JEQ".to_string()
	    }
	    else if jump=="JGE"{
		"JGE".to_string()
	    }
	    else if jump=="JLT"{
		"JLT".to_string()
	    }
	    else if jump=="JNE"{
		"JNE".to_string()
	    }
	    else if jump=="JLE"{
		"JLE".to_string()
	    }
	    else if jump=="JMP"{
		"JMP".to_string()
	    }
	    else{
		panic!("Parser jump Error");
	    }
	}
	else{
	    "null".to_string()
	}
    }

    pub fn reset_pos(&mut self) -> (){
	self.l = 0;
    }
    

}
