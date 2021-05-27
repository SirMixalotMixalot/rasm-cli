use std::{cmp::{max, min}, path::Path};
use std::collections::HashMap;
use super::EnvArgs;
pub mod exec;
pub struct SymbolTable {
    table : HashMap<String,usize>,
    pub labels : usize,
    min_addr : u16,
    max_addr : u16,
    pub num_vars : usize,

    
}
impl SymbolTable {
    pub fn new() -> Self {
        Self {
            table : HashMap::new(),
            labels : 0,
            min_addr : 0,
            max_addr : 0,
            num_vars : 0,
        }
    }
    pub fn add_var(&mut self,ident : String) {
        if ident.as_str() == "ACC" || ident.as_str() == "IX" {
            return;
        }
        let l = self.table.len();
        if !self.table.contains_key(&ident) {
            self.table.insert(ident, l);
            self.num_vars += 1;
        }
    }
    pub fn add_label(&mut self, k : String, v : usize) {
        self.table.insert(k, v);
        self.labels += 1;
    }
    pub fn get(&self,key : String) -> u16 {
        (match key.as_str() {
            "ACC" => 0,
            "IX"  => 1,
             _    => *self.table.get(&key).unwrap(),
        }) as u16
        
        
    }
}



pub struct Code{
    table : SymbolTable,
    pub code : Vec<Instruction>,
    pub debug_info : Vec<(usize,String)>
}
impl Code {
    pub fn new(table : SymbolTable,code : Vec<Instruction>,debug_info : Vec<(usize,String)>) -> Self {
        Self {table,code,debug_info}
    }
    pub fn get(&self,i : usize) -> Option<(&Instruction,&(usize,String))>
    {
        if let Some(v) = self.code.get(i) {
            Some((v,&self.debug_info[i]))
        }
        else {
            None
        }
    }
}

pub fn run(args : EnvArgs) {
    let code = build_code(&args.file); 
    exec::execute(code,args.style);
}

#[derive(Clone, Copy)]
pub enum AdrMode {
    Indexed,
    Immediate,
    Direct,
    Indirect,
}



