use std::convert::TryFrom;
use JackTokenizer;
use SymbolTable;
use VMWriter;

pub struct CompilationEngine{
    pub tokens: JackTokenizer::JackTokenizer,
    pub filename:String,
    pub vmwriter: VMWriter::VMWriter,
    pub comp_count: usize,
    pub ret_count: usize,
    pub symbolTbl: SymbolTable::SymbolTable,

    pub void_flag: bool,
}

impl CompilationEngine{
    pub fn new(tokens: JackTokenizer::JackTokenizer, out_file_path: String) -> CompilationEngine{
	//out_file_path = /dir/out_file_name.xml

	CompilationEngine{
	    tokens: tokens,
	    filename: "".to_string(),
	    vmwriter: VMWriter::VMWriter::new(out_file_path),
	    comp_count: 0,
	    ret_count: 0,
	    symbolTbl: SymbolTable::SymbolTable::new(),
	    void_flag: false,
	}
    }

    pub fn startCompile(&mut self) -> (){
	println!("compile start");
	self.tokens.advance();
	if self.is_keyword("class") {
	    self.compileClass();
	}
	else {
	    panic!("CompilationEngine:: .Jack must begin with class")
	}
    }

    pub fn compileClass(&mut self) -> (){
	self.writeLine("<class>");
	self.print_type_token();
	self.tokens.advance();

	//className
	self.symbolTbl.className = self.tokens.identifier();
	self.compileIdentifier("compileClass className ?");
	
	//symbol '{'
	self.compileSymbol("{", "at compileClass");

	//compile classVarDec *
	while self.is_classVarDec(){
	    self.compileclassVarDec();
	}
	
	//subroutineDec *
	while self.is_subroutineDec(){
	    self.compileSubroutine();
	}
	
	//symbol '}'
	self.compileSymbol("}", "at compileClass");

	//end compile class
	self.writeLine("</class>");
	    
    }

    pub fn compileclassVarDec(&mut self){	
	self.writeLine("<classVarDec>");
	//static | field
	self.symbolTbl.kind_buf = self.tokens.keyWord().to_lowercase();
	self.print_type_token();
	self.tokens.advance();

	//type
	self.symbolTbl.type_buf = self.tokens.current_token.clone();
	self.compileType("compileVarDec type?");

	//varName
	self.symbolTbl.symbol_buf = self.tokens.identifier();
	self.symbolTbl.define();
	self.compileIdentifier("compileVarDec varName ?");
	

	// (, varname)*
	while self.is_symbol(","){
	    // ','
	    self.print_type_token();
	    self.tokens.advance();

	    //varName
	    self.symbolTbl.symbol_buf = self.tokens.identifier();
	    self.symbolTbl.define();
	    self.compileIdentifier("compileVarDec varName ?");
	}

	// ;
	self.compileSymbol(";", "at compileVarDec");
	
	self.writeLine("</classVarDec>");
	
    }

    pub fn compileVarDec(&mut self){	
	self.writeLine("<varDec>");
	//var
	self.symbolTbl.kind_buf = self.tokens.keyWord().to_lowercase();
	self.print_type_token();
	self.tokens.advance();

	//type
	self.symbolTbl.type_buf = self.tokens.current_token.clone();
	self.compileType("compileVarDec type?");

	//varName
	self.symbolTbl.symbol_buf = self.tokens.identifier();
	self.symbolTbl.define();
	self.compileIdentifier("compileVarDec varName ?");

	// (, varname)*
	while self.is_symbol(","){
	    // ','
	    self.print_type_token();
	    self.tokens.advance();

	    //varName
	    self.symbolTbl.symbol_buf = self.tokens.identifier();
	    self.symbolTbl.define();
	    self.compileIdentifier("compileVarDec varName ?");
	}

	// ;
	self.compileSymbol(";", "at compileVarDec");
	
	self.writeLine("</varDec>");
	
    }

