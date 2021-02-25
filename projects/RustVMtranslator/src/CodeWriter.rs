use std::fs::File;
use std::io::{BufWriter, Write};


pub struct CodeWriter{
    pub filename:String,
    pub funcname:String,
    pub writer: BufWriter<File>,
    pub comp_count: usize,
    pub ret_count: usize,
}

impl CodeWriter{
    pub fn new(name: String) -> CodeWriter{
	println!("codewriter -> {}", name);
	CodeWriter{
	    filename: name.clone(),
	    funcname: "".to_string(),
	    writer: BufWriter::new(File::create(name).unwrap()),
	    comp_count: 0,
	    ret_count: 0,
	}

    }

    pub fn VMinit(&mut self) -> (){
	//set SP 261
	let _ = self.writer.write(format!("@261\n").as_bytes());
	let _ = self.writer.write(format!("D=A\n").as_bytes());
	let _ = self.writer.write(format!("@SP\n").as_bytes());
	let _ = self.writer.write(format!("M=D\n").as_bytes());

	//
	self.writePushPop("C_PUSH".to_string(), "constant".to_string(), 4);
	self.writeCall("Main.fibonacci".to_string(), 1);
	self.writeLabel("Sys.init$WHILE".to_string());
	self.writeGoto("Sys.init$WHILE".to_string());
	    
	
	/*
	//set SP 256
	let _ = self.writer.write(format!("@256\n").as_bytes());
	let _ = self.writer.write(format!("D=A\n").as_bytes());
	let _ = self.writer.write(format!("@SP\n").as_bytes());
	let _ = self.writer.write(format!("M=D\n").as_bytes());
	
	//set LCL 300
	let _ = self.writer.write(format!("@300\n").as_bytes());
	let _ = self.writer.write(format!("D=A\n").as_bytes());
	let _ = self.writer.write(format!("@LCL\n").as_bytes());
	let _ = self.writer.write(format!("M=D\n").as_bytes());

	//set ARG 400
	let _ = self.writer.write(format!("@400\n").as_bytes());
	let _ = self.writer.write(format!("D=A\n").as_bytes());
	let _ = self.writer.write(format!("@ARG\n").as_bytes());
	let _ = self.writer.write(format!("M=D\n").as_bytes());

	//set THIS 3000
	let _ = self.writer.write(format!("@3000\n").as_bytes());
	let _ = self.writer.write(format!("D=A\n").as_bytes());
	let _ = self.writer.write(format!("@THIS\n").as_bytes());
	let _ = self.writer.write(format!("M=D\n").as_bytes());

	//set THAT 3010
	let _ = self.writer.write(format!("@3010\n").as_bytes());
	let _ = self.writer.write(format!("D=A\n").as_bytes());
	let _ = self.writer.write(format!("@THAT\n").as_bytes());
	let _ = self.writer.write(format!("M=D\n").as_bytes());

	//set static 20
	for i in 0..21{
	    let _ = self.writer.write(format!("@{}.{}\n", self.filename, i).as_bytes());
	}
	*/
    }

