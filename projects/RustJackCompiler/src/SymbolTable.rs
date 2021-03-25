use std::collections::HashMap;

pub struct SymbolTable{
    pub class_table: HashMap<String, (String, String, u16)>,
    pub sub_table: HashMap<String, (String, String, u16)>,
    pub current_symbol: (String, String, u16),
    // (kind, type, index)

    pub className: String,
    pub subroutineName: String,
    pub kind_buf: String,
    pub type_buf: String,
    pub symbol_buf: String,
    pub nStatic: u16,
    pub nField: u16,
    pub nArg: u16,
    pub nVar: u16,

    pub nL: u16,

}


impl SymbolTable{
    pub fn new() -> SymbolTable{
	SymbolTable{
	    class_table: HashMap::new(),
	    sub_table: HashMap::new(),
	    current_symbol: ("".to_string(), "".to_string(), 0),
	    className: "".to_string(),
	    subroutineName: "".to_string(),
	    kind_buf: "".to_string(),
	    type_buf: "".to_string(),
	    symbol_buf: "".to_string(),
	    nStatic: 0,
	    nField: 0,
	    nArg: 0,
	    nVar: 0,
	    nL: 0,
	}
    }

    pub fn startSubroutine(&mut self) -> (){
	let table_init =  HashMap::new();
	self.sub_table = table_init.clone();
	self.nArg = 0;
	self.nVar = 0;
    }

    pub fn define(&mut self) -> (){
	println!("define");
	match &self.kind_buf[..]{
	    "static" => {
		self.class_table.insert(self.symbol_buf.clone(),
					(self.kind_buf.clone(), self.type_buf.clone(), self.nStatic));
		self.nStatic += 1;
	    }
	    
	    "field" => {
		self.class_table.insert(self.symbol_buf.clone(),
					(self.kind_buf.clone(), self.type_buf.clone(), self.nField));
		self.nField += 1;
	    }
	    
	    "arg" => {
		self.sub_table.insert(self.symbol_buf.clone(),
				      (self.kind_buf.clone(), self.type_buf.clone(), self.nArg));
		self.nArg += 1;
	    }
	    "var" => {
		self.sub_table.insert(self.symbol_buf.clone(),
				      (self.kind_buf.clone(), self.type_buf.clone(), self.nVar));
		self.nVar += 1;
	    }
	    
	    _ => panic!("undefined kind of symbol")
	}
    }

    pub fn isDefined(&mut self, name: String) -> bool {
	if self.sub_table.get(&name.to_string()).is_some(){
	    self.current_symbol = self.sub_table.get(&name.to_string()).unwrap().clone();
	    true
	}
	else{
	    if self.class_table.get(&name.to_string()).is_some(){
		self.current_symbol = self.class_table.get(&name.to_string()).unwrap().clone();
		true
	    }
	    else{
		false
	    }
	}
    }

    pub fn kindOf(&self) -> String{
	return self.current_symbol.0.clone()
    }

    pub fn typeOf(&self) -> String{
	return self.current_symbol.1.clone()
    }

    pub fn indexOf(&self) -> u16{
	return self.current_symbol.2.clone()
    }

    pub fn segmentOf(&self) -> String{
	match &self.current_symbol.0.clone()[..]{
	    "static" => {"static".to_string()}
	    "field" => {"this".to_string()}
	    "arg" => {"argument".to_string()}
	    "var" =>{"local".to_string()}
	    _ => {panic!("undefined kind of segment")}
	}
    }

    pub fn countL(&mut self) -> u16{
	let buf = self.nL;
	self.nL += 1;
	buf
    }

}
