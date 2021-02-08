// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.

	// Put your code here.
	//init
	@i
	M=1
	@R2
	M=0

	//LOOP add R0, R1 times
	(LOOP)
	//if R1-i<0 then break
	@i 
	D=M
	@R1
	D=M-D
	@END
	D			;JLT

	//add R0 to R2
	@R0
	D=M
	@R2
	M=M+D

	//increment
	@i 
	M=M+1

	//jump to (LOOP)
	@LOOP
	0			;JMP

	//END LOOP
	(END)
	@END
	0			;JMP

	