    pub fn writeArithmetic(&mut self, command: String) -> (){
	/*do arithmetic and D = result*/
	match &command[..] {
	    "add" => {
		self.pop_tD();
		self.pop_tM();
		//Set D = Stack[SP-2] + stackp[SP-1]
		let _ = self.writer.write(format!("D=M+D\n").as_bytes());
	    }

	    "sub" => {
		self.pop_tD();
		self.pop_tM();
		//Set D = Stack[SP-2] - stackp[SP-1]
		let _ = self.writer.write(format!("D=M-D\n").as_bytes());
	    }

	    "neg" => {
		self.pop_tM();
		//Set D = -Stack[SP-1]
		let _ = self.writer.write(format!("D=-M\n").as_bytes());
	    }

	    "eq" => {
		self.pop_tD();
		self.pop_tM();
		//Set D = Stack[SP-2] - stackp[SP-1]
		let _ = self.writer.write(format!("D=M-D\n").as_bytes());
		self.compD("JEQ".to_string());
		
		
	    }

	    "gt" => {
		self.pop_tD();
		self.pop_tM();
		//Set D = Stack[SP-2] - stackp[SP-1]
		let _ = self.writer.write(format!("D=M-D\n").as_bytes());
		self.compD("JGT".to_string());
	    }

	    "lt" => {
		self.pop_tD();
		self.pop_tM();
		//Set D = Stack[SP-2] - stackp[SP-1]
		let _ = self.writer.write(format!("D=M-D\n").as_bytes());
		self.compD("JLT".to_string());
	    }

	    "and" => {
		self.pop_tD();
		self.pop_tM();
		//Set D = Stack[SP-1] & stackp[SP-2]
		let _ = self.writer.write(format!("D=D&M\n").as_bytes());
	    }

	    "or" => {
		self.pop_tD();
		self.pop_tM();
		//Set D = Stack[SP-1] | stackp[SP-2]
		let _ = self.writer.write(format!("D=D|M\n").as_bytes());
	    }

	    "not" => {
		self.pop_tM();
		//Set D = !Stack[SP-1]
		let _ = self.writer.write(format!("D=!M\n").as_bytes());
	    }

	    _ => {
		panic!("writeArithmetic not defined command");
	    }
	}
	
	/*push result*/
	self.push_fD();


    }

    pub fn writePushPop(&mut self, command: String, segment: String, index: u16) -> (){
	/*Memory accsess by segment*/
	match &command[..] {
	    "C_PUSH" => {
		//D = segment[index] or constant
		match &segment[..] {
		    "local" => {
			self.SEGi_tD("LCL".to_string(), index);
		    }

		    "argument" => {
			self.SEGi_tD("ARG".to_string(), index);
		    }

		    "this" => {
			self.SEGi_tD("THIS".to_string(), index);
		    }

		    "that" => {
			self.SEGi_tD("THAT".to_string(), index);
		    }

		    "pointer" => {
			let _ = self.writer.write(format!("@{}\n", index+3).as_bytes());//M[pointer + i] = M[3+i]
			let _ = self.writer.write(format!("D=M\n").as_bytes()); //D=M[pointer+i]
		    }

		    "temp" => {
			let _ = self.writer.write(format!("@{}\n", index+5).as_bytes());//M[temp + i] = M[5+i]
			let _ = self.writer.write(format!("D=M\n").as_bytes()); //D=M[temp+i]
		    }

		    "constant" => {
			let _ = self.writer.write(format!("@{}\n", index).as_bytes());
			let _ = self.writer.write(format!("D=A\n").as_bytes());
		    }

		    "static" => {
			let _ = self.writer.write(format!("@{}.{}\n", self.filename, index).as_bytes());
			let _ = self.writer.write(format!("D=M\n").as_bytes());
		    }

		    _ => {
			panic!("writePushPop undefined segment");
		    }
		}

		/*Push*/ //M[SP] = D 
		self.push_fD();
	    }

	    "C_POP" => {
		match &segment[..] {
		    "local" => {
			self.popSEGi_fStk("LCL".to_string(), index);
		    }

		    "argument" => {
			self.popSEGi_fStk("ARG".to_string(), index);
		    }

		    "this" => {
			self.popSEGi_fStk("THIS".to_string(), index);
		    }

		    "that" => {
			self.popSEGi_fStk("THAT".to_string(), index);
		    }

		    "pointer" => {
			self.pop_tD();

			//M[pointer+i] = D
			let _ = self.writer.write(format!("@{}\n", index+3).as_bytes());
			let _ = self.writer.write(format!("M=D\n").as_bytes()); //M[pointer+i] = D
		    }

		    "temp" => {
			self.pop_tD();
			
			//M[temp+i] = D
			let _ = self.writer.write(format!("@{}\n", index+5).as_bytes());
			let _ = self.writer.write(format!("M=D\n").as_bytes()); //M[temp+i] = D
		    }

		    "static" => {
			self.pop_tD();

			//static[index] = D
			let _ = self.writer.write(format!("@{}.{}\n", self.filename, index).as_bytes());
			let _ = self.writer.write(format!("M=D\n").as_bytes());
		    }

		    _ => {
			panic!("writePushPop undefined segment");
		    }
		}
	    }

	    _ => {
		panic!("writePushPop undefined command");
	    }
	}
    }