    pub fn compileSubroutine(&mut self){
	self.writeLine("<subroutineDec>");
	self.symbolTbl.startSubroutine();
	
	//function | method | constructor
	let kind_buf = self.tokens.keyWord().to_lowercase();
	self.print_type_token();
	self.tokens.advance();
	
	//void | type
	if self.is_keyword("void") || self.is_type() {
	    if self.is_keyword("void"){
		self.void_flag = true;
	    }
	    else{
		self.void_flag = false;
	    }
		
	    self.print_type_token();
	    self.tokens.advance();
	}
	else{
	    println!("current token : {} {}", self.tokens.current_token, &self.tokens.tokenType()[..]);
	    panic!("CompilationEngine:: compileSubroutine void | type ?")
	}

	//subroutineName
	self.symbolTbl.subroutineName = self.tokens.identifier();
	self.compileIdentifier("compileSubroutine subroutineName ?");

	//'('
	self.compileSymbol("(", "at compileSubroutine");

	//if method arg += 1
	if &kind_buf[..] == "method"{
	    self.symbolTbl.nArg += 1;
	}
	//compileParameterList *
	self.compileParameterList();

	//')'
	self.compileSymbol(")", "at compileSubroutine");

	//subroutineBody
	self.writeLine("<subroutineBody>");
	
	//'{'
	self.compileSymbol("{", "at compileSubroutine");

	//varDec *
	while self.is_VarDec(){
	    self.compileVarDec();
	}

	//write function Dec
	// if function
	if &kind_buf[..] == "function"{
	    self.vmwriter.writeFunction(format!("{}.{}", self.symbolTbl.className, self.symbolTbl.subroutineName),
				    self.symbolTbl.nVar);
	}

	// if constructor
	else if &kind_buf[..] == "constructor" {
	    self.vmwriter.writeFunction(format!("{}.{}", self.symbolTbl.className, self.symbolTbl.subroutineName),
					self.symbolTbl.nVar);
	    
	    let size = self.symbolTbl.nField;
	    self.vmwriter.writePush("constant".to_string(), size);
	    self.vmwriter.writeCall("Memory.alloc".to_string(), 1);
	    self.vmwriter.writePop("pointer".to_string(), 0);
	}

	// if method
	else if &kind_buf[..] == "method" {
	    self.vmwriter.writeFunction(format!("{}.{}", self.symbolTbl.className, self.symbolTbl.subroutineName),
					self.symbolTbl.nVar);
	    self.vmwriter.writePush("argument".to_string(), 0);
	    self.vmwriter.writePop("pointer".to_string(), 0);
	}

	else{
	    panic!("compileSubroutine:: kind must be funticon|constructor|method")
	}
	
	//statements
	while self.is_statement(){
	    self.compileStatements();
	}

	//'}'
	self.compileSymbol("}", "at compileSubroutine");

	self.writeLine("</subroutineBody>");

	self.writeLine("</subroutineDec>");
    }

    pub fn compileParameterList(&mut self){
	self.writeLine("<parameterList>");
	while self.is_type(){
	    self.symbolTbl.kind_buf = "arg".to_string();
	    
	    //type
	    self.symbolTbl.type_buf = self.tokens.current_token.clone();
	    self.print_type_token();
	    self.tokens.advance();

	    //varname
	    self.symbolTbl.symbol_buf = self.tokens.identifier();
	    self.symbolTbl.define();
	    self.compileIdentifier("compileParameterList varName ?");
	}

	//, type varname
	while self.is_symbol(","){
	    // ','
	    self.print_type_token();
	    self.tokens.advance();

	    //type
	    self.symbolTbl.type_buf = self.tokens.current_token.clone();
	    self.compileType("compileParameterList type ?");

	    //varname
	    self.symbolTbl.symbol_buf = self.tokens.identifier();
	    self.symbolTbl.define();
	    self.compileIdentifier("compileParameterList varName ?");
	}
	
	self.writeLine("</parameterList>");
    }

