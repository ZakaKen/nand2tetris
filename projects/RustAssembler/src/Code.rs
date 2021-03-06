pub fn dest(mnemonic: String) -> u16{
    if mnemonic=="null"{
	0b000
    }
    else if mnemonic=="M"{
	0b001
    }
    else if mnemonic=="D"{
	0b010
    }
    else if mnemonic=="A"{
	0b100
    }
    else if mnemonic=="MD"{
	0b011
    }
    else if mnemonic=="AM"{
	0b101
    }
    else if mnemonic=="AD"{
	0b110
    }
    else if mnemonic=="AMD"{
	0b111
    }
    else{
	//Error
	panic!("Code::dest Error");
    }
}

pub fn comp(mnemonic: String) -> u16{
    if mnemonic=="0"{
	0b0101010
    }
    else if mnemonic=="1"{
	0b0111111
    }
    else if mnemonic=="-1"{
	0b0111010
    }
    else if mnemonic=="D"{
	0b0001100
    }
    else if mnemonic=="A"{
	0b0110000
    }
    else if mnemonic=="!D"{
	0b0001101
    }
    else if mnemonic=="!A"{
	0b0110001
    }
    else if mnemonic=="-D"{
	0b0001111
    }
    else if mnemonic=="-A"{
	0b0110011
    }
    else if mnemonic=="D+1"{
	0b0011111
    }
    else if mnemonic=="A+1"{
	0b0110111
    }
    else if mnemonic=="D-1"{
	0b0001110
    }
    else if mnemonic=="A-1"{
	0b0110010
    }
    else if mnemonic=="D+A"{
	0b0000010
    }
    else if mnemonic=="D-A"{
	0b0010011
    }
    else if mnemonic=="A-D"{
	0b0000111
    }
    else if mnemonic=="D&A"{
	0b0000000
    }
    else if mnemonic=="D|A"{
	0b0010101
    }
    else if mnemonic=="M"{
	0b1110000
    }
    else if mnemonic=="!M"{
	0b1110001
    }
    else if mnemonic=="-M"{
	0b1110011
    }
    else if mnemonic=="M+1"{
	0b1110111
    }
    else if mnemonic=="M-1"{
	0b1110010
    }
    else if mnemonic=="D+M"{
	0b1000010
    }
    else if mnemonic=="D-M"{
	0b1010011
    }
    else if mnemonic=="M-D"{
	0b1000111
    }
    else if mnemonic=="D&M"{
	0b1000000
    }
    else if mnemonic=="D|M"{
	0b1010101
    }
    else {
	//error
	panic!("Code::comp Error");
    }
}

pub fn jump(mnemonic: String) -> u16{
    if mnemonic=="null"{
	0b000
    }
    else if mnemonic=="JGT"{
	0b001
    }
    else if mnemonic=="JEQ"{
	0b010
    }
    else if mnemonic=="JGE"{
	0b011
    }
    else if mnemonic=="JLT"{
	0b100
    }
    else if mnemonic=="JNE"{
	0b101
    }
    else if mnemonic=="JLE"{
	0b110
    }
    else if mnemonic=="JMP"{
	0b111
    }
    else{
	//Error
	panic!("Code::jump Error");
    }
}