    pub fn writeLabel(&mut self, label: String) -> (){
	//let _ = self.writer.write(format!("({}${})\n", self.funcname, label).as_bytes());
	let _ = self.writer.write(format!("(${})\n", label).as_bytes());
    }
    
    pub fn writeGoto(&mut self, label: String) -> (){
	//let _ = self.writer.write(format!("@{}${}\n", self.funcname, label).as_bytes());
	let _ = self.writer.write(format!("@${}\n", label).as_bytes());
	let _ = self.writer.write(format!("0;JMP\n").as_bytes());
    }

    pub fn writeIf(&mut self, label: String) -> (){
	self.pop_tD();
	//let _ = self.writer.write(format!("@{}${}\n", self.funcname, label).as_bytes());
	let _ = self.writer.write(format!("@${}\n", label).as_bytes());
	let _ = self.writer.write(format!("D;JNE\n").as_bytes());
    }

    pub fn writeCall(&mut self, functionName: String, numArgs: u16) -> (){
	//println!("call {} {}", functionName, numArgs);
	//PUSH RETURN_ADDR	
	//let _ = self.writer.write(format!("@{}$RETURN_ADDR\n", functionName).as_bytes());
	let _ = self.writer.write(format!("@$RETURN_ADDR{}\n", self.ret_count).as_bytes());
	let _ = self.writer.write(format!("D=A\n").as_bytes()); //D=RETURN_ADDR
	self.push_fD();

	//push LCL
	self.SEG_tD("LCL".to_string());
	self.push_fD();

	//push ARG
	self.SEG_tD("ARG".to_string());
	self.push_fD();

	//push THIS
	self.SEG_tD("THIS".to_string());
	self.push_fD();

	//push THAT
	self.SEG_tD("THAT".to_string());
	self.push_fD();

	//ARG = SP-n-5
	let _ = self.writer.write(format!("@SP\n").as_bytes());
	let _ = self.writer.write(format!("D=M\n").as_bytes()); //D = SP
	let _ = self.writer.write(format!("@5\n").as_bytes());
	let _ = self.writer.write(format!("D=D-A\n").as_bytes()); //D = SP - 5
	let _ = self.writer.write(format!("@{}\n", numArgs).as_bytes());
	let _ = self.writer.write(format!("D=D-A\n").as_bytes()); //D = SP - 5 - numArgs
	let _ = self.writer.write(format!("@ARG\n").as_bytes());
	let _ = self.writer.write(format!("M=D\n").as_bytes()); //ARG = SP-5-numArgs

	//LCL = SP
	let _ = self.writer.write(format!("@SP\n").as_bytes());
	let _ = self.writer.write(format!("D=M\n").as_bytes()); //D = SP
	let _ = self.writer.write(format!("@LCL\n").as_bytes());
	let _ = self.writer.write(format!("M=D\n").as_bytes()); //LCL = SP
	
	//go to function
	self.writeGoto(functionName);

	//Label for return address
	self.writeLabel(format!("RETURN_ADDR{}", self.ret_count).to_string());
	self.ret_count += 1;
    }