    pub fn compileStatements(&mut self){
	self.writeLine("<statements>");

	//statement *
	while self.is_statement(){
	    if self.is_keyword("let"){
		self.compileLet();
	    }
	    if self.is_keyword("if"){
		self.compileIf();
	    }
	    if self.is_keyword("while"){
		self.compileWhile();
	    }
	    if self.is_keyword("do"){
		self.compileDo();
	    }
	    if self.is_keyword("return"){
		self.compileReturn();
	    }
	}
	self.writeLine("</statements>");
    }

    pub fn compileLet(&mut self){
	self.writeLine("<letStatement>");
	//let
	self.print_type_token();
	self.tokens.advance();

	//sakiyomi
	self.tokens.advance();
	
	// if varName[' expression ']
	if self.is_symbol("["){
	    self.tokens.back();
	    
	    //varName
	    if self.symbolTbl.isDefined(self.tokens.identifier()){
		self.vmwriter.writePush(self.symbolTbl.segmentOf(), self.symbolTbl.indexOf());//segment, index
	    }
	    else{
		panic!("undefiend symbol(identifier)")
	    }
	    self.compileIdentifier("compileLet varName?");
	    
	    // '['
	    self.compileSymbol("[", "at compileLet");

	    //expression
	    self.compileExpression();

	    // ']'
	    self.compileSymbol("]", "at compileLet");

	    //add varName + expression
	    self.vmwriter.writeArithmetic("ADD".to_string());

	    // '='
	    self.compileSymbol("=", "at compileLet");

	    //expression
	    self.compileExpression();

	    // ';'
	    self.compileSymbol(";", "at compileLet");

	    //expression to temp 1 = RAM[5]
	    self.vmwriter.writePop("temp".to_string(), 0);

	    //pop addr to pointer 1
	    self.vmwriter.writePop("pointer".to_string(), 1);

	    //expression to stack
	    self.vmwriter.writePush("temp".to_string(), 0);
	    
	    // input expression to varName[expression]
	    self.vmwriter.writePop("that".to_string(), 0);
	}

	//else varName
	else{
	    self.tokens.back();
	    let segment_buf:String;
	    let index_buf: u16;
	    //varName
	    if self.symbolTbl.isDefined(self.tokens.identifier()){
		segment_buf = self.symbolTbl.segmentOf();
		index_buf = self.symbolTbl.indexOf();
	    }
	    else{
		panic!("undefiend symbol(identifier)")
	    }
	    self.compileIdentifier("compileLet varName?");

	    // '='
	    self.compileSymbol("=", "at compileLet");

	    //expression
	    self.compileExpression();

	    // ';'
	    self.compileSymbol(";", "at compileLet");

	    // input expression to varName
	    self.vmwriter.writePop(segment_buf, index_buf);
	    
	}	
	self.writeLine("</letStatement>")
    }

    pub fn compileIf(&mut self){
	self.writeLine("<ifStatement>");

	//count Label num and increment countL
	let l_num:u16 = self.symbolTbl.countL();
	
	//if
	self.print_type_token();
	self.tokens.advance();

	// '('
	self.compileSymbol("(", "at compileIf");

	// expression
	self.compileExpression();

	// ')'
	self.compileSymbol(")", "at compileIf");

	//~expression
	self.vmwriter.writeArithmetic("NOT".to_string());
	//if-goto L1
	self.vmwriter.writeIf(format!("{}.L1_{}",
				      self.symbolTbl.className.clone(),
				      l_num));

	// '{'
	self.compileSymbol("{", "at compileIf");

	//statements
	self.compileStatements();

	// '}'
	self.compileSymbol("}", "at compileIf");

	//goto L2
	self.vmwriter.writeGoto(format!("{}.L2_{}",
					self.symbolTbl.className.clone(),
					l_num));

	//label L1
	self.vmwriter.writeLabel(format!("{}.L1_{}",
					 self.symbolTbl.className.clone(),
					 l_num));
	
	//(else {statement})?
	if self.is_keyword("else"){
	    //else
	    self.print_type_token();
	    self.tokens.advance();
	    
	    // '{'
	    self.compileSymbol("{", "at compileIf");

	    //statements
	    self.compileStatements();

	    // '}'
	    self.compileSymbol("}", "at compileIf");
	}
	
	//label L2
	self.vmwriter.writeLabel(format!("{}.L2_{}",
					 self.symbolTbl.className.clone(),
					 l_num));

	self.writeLine("</ifStatement>");
    }

