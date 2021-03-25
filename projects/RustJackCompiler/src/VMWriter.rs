use std::fs::File;
use std::io::{BufWriter, Write};

pub struct VMWriter{
    pub writer: BufWriter<File>,
}

impl VMWriter{
    pub fn new(out_file_path: String) -> VMWriter{
	let trimed: Vec<&str> = out_file_path.split("/").collect();
	println!("VMWriter -> {}", trimed.last().unwrap());

	VMWriter{
	    writer: BufWriter::new(File::create(out_file_path).unwrap()),
	}
    }

    pub fn writeCode(&mut self, code: String) -> (){
	let _ = self.writer.write(format!("{}\n", code).as_bytes());
    }

    pub fn writePush(&mut self, Segment: String, Index: u16)-> (){
	self.writeCode(format!("push {} {}", Segment, Index));
    }

    pub fn writePop(&mut self, Segment: String, Index: u16)-> (){
	self.writeCode(format!("pop {} {}", Segment, Index));
    }

    pub fn writeArithmetic(&mut self, command: String) -> (){
	match &command[..]{
	    "ADD" => {self.writeCode("add".to_string());}
	    "SUB" => {self.writeCode("sub".to_string());}
	    "NEG" => {self.writeCode("neg".to_string());}
	    "EQ" => {self.writeCode("eq".to_string());}
	    "GT" => {self.writeCode("gt".to_string());}
	    "LT" => {self.writeCode("lt".to_string());}
	    "AND" => {self.writeCode("and".to_string());}
	    "OR" => {self.writeCode("or".to_string());}
	    "NOT" => {self.writeCode("not".to_string());}
	    _ => (panic!("VMWriter::writeArithmetic undefined command"))
	}
    }

    pub fn writeLabel(&mut self, label: String) -> (){
	self.writeCode(format!("label {}", label));
    }

    pub fn writeGoto(&mut self, label: String) -> (){
	self.writeCode(format!("goto {}", label));
    }

    pub fn writeIf(&mut self, label: String) -> (){
	self.writeCode(format!("if-goto {}", label));
    }

    pub fn writeCall(&mut self, name: String, nArgs: u16) -> (){
	self.writeCode(format!("call {} {}", name, nArgs));
    }

    pub fn writeFunction(&mut self, name: String, nLocals: u16) -> (){
	self.writeCode(format!("function {} {}", name, nLocals));
    }

    pub fn writeReturn(&mut self) -> (){
	self.writeCode("return".to_string());
    }

    

}
