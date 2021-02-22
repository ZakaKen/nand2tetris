use std::fs::File;
use std::io::{BufWriter, Write};

pub struct CodeWriter{
    pub filename:String,
    pub writer: BufWriter<File>,
    pub eq_count: usize,
    pub gt_count: usize,
    pub lt_count: usize,
}

impl CodeWriter{
    pub fn new(name: String) -> CodeWriter{
	CodeWriter{
	    filename: name.clone(),
	    writer: BufWriter::new(File::create(name).unwrap()),
	    eq_count: 0,
	    gt_count: 0,
	    lt_count: 0,
	}

    }

    pub fn VMinit(&mut self) -> (){
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
    }

    pub fn writeArithmetic(&mut self, command: String) -> (){
	/*do arithmetic and D = result*/
	match &command[..] {
	    "add" => {
		//Set SP=SP-1 and A=SP
		let _ = self.writer.write(format!("@SP\n").as_bytes());
		let _ = self.writer.write(format!("M=M-1\n").as_bytes());
		let _ = self.writer.write(format!("A=M\n").as_bytes());
		//Set D = Stack[SP-1]
		let _ = self.writer.write(format!("D=M\n").as_bytes());
		//Set SP=SP-2 and A=SP
		let _ = self.writer.write(format!("@SP\n").as_bytes());
		let _ = self.writer.write(format!("M=M-1\n").as_bytes());
		let _ = self.writer.write(format!("A=M\n").as_bytes());
		//Set D = Stack[SP-2] + stackp[SP-1]
		let _ = self.writer.write(format!("D=M+D\n").as_bytes());
	    }

	    "sub" => {
		//Set SP=SP-1 and A=SP
		let _ = self.writer.write(format!("@SP\n").as_bytes());
		let _ = self.writer.write(format!("M=M-1\n").as_bytes());
		let _ = self.writer.write(format!("A=M\n").as_bytes());
		//Set D = Stack[SP-1]
		let _ = self.writer.write(format!("D=M\n").as_bytes());
		//Set SP=SP-2 and A=SP
		let _ = self.writer.write(format!("@SP\n").as_bytes());
		let _ = self.writer.write(format!("M=M-1\n").as_bytes());
		let _ = self.writer.write(format!("A=M\n").as_bytes());
		//Set D = Stack[SP-2] - stackp[SP-1]
		let _ = self.writer.write(format!("D=M-D\n").as_bytes());
	    }

	    "neg" => {
		//Set SP=SP-1 and A=SP
		let _ = self.writer.write(format!("@SP\n").as_bytes());
		let _ = self.writer.write(format!("M=M-1\n").as_bytes());
		let _ = self.writer.write(format!("A=M\n").as_bytes());
		//Set D = -Stack[SP-1]
		let _ = self.writer.write(format!("D=-M\n").as_bytes());
	    }

	    "eq" => {
		//Set SP=SP-1 and A=SP
		let _ = self.writer.write(format!("@SP\n").as_bytes());
		let _ = self.writer.write(format!("M=M-1\n").as_bytes());
		let _ = self.writer.write(format!("A=M\n").as_bytes());
		//Set D = Stack[SP-1]
		let _ = self.writer.write(format!("D=M\n").as_bytes());
		//Set SP=SP-2 and A=SP
		let _ = self.writer.write(format!("@SP\n").as_bytes());
		let _ = self.writer.write(format!("M=M-1\n").as_bytes());
		let _ = self.writer.write(format!("A=M\n").as_bytes());
		//Set D = Stack[SP-2] - stackp[SP-1]
		let _ = self.writer.write(format!("D=M-D\n").as_bytes());

		//jump to (TRUE)
		let _ = self.writer.write(format!("@EQ_TRUE{}\n", self.eq_count).as_bytes());
		let _ = self.writer.write(format!("D;JEQ\n").as_bytes());
		//not jump
		let _ = self.writer.write(format!("D=0\n").as_bytes());
		let _ = self.writer.write(format!("@EQ_END_COMP{}\n", self.eq_count).as_bytes());
		let _ = self.writer.write(format!("0;JMP\n").as_bytes());
		
		//(TRUE)
		let _ = self.writer.write(format!("(EQ_TRUE{})\n", self.eq_count).as_bytes());
		let _ = self.writer.write(format!("D=-1\n").as_bytes());
		let _ = self.writer.write(format!("@EQ_END_COMP{}\n", self.eq_count).as_bytes());
		let _ = self.writer.write(format!("0;JMP\n").as_bytes());

		//(END_COMP)
		let _ = self.writer.write(format!("(EQ_END_COMP{})\n", self.eq_count).as_bytes());

		//count increment
		self.eq_count += 1;
	    }

	    "gt" => {
		//Set SP=SP-1 and A=SP
		let _ = self.writer.write(format!("@SP\n").as_bytes());
		let _ = self.writer.write(format!("M=M-1\n").as_bytes());
		let _ = self.writer.write(format!("A=M\n").as_bytes());
		//Set D = Stack[SP-1]
		let _ = self.writer.write(format!("D=M\n").as_bytes());
		//Set SP=SP-2 and A=SP
		let _ = self.writer.write(format!("@SP\n").as_bytes());
		let _ = self.writer.write(format!("M=M-1\n").as_bytes());
		let _ = self.writer.write(format!("A=M\n").as_bytes());
		//Set D = Stack[SP-2] - stackp[SP-1]
		let _ = self.writer.write(format!("D=M-D\n").as_bytes());

		//jump to (TRUE)
		let _ = self.writer.write(format!("@GT_TRUE{}\n", self.gt_count).as_bytes());
		let _ = self.writer.write(format!("D;JGT\n").as_bytes());
		//not jump
		let _ = self.writer.write(format!("D=0\n").as_bytes());
		let _ = self.writer.write(format!("@GT_END_COMP{}\n", self.gt_count).as_bytes());
		let _ = self.writer.write(format!("0;JMP\n").as_bytes());
		
		//(TRUE)
		let _ = self.writer.write(format!("(GT_TRUE{})\n", self.gt_count).as_bytes());
		let _ = self.writer.write(format!("D=-1\n").as_bytes());
		let _ = self.writer.write(format!("@GT_END_COMP{}\n", self.gt_count).as_bytes());
		let _ = self.writer.write(format!("0;JMP\n").as_bytes());

		//(END_COMP)
		let _ = self.writer.write(format!("(GT_END_COMP{})\n", self.gt_count).as_bytes());

		//count increment
		self.gt_count += 1;
	    }

	    "lt" => {
		//Set SP=SP-1 and A=SP
		let _ = self.writer.write(format!("@SP\n").as_bytes());
		let _ = self.writer.write(format!("M=M-1\n").as_bytes());
		let _ = self.writer.write(format!("A=M\n").as_bytes());
		//Set D = Stack[SP-1]
		let _ = self.writer.write(format!("D=M\n").as_bytes());
		//Set SP=SP-2 and A=SP
		let _ = self.writer.write(format!("@SP\n").as_bytes());
		let _ = self.writer.write(format!("M=M-1\n").as_bytes());
		let _ = self.writer.write(format!("A=M\n").as_bytes());
		//Set D = Stack[SP-2] - stackp[SP-1]
		let _ = self.writer.write(format!("D=M-D\n").as_bytes());

		//jump to (TRUE)
		let _ = self.writer.write(format!("@LT_TRUE{}\n", self.lt_count).as_bytes());
		let _ = self.writer.write(format!("D;JLT\n").as_bytes());
		//not jump
		let _ = self.writer.write(format!("D=0\n").as_bytes());
		let _ = self.writer.write(format!("@LT_END_COMP{}\n", self.lt_count).as_bytes());
		let _ = self.writer.write(format!("0;JMP\n").as_bytes());
		
		//(TRUE)
		let _ = self.writer.write(format!("(LT_TRUE{})\n", self.lt_count).as_bytes());
		let _ = self.writer.write(format!("D=-1\n").as_bytes());
		let _ = self.writer.write(format!("@LT_END_COMP{}\n", self.lt_count).as_bytes());
		let _ = self.writer.write(format!("0;JMP\n").as_bytes());

		//(END_COMP)
		let _ = self.writer.write(format!("(LT_END_COMP{})\n", self.lt_count).as_bytes());

		//count increment
		self.lt_count += 1;
	    }

	    "and" => {
		//Set SP=SP-1 and A=SP
		let _ = self.writer.write(format!("@SP\n").as_bytes());
		let _ = self.writer.write(format!("M=M-1\n").as_bytes());
		let _ = self.writer.write(format!("A=M\n").as_bytes());
		//Set D = Stack[SP-1]
		let _ = self.writer.write(format!("D=M\n").as_bytes());
		//Set SP=SP-2 and A=SP
		let _ = self.writer.write(format!("@SP\n").as_bytes());
		let _ = self.writer.write(format!("M=M-1\n").as_bytes());
		let _ = self.writer.write(format!("A=M\n").as_bytes());
		//Set D = Stack[SP-1] & stackp[SP-2]
		let _ = self.writer.write(format!("D=D&M\n").as_bytes());
	    }

	    "or" => {
		//Set SP=SP-1 and A=SP
		let _ = self.writer.write(format!("@SP\n").as_bytes());
		let _ = self.writer.write(format!("M=M-1\n").as_bytes());
		let _ = self.writer.write(format!("A=M\n").as_bytes());
		//Set D = Stack[SP-1]
		let _ = self.writer.write(format!("D=M\n").as_bytes());
		//Set SP=SP-2 and A=SP
		let _ = self.writer.write(format!("@SP\n").as_bytes());
		let _ = self.writer.write(format!("M=M-1\n").as_bytes());
		let _ = self.writer.write(format!("A=M\n").as_bytes());
		//Set D = Stack[SP-1] | stackp[SP-2]
		let _ = self.writer.write(format!("D=D|M\n").as_bytes());
	    }

	    "not" => {
		//Set SP=SP-1 and A=SP
		let _ = self.writer.write(format!("@SP\n").as_bytes());
		let _ = self.writer.write(format!("M=M-1\n").as_bytes());
		let _ = self.writer.write(format!("A=M\n").as_bytes());
		//Set D = !Stack[SP-1]
		let _ = self.writer.write(format!("D=!M\n").as_bytes());
	    }

	    _ => {
		panic!("writeArithmetic not defined command");
	    }
	}
	
	/*push result*/
	let _ = self.writer.write(format!("@SP\n").as_bytes());
	let _ = self.writer.write(format!("A=M\n").as_bytes());
	let _ = self.writer.write(format!("M=D\n").as_bytes());
	let _ = self.writer.write(format!("@SP\n").as_bytes());
	let _ = self.writer.write(format!("M=M+1\n").as_bytes());


    }