    pub fn compileWhile(&mut self){
	self.writeLine("<whileStatement>");

	//count L1, L2 and increment
	let l_num:u16 = self.symbolTbl.countL();
	
	//while
	self.print_type_token();
	self.tokens.advance();

	//label L1
	self.vmwriter.writeLabel(format!("{}.L1_{}",
					 self.symbolTbl.className.clone(),
					 l_num));

	// '('
	self.compileSymbol("(", "at compileIf");

	// expression
	self.compileExpression();

	// ')'
	self.compileSymbol(")", "at compileIf");

	//~expression
	self.vmwriter.writeArithmetic("NOT".to_string());
	//if-goto L2
	self.vmwriter.writeIf(format!("{}.L2_{}",
				      self.symbolTbl.className.clone(),
				      l_num));

	// '{'
	self.compileSymbol("{", "at compileIf");

	//statements
	self.compileStatements();

	// '}'
	self.compileSymbol("}", "at compileIf");

	//goto L1
	self.vmwriter.writeGoto(format!("{}.L1_{}",
					self.symbolTbl.className.clone(),
					l_num));

	//label L2
	self.vmwriter.writeLabel(format!("{}.L2_{}",
					 self.symbolTbl.className.clone(),
					 l_num));

	self.writeLine("</whileStatement>");
    }
    
    pub fn compileDo(&mut self){
	self.writeLine("<doStatement>");
	//do
	self.print_type_token();
	self.tokens.advance();

	//subroutineCall
	self.compileSubroutineCall();

	//ignore return value
	self.vmwriter.writePop("temp".to_string(), 0);

	// ';'
	self.compileSymbol(";", "at compileDo");
	
	self.writeLine("</doStatement>");
    }

    pub fn compileReturn(&mut self){
	self.writeLine("<returnStatement>");
	
	//return
	self.print_type_token();
	self.tokens.advance();
	
	//expression
	if self.is_term(){
	    self.compileExpression();
	}

	// ';'
	self.compileSymbol(";", "at compile return");

	/*
	if (self.symbolTbl.kind_buf == "method".to_string()){
	    self.vmwriter.writePush("pointer".to_string(), 0);
	}
	*/
        //if function void write push constant 0
	if self.void_flag{
	    self.vmwriter.writePush("constant".to_string(), 0);
	}
	//write return
	self.vmwriter.writeReturn();
	
	self.writeLine("</returnStatement>");
    }

    pub fn compileExpression(&mut self){
	self.writeLine("<expression>");

	//term
	if self.is_term(){
	    self.compileTerm();
	}

	while self.is_op(){
	    //Op
	    self.print_type_token();
	    let Op: String = self.tokens.symbol();
	    self.tokens.advance();

	    //term
	    self.compileTerm();

	    //write op
	    match &Op[..]{
		"+" => {self.vmwriter.writeArithmetic("ADD".to_string());}
		"-" => {self.vmwriter.writeArithmetic("SUB".to_string());}
		"*" => {self.vmwriter.writeCall("Math.multiply".to_string(), 2);}
		"/" => {self.vmwriter.writeCall("Math.divide".to_string(), 2);}
		"&" => {self.vmwriter.writeArithmetic("AND".to_string());}
		"|" => {self.vmwriter.writeArithmetic("OR".to_string());}
		"<" => {self.vmwriter.writeArithmetic("LT".to_string());}
		">" => {self.vmwriter.writeArithmetic("GT".to_string());}
		"=" => {self.vmwriter.writeArithmetic("EQ".to_string());}
		_ => {panic!("compileExpression undefined op");}
	    }

	}
	self.writeLine("</expression>");
    }



