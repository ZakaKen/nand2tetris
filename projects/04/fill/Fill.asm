// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.
// Put your code here.
	
	(LOOP)
	//Read input
	@24576
	D=M
	//switch WorB
	@WHITE
	D			;JEQ
	@BLACK
	D			;JNE
	//end loop
	(ENDLOOP)
	@LOOP
	0			;JMP

	//Set White
	(WHITE)
	@0
	D=A
	@R1
	M=D
	@SCREEN
	M=0
	@ENDLOOP
	0			;JMP

	//Set Black
	(BLACK)
	@1
	D=A
	@R1
	M=D
	@SCREEN
	M=1
	@ENDLOOP
	0			;JMP