    pub fn writeReturn(&mut self) -> (){
	//return address -> R13 (where reutrn address saved is LCL-5)
	self.state_i_tD(5);
	let _ = self.writer.write(format!("@R13\n").as_bytes());
	let _ = self.writer.write(format!("M=D\n").as_bytes()); //*R13 = return address

	//pop func return value to ARG.
	self.pop_tD();
	let _ = self.writer.write(format!("@ARG\n").as_bytes());
	let _ = self.writer.write(format!("A=M\n").as_bytes()); //set A=ARG
	let _ = self.writer.write(format!("M=D\n").as_bytes()); // M[ARG] = return value

	//SP -> ARG+1 (where SP was before function called)
	let _ = self.writer.write(format!("@ARG\n").as_bytes());
	let _ = self.writer.write(format!("D=M\n").as_bytes()); //D = ARG
	let _ = self.writer.write(format!("D=D+1\n").as_bytes()); //D = ARG+1
	let _ = self.writer.write(format!("@SP\n").as_bytes());
	let _ = self.writer.write(format!("M=D\n").as_bytes()); //SP = ARG+1

	//THAT -> M[LCL-1] (where *THAT saved before function called)
	self.state_i_tD(1);
	let _ = self.writer.write(format!("@THAT\n").as_bytes());
	let _ = self.writer.write(format!("M=D\n").as_bytes()); //THAT recalled

	//THIS -> M[LCL-2] (where *THAT saved before function called)
	self.state_i_tD(2);
	let _ = self.writer.write(format!("@THIS\n").as_bytes());
	let _ = self.writer.write(format!("M=D\n").as_bytes()); //THIS recalled

	//ARG -> M[LCL-3] (where *THAT saved before function called)
	self.state_i_tD(3);
	let _ = self.writer.write(format!("@ARG\n").as_bytes());
	let _ = self.writer.write(format!("M=D\n").as_bytes()); //ARG recalled

	//LCL -> M[LCL-4] (where *THAT saved before function called)
	self.state_i_tD(4);
	let _ = self.writer.write(format!("@LCL\n").as_bytes());
	let _ = self.writer.write(format!("M=D\n").as_bytes()); //LCL recalled 

	//jump to return address
	let _ = self.writer.write(format!("@R13\n").as_bytes());
	let _ = self.writer.write(format!("A=M\n").as_bytes());
	let _ = self.writer.write(format!("0;JMP\n").as_bytes());
	
    }

    pub fn writeFunction(&mut self, functionName: String, numLocals: u16) -> (){
	self.funcname = functionName.clone();
	let _ = self.writer.write(format!("(${})\n", functionName).as_bytes());
	for _i in 0..numLocals {
	    let _ = self.writer.write(format!("@0\n").as_bytes());
	    let _ = self.writer.write(format!("D=A\n").as_bytes());
	    self.push_fD();
	}
    }



    /*sub Method to support write assembry code*/
    
    pub fn pop_tD(&mut self) -> (){
	//Set SP=SP-1 and A=SP
	let _ = self.writer.write(format!("@SP\n").as_bytes());
	let _ = self.writer.write(format!("M=M-1\n").as_bytes());
	let _ = self.writer.write(format!("A=M\n").as_bytes());
	//Set D = Stack[SP-1]
	let _ = self.writer.write(format!("D=M\n").as_bytes());
    }
    
    pub fn pop_tM(&mut self) -> (){
	//Set SP=SP-1 and A=SP
	let _ = self.writer.write(format!("@SP\n").as_bytes());
	let _ = self.writer.write(format!("M=M-1\n").as_bytes());
	let _ = self.writer.write(format!("A=M\n").as_bytes());
    }

    pub fn push_fD(&mut self) -> (){
	let _ = self.writer.write(format!("@SP\n").as_bytes());
	let _ = self.writer.write(format!("A=M\n").as_bytes());
	let _ = self.writer.write(format!("M=D\n").as_bytes());
	let _ = self.writer.write(format!("@SP\n").as_bytes());
	let _ = self.writer.write(format!("M=M+1\n").as_bytes());
    }