    /*original compile method*/
    pub fn compileSymbol(&mut self, symbol: &str, err_msg: &str){
	if self.is_symbol(symbol){
	    self.print_type_token();
	    self.tokens.advance();
	}
	else{
	    println!("current token : {} {}", self.tokens.current_token, &self.tokens.tokenType()[..]);
	    panic!("CompilationEngine:: missing symbol '{}' msg:{}", symbol, err_msg)
	}
    }

    pub fn compileIdentifier(&mut self, err_msg: &str){
	if self.is_identifier(){
	    //advance token
	    self.print_type_token();
	    self.tokens.advance();
	}
	else{
	    println!("current token : {} {}", self.tokens.current_token, &self.tokens.tokenType()[..]);
	    panic!("CompilationEngine:: {}", err_msg);
	}
    }

    pub fn compileType(&mut self, err_msg: &str){
	if self.is_type() {
	    self.print_type_token();
	    self.tokens.advance();
	}
	else{
	    println!("current token : {} {}", self.tokens.current_token, &self.tokens.tokenType()[..]);
	    panic!("CompilationEngine:: {}", err_msg)
	}
    }

    pub fn compileSubroutineCall(&mut self){
	//sakiyomi
	//subroutineName(expressionList) or (className | varName).subroutineName
	self.tokens.advance();

	// kind:method (in class)
	// subroutineName (expressionList)
	if self.is_symbol("("){
	    self.tokens.back();

	    // subroutineName
	    self.symbolTbl.subroutineName = self.tokens.identifier();
	    self.compileIdentifier("compileSubroutineCall subrtoutineName or className|varName ?");

	    //push THIS as argment 0
	    self.vmwriter.writePush("pointer".to_string(), 0);

	    // '('
	    self.compileSymbol("(", "at compileSubroutineCall");

	    //push args
	    //expressionList 
	    let nArgs = self.compileExpressionList();

	    // ')'
	    self.compileSymbol(")", "at compileSubroutineCall");

	    //write call
	    self.vmwriter.writeCall(format!("{}.{}",self.symbolTbl.className,self.symbolTbl.subroutineName.clone()),nArgs+1);	
	}

	// (className | varName) . subroutineName (expressionList)
	else if self.is_symbol("."){
	    self.tokens.back();
	    let class_buf:String;
	    
	    // kind: method  (another class)
	    // if varName.subroutineName(expressionList)
	    if self.symbolTbl.isDefined(self.tokens.identifier()){
		let class_buf1 = self.symbolTbl.typeOf();
		self.compileIdentifier("compileSubroutineCall subrtoutineName or className|varName ?");
		
		// '.'
		self.compileSymbol(".", "at compileSubroutineCall");
		
		//subroutineName
		let subroutineName1 = self.tokens.identifier();
		self.compileIdentifier("compileSubroutineCall subroutineName?");

		//push THIS as argment 0
		self.vmwriter.writePush(self.symbolTbl.segmentOf(), self.symbolTbl.indexOf());
		
		// '('
		self.compileSymbol("(", "at compileSubroutineCall");

		//push args
		//expressionList
		let nArgs = self.compileExpressionList();
		
		// ')'
		self.compileSymbol(")", "at compileSubroutineCall");
		
		//write call
		self.vmwriter.writeCall(format!("{}.{}",class_buf1,subroutineName1),nArgs+1);
	    }

	    //kind function
	    //if className
	    else{
		let class_buf2 = self.tokens.identifier();
		self.compileIdentifier("compileSubroutineCall subrtoutineName or className|varName ?");

		// '.'
		self.compileSymbol(".", "at compileSubroutineCall");
		
		//subroutineName
		let subroutineName2 = self.tokens.identifier();
		self.compileIdentifier("compileSubroutineCall subroutineName?");

		// '('
		self.compileSymbol("(", "at compileSubroutineCall");

		//expressionList
		let nArgs = self.compileExpressionList();
		
		// ')'
		self.compileSymbol(")", "at compileSubroutineCall");

		//write call
		self.vmwriter.writeCall(format!("{}.{}",class_buf2,subroutineName2),nArgs);
	    }
	}

	else{
	    println!("current token : {} {}", self.tokens.current_token, &self.tokens.tokenType()[..]);
	    panic!("CompilationEngine:: {}", "compileSubroutineCall not in rule");
	}
    }

