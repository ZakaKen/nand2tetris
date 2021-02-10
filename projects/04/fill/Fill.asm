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

	//init
	@color
	M=0
	@index
	M=0

	//*Main Loop*//
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

	//*Set White*//
	(WHITE)
	@0
	D=A
	@color
	M=D
	@FILL
	0			;JMP

	//*Set Black*//
	(BLACK)
	@32767 //you can't @65535, because @ can set only 15bits
	D=A
	@32767
	D=D+A
	@1
	D=D+A
	@color
	M=D
	@FILL
	0			;JMP

	//*Fill SCREEN*//
	(FILL)
	@SCREEN
	D=A
	@index
	M=D
	//fill all screen
	(FLOOP)
	@color
	D=M
	@index
	A=M
	M=D
	//increment
	@index
	M=M+1
	//judge refill or break
	@24575 	//16384 + 8192 - 1= 24575
	D=A
	@index
	D=D-M
	@FLOOP
	D			;JGE
	@ENDLOOP
	0			;JMP