    pub fn writePushPop(&mut self, command: String, segment: String, index: u16) -> (){
	/*Memory accsess by segment*/
	match &command[..] {
	    "C_PUSH" => {
		//D = segment[index] or constant
		match &segment[..] {
		    "local" => {
			let _ = self.writer.write(format!("@LCL\n").as_bytes());
			let _ = self.writer.write(format!("D=M\n").as_bytes()); //D=LCLbase
			let _ = self.writer.write(format!("@{}\n", index).as_bytes());
			let _ = self.writer.write(format!("D=D+A\n").as_bytes()); //D=LCLbase + index
			let _ = self.writer.write(format!("A=D\n").as_bytes());
			let _ = self.writer.write(format!("D=M\n").as_bytes()); //D=M[LCLbase + index]
		    }

		    "argument" => {
			let _ = self.writer.write(format!("@ARG\n").as_bytes());
			let _ = self.writer.write(format!("D=M\n").as_bytes()); //D=ARGbase
			let _ = self.writer.write(format!("@{}\n", index).as_bytes());
			let _ = self.writer.write(format!("D=D+A\n").as_bytes()); //D=ARGbase + index
			let _ = self.writer.write(format!("A=D\n").as_bytes());
			let _ = self.writer.write(format!("D=M\n").as_bytes()); //D=M[ARGbase + index]
		    }

		    "this" => {
			let _ = self.writer.write(format!("@THIS\n").as_bytes());
			let _ = self.writer.write(format!("D=M\n").as_bytes()); //D=THISbase
			let _ = self.writer.write(format!("@{}\n", index).as_bytes());
			let _ = self.writer.write(format!("D=D+A\n").as_bytes()); //D=THISbase + index
			let _ = self.writer.write(format!("A=D\n").as_bytes());
			let _ = self.writer.write(format!("D=M\n").as_bytes()); //D=M[THISbase + index]
		    }

		    "that" => {
			let _ = self.writer.write(format!("@THAT\n").as_bytes());
			let _ = self.writer.write(format!("D=M\n").as_bytes()); //D=THATbase
			let _ = self.writer.write(format!("@{}\n", index).as_bytes());
			let _ = self.writer.write(format!("D=D+A\n").as_bytes()); //D=THATbase + index
			let _ = self.writer.write(format!("A=D\n").as_bytes());
			let _ = self.writer.write(format!("D=M\n").as_bytes()); //D=M[THATbase + index]
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
		let _ = self.writer.write(format!("@SP\n").as_bytes());
		let _ = self.writer.write(format!("A=M\n").as_bytes());
		let _ = self.writer.write(format!("M=D\n").as_bytes());
		let _ = self.writer.write(format!("@SP\n").as_bytes());
		let _ = self.writer.write(format!("M=M+1\n").as_bytes());
	    }

	    "C_POP" => {
		match &segment[..] {
		    "local" => {
			//set R13 = LCLbase + index
			let _ = self.writer.write(format!("@LCL\n").as_bytes());
			let _ = self.writer.write(format!("D=M\n").as_bytes()); //D=LCLbase
			let _ = self.writer.write(format!("@{}\n", index).as_bytes());
			let _ = self.writer.write(format!("D=D+A\n").as_bytes()); //D=LCLbase + index
			let _ = self.writer.write(format!("@R13\n").as_bytes());
			let _ = self.writer.write(format!("M=D\n").as_bytes());//R13 = LCLbase + index

			//POP D = M[SP]
			let _ = self.writer.write(format!("@SP\n").as_bytes());
			let _ = self.writer.write(format!("M=M-1\n").as_bytes()); //SP = SP - 1
			let _ = self.writer.write(format!("A=M\n").as_bytes()); //set A=SP
			let _ = self.writer.write(format!("D=M\n").as_bytes()); //D = M[SP]

			//set M[LCLbase + index] = D
			let _ = self.writer.write(format!("@R13\n").as_bytes());
			let _ = self.writer.write(format!("A=M\n").as_bytes());
			let _ = self.writer.write(format!("M=D\n").as_bytes()); //D=M[LCLbase + index]
		    }

		    "argument" => {
			//set R13 = ARGbase + index
			let _ = self.writer.write(format!("@ARG\n").as_bytes());
			let _ = self.writer.write(format!("D=M\n").as_bytes()); //D=ARGbase
			let _ = self.writer.write(format!("@{}\n", index).as_bytes());
			let _ = self.writer.write(format!("D=D+A\n").as_bytes()); //D=ARGbase + index
			let _ = self.writer.write(format!("@R13\n").as_bytes());
			let _ = self.writer.write(format!("M=D\n").as_bytes());//R13 = ARGbase + index

			//POP D = M[SP]
			let _ = self.writer.write(format!("@SP\n").as_bytes());
			let _ = self.writer.write(format!("M=M-1\n").as_bytes()); //SP = SP - 1
			let _ = self.writer.write(format!("A=M\n").as_bytes()); //set A=SP
			let _ = self.writer.write(format!("D=M\n").as_bytes()); //D = M[SP]

			//set M[ARGbase + index] = D
			let _ = self.writer.write(format!("@R13\n").as_bytes());
			let _ = self.writer.write(format!("A=M\n").as_bytes());
			let _ = self.writer.write(format!("M=D\n").as_bytes()); //D=M[ARGbase + index]

		    }

		    "this" => {
			//set R13 = THISbase + index
			let _ = self.writer.write(format!("@THIS\n").as_bytes());
			let _ = self.writer.write(format!("D=M\n").as_bytes()); //D=THISbase
			let _ = self.writer.write(format!("@{}\n", index).as_bytes());
			let _ = self.writer.write(format!("D=D+A\n").as_bytes()); //D=THISbase + index
			let _ = self.writer.write(format!("@R13\n").as_bytes());
			let _ = self.writer.write(format!("M=D\n").as_bytes());//R13 = THISbase + index

			//POP D = M[SP]
			let _ = self.writer.write(format!("@SP\n").as_bytes());
			let _ = self.writer.write(format!("M=M-1\n").as_bytes()); //SP = SP - 1
			let _ = self.writer.write(format!("A=M\n").as_bytes()); //set A=SP
			let _ = self.writer.write(format!("D=M\n").as_bytes()); //D = M[SP]

			//set M[THISbase + index] = D
			let _ = self.writer.write(format!("@R13\n").as_bytes());
			let _ = self.writer.write(format!("A=M\n").as_bytes());
			let _ = self.writer.write(format!("M=D\n").as_bytes()); //D=M[THISbase + index]
		    }

		    "that" => {
			//set R13 = THATbase + index
			let _ = self.writer.write(format!("@THAT\n").as_bytes());
			let _ = self.writer.write(format!("D=M\n").as_bytes()); //D=THATbase
			let _ = self.writer.write(format!("@{}\n", index).as_bytes());
			let _ = self.writer.write(format!("D=D+A\n").as_bytes()); //D=THATbase + index
			let _ = self.writer.write(format!("@R13\n").as_bytes());
			let _ = self.writer.write(format!("M=D\n").as_bytes());//R13 = THATbase + index

			//POP D = M[SP]
			let _ = self.writer.write(format!("@SP\n").as_bytes());
			let _ = self.writer.write(format!("M=M-1\n").as_bytes()); //SP = SP - 1
			let _ = self.writer.write(format!("A=M\n").as_bytes()); //set A=SP
			let _ = self.writer.write(format!("D=M\n").as_bytes()); //D = M[SP]

			//set M[THATbase + index] = D
			let _ = self.writer.write(format!("@R13\n").as_bytes());
			let _ = self.writer.write(format!("A=M\n").as_bytes());
			let _ = self.writer.write(format!("M=D\n").as_bytes()); //D=M[THATbase + index]
		    }

		    "pointer" => {
			//POP D = M[SP]
			let _ = self.writer.write(format!("@SP\n").as_bytes());
			let _ = self.writer.write(format!("M=M-1\n").as_bytes()); //SP = SP - 1
			let _ = self.writer.write(format!("A=M\n").as_bytes()); //set A=SP
			let _ = self.writer.write(format!("D=M\n").as_bytes()); //D = M[SP]

			//M[pointer+i] = D
			let _ = self.writer.write(format!("@{}\n", index+3).as_bytes());
			let _ = self.writer.write(format!("M=D\n").as_bytes()); //M[pointer+i] = D
		    }

		    "temp" => {
			//POP D = M[SP]
			let _ = self.writer.write(format!("@SP\n").as_bytes());
			let _ = self.writer.write(format!("M=M-1\n").as_bytes()); //SP = SP - 1
			let _ = self.writer.write(format!("A=M\n").as_bytes()); //set A=SP
			let _ = self.writer.write(format!("D=M\n").as_bytes()); //D = M[SP]

			//M[temp+i] = D
			let _ = self.writer.write(format!("@{}\n", index+5).as_bytes());
			let _ = self.writer.write(format!("M=D\n").as_bytes()); //M[temp+i] = D
		    }

		    "static" => {
			//POP D = M[SP]
			let _ = self.writer.write(format!("@SP\n").as_bytes());
			let _ = self.writer.write(format!("M=M-1\n").as_bytes()); //SP = SP - 1
			let _ = self.writer.write(format!("A=M\n").as_bytes()); //set A=SP
			let _ = self.writer.write(format!("D=M\n").as_bytes()); //D = M[SP]

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
    }//pub fn writePushPop
}