    pub fn compileTerm(&mut self){
	self.writeLine("<term>");

	//integerConstant
	if &self.tokens.tokenType()[..] == "INT_CONST"{
	    print!("<integerConstant>");
	    print!("{}", self.tokens.intVal());
	    println!("</integerConstant>");
	    //write
	    self.vmwriter.writePush("constant".to_string(), self.tokens.intVal());

	    self.tokens.advance();
	}

	//stringConstant
	else if &self.tokens.tokenType()[..] == "STRING_CONST"{
	    let string_buf = &self.tokens.stringVal()[..];

	    print!("<stringConstant>");
	    print!("{}", string_buf);
	    println!("</stringConstant>");

	    let size_usize: usize = string_buf.chars().count();
	    let size_u16: u16 = TryFrom::try_from(string_buf.chars().count()).unwrap();
	    self.vmwriter.writePush("constant".to_string(), size_u16);
	    self.vmwriter.writeCall("String.new".to_string(), 1);
	    for i in 0..size_usize{
		self.vmwriter.writePush("constant".to_string(), string_buf.as_bytes()[i] as u16);
		self.vmwriter.writeCall("String.appendChar".to_string(), 2);
	    }

	    self.tokens.advance();
	}

	//keywordConstant
	else if self.is_keywordConstant(){
	    match &self.tokens.keyWord()[..]{
		"TRUE" => {
		    self.vmwriter.writePush("constant".to_string(), 1);
		    self.vmwriter.writeArithmetic("NEG".to_string());
		}
		"FALSE" => {
		    self.vmwriter.writePush("constant".to_string(), 0);
		}
		"NULL" => {
		    self.vmwriter.writePush("constant".to_string(), 0);
		}
		"THIS" => {
		    self.vmwriter.writePush("pointer".to_string(), 0);
		}
		_ => {panic!("undefined keywordConst")}
	    }
	    self.print_type_token();
	    self.tokens.advance();
	}

	// (expression)
	else if self.is_symbol("("){
	    // '('
	    self.compileSymbol("(", "at compileTerm");

	    //expression
	    self.compileExpression();

	    // ')'
	    self.compileSymbol(")", "at compileTerm");
	}

	// unaryOp term
	else if self.is_unaryOp(){
	    // '-' or '~'
	    self.print_type_token();
	    let unaryOp:String = self.tokens.identifier();
	    self.tokens.advance();

	    //term
	    self.compileTerm();

	    //write unaryOp
	    match &unaryOp[..]{
		"-" =>{self.vmwriter.writeArithmetic("NEG".to_string());}
		"~" =>{self.vmwriter.writeArithmetic("NOT".to_string());}
		_  =>{panic!("undefined unaryOp")}
		    
	    }
	}


	// varName or varName[expression] or subroutineCall
	//subroutineCall : subroutineName (expressionList)
	//             or  (className|varName).subrtouineName(expressionList)
	else if self.is_identifier(){
	    //sakiyomi
	    self.tokens.advance();
	    
	    //varName[expression]
	    if self.is_symbol("["){
		self.tokens.back();
		
		//varName
		if self.symbolTbl.isDefined(self.tokens.identifier()){
		    self.vmwriter.writePush(self.symbolTbl.segmentOf(), self.symbolTbl.indexOf());//segment, index
		}
		else{
		    panic!("undefiend symbol(identifier)")
		}
		self.compileIdentifier("compileTerm varName ?");
		
		// '['
		self.compileSymbol("[", "at compileTerm varName[]");

		//expression
		self.compileExpression();

		// ']'
		self.compileSymbol("]", "at compileTerm varName[]");

		//add varName + expression
		self.vmwriter.writeArithmetic("ADD".to_string());

		//pop addr to pointer 1
		self.vmwriter.writePop("pointer".to_string(), 1);

		//push varName[exprresion]
		self.vmwriter.writePush("that".to_string(), 0);
	    }

	    //subroutineCall
	    else if self.is_symbol("(")||self.is_symbol("."){
		self.tokens.back();
		self.compileSubroutineCall();
	    }

	    //varName
	    else{
		self.tokens.back();
		
		//varName
		if self.symbolTbl.isDefined(self.tokens.identifier()){
		    self.vmwriter.writePush(self.symbolTbl.segmentOf(), self.symbolTbl.indexOf());//segment, index
		}
		else{
		    panic!("undefiend symbol(identifier)")
		}
		self.compileIdentifier("compileTerm varName ?");
	    }
	}
	self.writeLine("</term>");	
    }