    pub fn compD(&mut self, comp_op: String) -> (){
	//if D [comp_op] 0 then put true into D
	//else put false into D

	//if D==0 then jump to (TRUE)
	let _ = self.writer.write(format!("@COMP_TRUE{}\n", self.comp_count).as_bytes());
	let _ = self.writer.write(format!("D;{}\n", comp_op).as_bytes());
	//else D=False and jump to (END_COMP)
	let _ = self.writer.write(format!("D=0\n").as_bytes());
	let _ = self.writer.write(format!("@COMP_END_COMP{}\n", self.comp_count).as_bytes());
	let _ = self.writer.write(format!("0;JMP\n").as_bytes());
	
	//(TRUE) D=True and jump to (END_COMP)
	let _ = self.writer.write(format!("(COMP_TRUE{})\n", self.comp_count).as_bytes());
	let _ = self.writer.write(format!("D=-1\n").as_bytes());
	let _ = self.writer.write(format!("@COMP_END_COMP{}\n", self.comp_count).as_bytes());
	let _ = self.writer.write(format!("0;JMP\n").as_bytes());
	
	//(END_COMP)
	let _ = self.writer.write(format!("(COMP_END_COMP{})\n", self.comp_count).as_bytes());
	
	//count increment
	self.comp_count += 1;
    }

    pub fn SEGi_tD(&mut self, seg: String, index: u16) -> (){
	let _ = self.writer.write(format!("@{}\n", seg).as_bytes());
	let _ = self.writer.write(format!("D=M\n").as_bytes()); //D=SEGbase
	let _ = self.writer.write(format!("@{}\n", index).as_bytes());
	let _ = self.writer.write(format!("D=D+A\n").as_bytes()); //D=SEGbase + index
	let _ = self.writer.write(format!("A=D\n").as_bytes());
	let _ = self.writer.write(format!("D=M\n").as_bytes()); //D=M[SEGbase + index]
    }

    pub fn popSEGi_fStk(&mut self, seg: String, index: u16) -> (){
	//set R13 = SEGbase + index
	let _ = self.writer.write(format!("@{}\n", seg).as_bytes());
	let _ = self.writer.write(format!("D=M\n").as_bytes()); //D=SEGbase
	let _ = self.writer.write(format!("@{}\n", index).as_bytes());
	let _ = self.writer.write(format!("D=D+A\n").as_bytes()); //D=SEGbase + index
	let _ = self.writer.write(format!("@R13\n").as_bytes());
	let _ = self.writer.write(format!("M=D\n").as_bytes());//R13 = SEGbase + index
	
	//POP D = M[SP]
	let _ = self.writer.write(format!("@SP\n").as_bytes());
	let _ = self.writer.write(format!("M=M-1\n").as_bytes()); //SP = SP - 1
	let _ = self.writer.write(format!("A=M\n").as_bytes()); //set A=SP
	let _ = self.writer.write(format!("D=M\n").as_bytes()); //D = M[SP]

	//set M[SEGbase + index] = D
	let _ = self.writer.write(format!("@R13\n").as_bytes());
	let _ = self.writer.write(format!("A=M\n").as_bytes());
	let _ = self.writer.write(format!("M=D\n").as_bytes()); //M[SEGbase + index] = D
    }

    pub fn state_i_tD(&mut self, index: u16) -> (){
	let _ = self.writer.write(format!("@LCL\n").as_bytes());
	let _ = self.writer.write(format!("D=M\n").as_bytes()); //set D=*LCL
	let _ = self.writer.write(format!("@{}\n", index).as_bytes());
	let _ = self.writer.write(format!("D=D-A\n").as_bytes()); //D = *LCL - 5 = &return address
	let _ = self.writer.write(format!("A=D\n").as_bytes());
	let _ = self.writer.write(format!("D=M\n").as_bytes()); //D = return address
    }

    pub fn SEG_tD(&mut self, seg: String) -> (){
	let _ = self.writer.write(format!("@{}\n", seg).as_bytes());
	let _ = self.writer.write(format!("D=M\n").as_bytes()); //D=SEGbase
    }

    
}

