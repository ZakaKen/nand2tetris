use std::convert::TryFrom;

pub struct SymbolTable{
    pub symbol: Vec<String>,
    pub address: Vec<u16>,
}

impl SymbolTable{
    pub fn new() -> SymbolTable{
	//instace init
	SymbolTable{
	    symbol:vec!["SP".to_string(), "LCL".to_string(), "ARG".to_string(),
			"THIS".to_string(), "THAT".to_string(),
			"R0".to_string(), "R1".to_string(), "R2".to_string(),
			"R3".to_string(), "R4".to_string(), "R5".to_string(),
			"R6".to_string(), "R7".to_string(), "R8".to_string(),
			"R9".to_string(), "R10".to_string(), "R11".to_string(),
			"R12".to_string(), "R13".to_string(), "R14".to_string(),
			"R15".to_string(), "SCREEN".to_string(), "KBD".to_string()],
	    
	    address:vec![0, 1, 2, 3, 4,
			 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
			 16384, 24576],
	}
    }

    pub fn addEntry(&mut self, symbol: String, address:u16) -> (){
	self.symbol.push(symbol);
	self.address.push(address);
    }

    pub fn contains(&self, symbol: String) -> bool{
	self.symbol.contains(&symbol)
    }

    pub fn getAddress(&self, symbol: String) -> u16{
	let mut iter = self.symbol.iter();
	let mut index_usize = iter.position(|x| x == &symbol).unwrap();
	index_usize = TryFrom::try_from(index_usize).unwrap();
	self.address[index_usize]
	    
    }

    
}
