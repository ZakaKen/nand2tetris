use std::fs::File;
use std::io::{BufWriter, Write};
use JackTokenizer;

pub struct CompilationEngine{
    pub tokens: JackTokenizer::JackTokenizer,
    pub filename:String,
    pub writer: BufWriter<File>,
    pub comp_count: usize,
    pub ret_count: usize,
}

impl CompilationEngine{
    pub fn new(tokens: JackTokenizer::JackTokenizer, out_file_path: String) -> CompilationEngine{
	//out_file_path = /dir/out_file_name.xml

	let mut trimed: Vec<&str> = out_file_path.split("/").collect();
	println!("CompilationEngine -> {}", trimed.last().unwrap());
	
	CompilationEngine{
	    tokens: tokens,
	    filename: "".to_string(),
	    writer: BufWriter::new(File::create(out_file_path).unwrap()),
	    comp_count: 0,
	    ret_count: 0,
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
	self.print_type_token();
	self.tokens.advance();

	//type
	self.compileType("compileVarDec type?");

	//varName
	self.compileIdentifier("compileVarDec varName ?");

	// (, varname)*
	while self.is_symbol(","){
	    // ','
	    self.print_type_token();
	    self.tokens.advance();

	    //varName
	    self.compileIdentifier("compileVarDec varName ?");
	}

	// ;
	self.compileSymbol(";", "at compileVarDec");
	
	self.writeLine("</classVarDec>");
	
    }

    pub fn compileVarDec(&mut self){	
	self.writeLine("<varDec>");
	//var
	self.print_type_token();
	self.tokens.advance();

	//type
	self.compileType("compileVarDec type?");

	//varName
	self.compileIdentifier("compileVarDec varName ?");

	// (, varname)*
	while self.is_symbol(","){
	    // ','
	    self.print_type_token();
	    self.tokens.advance();

	    //varName
	    self.compileIdentifier("compileVarDec varName ?");
	}

	// ;
	self.compileSymbol(";", "at compileVarDec");
	
	self.writeLine("</varDec>");
	
    }

    pub fn compileSubroutine(&mut self){
	self.writeLine("<subroutineDec>");
	self.print_type_token();
	self.tokens.advance();

	//void | type
	if self.is_keyword("void") || self.is_type() {
	    self.print_type_token();
	    self.tokens.advance();
	}
	else{
	    println!("current token : {} {}", self.tokens.current_token, &self.tokens.tokenType()[..]);
	    panic!("CompilationEngine:: compileSubroutine void | type ?")
	}

	//subroutineName
	self.compileIdentifier("compileSubroutine subroutineName ?");

	//'('
	self.compileSymbol("(", "at compileSubroutine");
	
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
	    //type
	    self.print_type_token();
	    self.tokens.advance();

	    //varname
	    self.compileIdentifier("compileParameterList varName ?");
	}

	//, type varname
	while self.is_symbol(","){
	    // ','
	    self.print_type_token();
	    self.tokens.advance();

	    //type
	    self.compileType("compileParameterList type ?");

	    //varname
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

	//varName
	self.compileIdentifier("compileLet varName?");

	// ('[' expression ']') ?
	if self.is_symbol("["){
	    // '['
	    self.compileSymbol("[", "at compileLet");

	    //expression
	    self.compileExpression();

	    // ']'
	    self.compileSymbol("]", "at compileLet");
	}

	// '='
	self.compileSymbol("=", "at compileLet");

	//expression
	self.compileExpression();

	// ';'
	self.compileSymbol(";", "at compileLet");

	self.writeLine("</letStatement>")
    }

    pub fn compileIf(&mut self){
	self.writeLine("<ifStatement>");
	//if
	self.print_type_token();
	self.tokens.advance();

	// '('
	self.compileSymbol("(", "at compileIf");

	// expression
	self.compileExpression();

	// ')'
	self.compileSymbol(")", "at compileIf");

	// '{'
	self.compileSymbol("{", "at compileIf");

	//statements
	self.compileStatements();

	// '}'
	self.compileSymbol("}", "at compileIf");
	
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
	
	self.writeLine("</ifStatement>");
    }

    pub fn compileWhile(&mut self){
	self.writeLine("<whileStatement>");
	//while
	self.print_type_token();
	self.tokens.advance();

	// '('
	self.compileSymbol("(", "at compileIf");

	// expression
	self.compileExpression();

	// ')'
	self.compileSymbol(")", "at compileIf");

	// '{'
	self.compileSymbol("{", "at compileIf");

	//statements
	self.compileStatements();

	// '}'
	self.compileSymbol("}", "at compileIf");
	
	self.writeLine("</whileStatement>");
    }
    
    pub fn compileDo(&mut self){
	self.writeLine("<doStatement>");
	//do
	self.print_type_token();
	self.tokens.advance();

	//subroutineCall
	self.compileSubroutineCall();

	// ';'
	self.compileSymbol(";", "at compileDo");
	
	self.writeLine("</doStatement>");
    }

    pub fn compileReturn(&mut self){
	self.writeLine("<returnStatement>");
	//return
	self.print_type_token();
	self.tokens.advance();
	
	//expression? test
	if self.is_term(){
	    self.compileExpression();
	}

	// ';'
	self.compileSymbol(";", "at compile return");
	
	self.writeLine("</returnStatement>");
    }

    pub fn compileExpression(&mut self){
	//simple mode. compile identifier
	self.writeLine("<expression>");

	//term
	if self.is_term(){
	    self.compileTerm();
	}

	while self.is_op(){
	    //Op
	    self.print_type_token();
	    self.tokens.advance();

	    //term
	    self.compileTerm();
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
	//subroutineCall atode

	//subroutineName or (className | varName)
	self.compileIdentifier("compileSubroutineCall subrtoutineName or className|varName ?");

	// subroutineName (expressionList)
	if self.is_symbol("("){
	    // '('
	    self.compileSymbol("(", "at compileSubroutineCall");

	    //expressionList
	    self.compileExpressionList();

	    // ')'
	    self.compileSymbol(")", "at compileSubroutineCall");
	}

	// (className | varName) . subroutineName (expressionList)
	else if self.is_symbol("."){
	    // '.'
	    self.compileSymbol(".", "at compileSubroutineCall");
	    
	    //subroutineName
	    self.compileIdentifier("compileSubroutineCall subroutineName?");

	    // '('
	    self.compileSymbol("(", "at compileSubroutineCall");

	    //expressionList
	    self.compileExpressionList();

	    // ')'
	    self.compileSymbol(")", "at compileSubroutineCall");
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
	    let _ = self.writer.write(format!("<integerConstant> {} </integerConstant>\n",
					      self.tokens.intVal()).as_bytes());
	    self.tokens.advance();
	}

	//stringConstant
	else if &self.tokens.tokenType()[..] == "STRING_CONST"{
	    print!("<stringConstant>");
	    print!("{}", self.tokens.stringVal());
	    println!("</stringConstant>");
	    let _ = self.writer.write(format!("<stringConstant> {} </stringConstant>\n",
					      self.tokens.stringVal()).as_bytes());

	    self.tokens.advance();
	}

	//keywordConstant
	else if self.is_keywordConstant(){
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
	    self.tokens.advance();

	    //term
	    self.compileTerm();
	}

	// varName or varName[expression] or subroutineCall
	//subroutineCall : subroutineName (expressionList)
	//             or  (className|varName).subrtouineName(expressionList)
	else if self.is_identifier(){
	    //varName or subroutineName or className|varName
	    self.compileIdentifier("compileTerm identifier1 ?");

	    //varName[expression]
	    if self.is_symbol("["){
		// '['
		self.compileSymbol("[", "at compileTerm varName[]");

		//expression
		self.compileExpression();

		// ']'
		self.compileSymbol("]", "at compileTerm varName[]");
	    }

	    //subroutineName (expressionList)
	    else if self.is_symbol("("){
		// '('
		self.compileSymbol("(", "at compileTerm subroutineName()");

		//expressionList
		self.compileExpressionList();

		// ')'
		self.compileSymbol(")", "at compileTerm subroutineName()");
	    }

	    //(className|varName).subrtouineName(expressionList)
	    else if self.is_symbol("."){
		// '.'
		self.compileSymbol(".", "at compileTerm (className|varName).subroutineName()");

		//sunroutineName
		self.compileIdentifier("compileTerm sunroutineName?");

		// '('
		self.compileSymbol("(", "at compileTerm (className|varName).subroutineName()");

		//expressionList
		self.compileExpressionList();

		// ')'
		self.compileSymbol(")", "at compileTerm (className|varName).subroutineName()");
	    }

	}
	self.writeLine("</term>");	
    }

    pub fn compileExpressionList(&mut self){
	self.writeLine("<expressionList>");

	//  (expression (, expression)* )?
	if self.is_term(){
	    self.compileExpression();

	    while self.is_symbol(","){
		self.compileSymbol(",", "at compileExpressionList");
		if self.is_term(){
		    self.compileExpression();
		}
		else{
		    panic!("compileExpressionList term?");
		}
	    }
	    
	}
	self.writeLine("</expressionList>");
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
	let _ = self.writer.write(format!("<{}> {} </{}>\n", ty, token, ty).as_bytes());
    }

    pub fn writeLine(&mut self, word: &str) -> (){
	println!("{}", word);
	let _ = self.writer.write(format!("{}\n", word).as_bytes());
    }

    pub fn setFileName(&mut self, input_file_path: &String) -> (){
	//input_file_path(Ex /dir/Xxx.jack) -> codewriter.filename = Xxx
	
	let trimed: Vec<&str> = input_file_path.split(".jack").collect();
	let mut filename: String = trimed[0].to_string();
	let trimed_buf: Vec<&str> = filename.split("/").collect();
	filename = trimed_buf.last().unwrap().to_string();
	
	self.filename = filename;
	println!("load {}.jack", self.filename);
    }
}