//#[derive(Debug)]
pub enum Instruction {
    IO(bool),
    LOAD {data : u16, adr_mode : AdrMode},
    LDR(i16), //Load into index register <imm> 
    MOV     , //ACC -> IX lol 
    SUB {rhs : u16, adr_mode : AdrMode},
    STO(u16),
    ADD {rhs : u16, adr_mode : AdrMode},
    INC(bool), //true:ACC, false:IX
    DEC(bool), //true:ACC, false:IX
    LSL(u16),  //logical shift left 
    LSR(u16),  //logical shift right
    XOR {rhs : u16, adr_mode : AdrMode},
    OR {rhs : u16, adr_mode : AdrMode},
    CMP {rhs : u16, adr_mode : AdrMode},
    JPE {addr : u16, adr_mode : AdrMode},
    JPN {addr : u16, adr_mode : AdrMode},
    END,
    UNKNOWN,


}
impl Instruction {
    pub fn new(opcode : String,rhs : u16) -> Self {
        //handle sto
        //all addresses have to be offset
        //TODO:handle acc and ix in a better way?
        if &opcode == "STO" {
            return Self::STO(rhs)
        }
        let adr_mode = AdrMode::Direct;
        match opcode.chars().nth(0).unwrap() {
            'L' => Instruction::load_instruction(opcode,rhs),
            'S' => Instruction::SUB {rhs,adr_mode},
            'A' => Instruction::ADD {rhs,adr_mode},
            'I' => Instruction::INC(rhs == 0), //Acc is 0 in symtable
            'D' => Instruction::DEC(rhs == 0),
            'C' => Instruction::CMP{rhs,adr_mode},
            'X' => Instruction::XOR{rhs,adr_mode},
            'O' => Instruction::OR {rhs,adr_mode},
            'M' => Instruction::MOV,
            'J' => Instruction::jmp_instruction(opcode,rhs,adr_mode),
            'E' => Instruction::END,
             _  => Instruction::UNKNOWN,

        }
    }
    pub fn with_imm(opcode : String, rhs : u16) -> Self {
        let first_char = opcode.chars().nth(0).unwrap();
        let adr_mode = AdrMode::Immediate;
        match first_char {
            'L' => {
                if opcode.chars().nth(1).unwrap() == 'S' {
                    if opcode.chars().nth(2).unwrap() == 'R' {
                        Instruction::LSR(rhs)
                    }else {
                        Instruction::LSL(rhs)
                    }   
                }else {
                    Self::load_instruction(opcode,rhs)
                }
            },

            'S' => Instruction::SUB{rhs,adr_mode},
            'A' => Instruction::ADD{rhs,adr_mode},
            'C' => Instruction::CMP{rhs,adr_mode},
            'X' => Instruction::XOR{rhs,adr_mode},
            'O' => Instruction::OR{rhs,adr_mode},
            'J' => Self::jmp_instruction(opcode,rhs,adr_mode),
             _  => Instruction::UNKNOWN,

        }
    }
    fn load_instruction(opcode : String,data : u16) -> Self {
        match opcode.chars().last().expect("Error on load instruction") {
            'D' => Instruction::LOAD{data,adr_mode : AdrMode::Direct},
            'M' => Instruction::LOAD{data,adr_mode : AdrMode::Immediate},
            'R' => Instruction::LDR(data as i16), //Loads into register
            'I' => Instruction::LOAD{data, adr_mode : AdrMode::Indirect},
            'X' => Instruction::LOAD{data, adr_mode : AdrMode::Indexed},
             _  => Instruction::UNKNOWN,
        }
    }
    fn jmp_instruction(opcode : String,addr : u16,adr_mode : AdrMode) -> Self {
    
        match opcode.chars().nth(2).expect("Error on Jump instruction") {
            'E' => Instruction::JPE {addr,adr_mode},
            'N' => Instruction::JPN {addr,adr_mode},
            
             _  => Instruction::UNKNOWN,
        }
    }
    
}
fn build_code(file : &Path) -> Code {
    let mut table = SymbolTable::new();
    let mut code = Vec::new();
    let mut debug_info = Vec::new();
    let raw_bytes = std::fs::read(file).unwrap();
    let file_contents = String::from_utf8_lossy(&raw_bytes);
    for (index,line) in file_contents.lines().enumerate() {
       //LABELS:
       //   Labels need to be added to the table with the line they are on
       //Variables:
       //   address aliases need to be added to table with offset to bottom of code 
       //Address literals:
       //   Address literals start with # and are left as normal
       //Comments:
       //   Comments should start with ';' and are removed from the code
        
        let mut line = line.trim();
        if let Some(n) = line.find(";") {
            if n == 0 {
                continue;
            }
            line = line.split_at(n).0;
        }
        if line == "END" {
            code.push(Instruction::END);
            debug_info.push((index,line.to_string()));
            continue;
        }
        if line.ends_with(":") {
            let pos = line.rfind(":").unwrap();
            table.add_label(line[..pos].trim().to_string(),index);
            continue;
        }
        //code.push(line.chars().skip_while(|c| !c.is_alphabetic()).collect());
        //tabxxxspace---
        if line.ends_with("IN") || line.ends_with("OUT") {
            code.push(Instruction::IO(line.ends_with("IN")));
            debug_info.push((index,line.to_string()));
            continue;
        }
    

        let opcode = line.chars()
                .take_while(|c| c.is_alphabetic())
                .collect::<String>();
        let n = opcode.len(); // presumably 3
        
        let ident = line.split_at(n + 1).1.trim();
        //immediate addresses
        if !ident.starts_with("#") {
        
            let p = ident.parse::<i16>();
            if p.is_err() {
                table.add_var(ident.to_string());
                
                
            }else 
            {
                let p = p.unwrap();
                code.push(Instruction::with_imm(opcode,p as u16));
                table.min_addr = min(table.min_addr, p as u16);
                table.max_addr = max(table.max_addr, p as u16);
                debug_info.push((index,line.to_string()));
                continue;
            }
        }
        //dealing with immediate values
        if ident.starts_with("#") {
            let fstring = &ident[1..];
            let imm  = match fstring.chars().nth(0).unwrap() {
                'B' => i16::from_str_radix(&fstring[2..],2),
                '&' => i16::from_str_radix(&fstring[2..], 16),
                '0'..='9' => i16::from_str_radix(fstring, 10),
                _       => panic!("Immediate value not formatted correctly")
            };
            let imm = imm.expect("Error while parsing immediate value");
            code.push(Instruction::with_imm(opcode.to_string(), imm as u16));
            debug_info.push((index,line.to_string()));
            continue;
        }
        code.push(Instruction::new(opcode,table.get(ident.to_string())));
        debug_info.push((index,line.to_string()));
    }
    Code::new(table,code,debug_info)
}