    pub fn compileExpressionList(&mut self) -> u16{
	self.writeLine("<expressionList>");
	let mut nargs:u16 = 0;

	//  (expression (, expression)* )?
	if self.is_term(){
	    self.compileExpression();
	    nargs += 1;
	    
	    while self.is_symbol(","){
		self.compileSymbol(",", "at compileExpressionList");
		if self.is_term(){
		    self.compileExpression();
		    nargs += 1;
		}
		else{
		    panic!("compileExpressionList term?");
		}
	    }
	}
	self.writeLine("</expressionList>");
	return nargs;
    }

    
    /*original checker*/
    
    pub fn is_keyword(&self, keyword: &str) -> bool{
	&self.tokens.tokenType()[..] == "KEYWORD" && &self.tokens.keyWord()[..].to_lowercase() == keyword
    }

    pub fn is_symbol(&self, symbol: &str) -> bool{
	&self.tokens.tokenType()[..] == "SYMBOL" && &self.tokens.symbol() == symbol
    }

    pub fn is_identifier(&self) -> bool{
	&self.tokens.tokenType()[..] == "IDENTIFIER"
    }

    pub fn is_classVarDec(&self) -> bool{
	self.is_keyword("static") || self.is_keyword("field")
    }

    pub fn is_VarDec(&self) -> bool{
	self.is_keyword("var")
    }

    pub fn is_subroutineDec(&self) -> bool{
	self.is_keyword("constructor") || self.is_keyword("function") || self.is_keyword("method")
    }

    pub fn is_type(&self) -> bool{
	self.is_keyword("int") || self.is_keyword("char") || self.is_keyword("boolean")
	    ||self.is_identifier()
    }

    pub fn is_statement(&self) -> bool{
	self.is_keyword("let")||self.is_keyword("if")||self.is_keyword("while")
	    ||self.is_keyword("do")||self.is_keyword("return")
    }

    pub fn is_keywordConstant(&self) -> bool{
	self.is_keyword("true")||self.is_keyword("false")
	    ||self.is_keyword("null")||self.is_keyword("this")
    }

    pub fn is_unaryOp(&self) -> bool{
	self.is_symbol("-")||self.is_symbol("~")
    }

    pub fn is_term(&self) -> bool{
	self.is_keywordConstant()
	    ||&self.tokens.tokenType()[..] == "INT_CONST"
	    ||&self.tokens.tokenType()[..] == "STRING_CONST"
	    ||self.is_identifier()
	    ||self.is_unaryOp()
	    ||self.is_symbol("(")
    }

    pub fn is_op(&self) -> bool{
	self.is_symbol("+")||self.is_symbol("-")||self.is_symbol("*")||self.is_symbol("/")
	    ||self.is_symbol("&")||self.is_symbol("|")||self.is_symbol("<")||self.is_symbol(">")
	    ||self.is_symbol("=")
    }

    pub fn print_type_token(&mut self) -> (){
	let ty: String = self.tokens.tokenType().to_lowercase();
	let token: String = self.tokens.current_token.to_string();
	print!("<{}>", ty);
	print!("{}", token);
	println!("</{}>", ty);
    }

    pub fn writeLine(&mut self, word: &str) -> (){
	println!("{}", word);
    }

}
