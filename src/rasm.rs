use std::{cmp::{max, min}, path::Path};
use std::collections::HashMap;
use super::EnvArgs;
pub mod exec;
pub mod cpu;
pub mod computer;
pub mod mem;
pub mod instr;
use instr::{Instruction,AdrMode};
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
        if line.ends_with(":") {
            let pos = line.rfind(":").unwrap();
            table.add_label(line[..pos].trim().to_string(),index);
            continue;
        }
        code.push(str_to_instr(&mut table,line));
        debug_info.push((index,line.to_string()));
    }
    Code::new(table,code,debug_info)
}
pub fn str_to_instr(table : &mut SymbolTable,line : &str) -> Instruction {
    if line == "END" {
         return Instruction::END
      
    }

    //code.push(line.chars().skip_while(|c| !c.is_alphabetic()).collect());
    //tabxxxspace---
    else if line.ends_with("IN") || line.ends_with("OUT") {
         return Instruction::IO(line.ends_with("IN"))
        
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
        }else{
            let p = p.unwrap();
            table.min_addr = min(table.min_addr, p as u16);
            table.max_addr = max(table.max_addr, p as u16);
            return Instruction::with_imm(opcode,p as u16)

        }
    }
    //dealing with immediate values
    if ident.starts_with("#") {
        let fstring = &ident[1..];
        let imm  = match fstring.chars().nth(0).unwrap() {
            'B'       => i16::from_str_radix(&fstring[2..],2),
            '&'       => i16::from_str_radix(&fstring[2..], 16),
            '0'..='9' => i16::from_str_radix(fstring, 10),
            _         => {
                            eprintln!("Incorrectly formated immediate ({})", line);
                            std::process::exit(-1);
            }        
        };
        let imm = imm.expect("Error while parsing immediate value");
        return Instruction::with_imm(opcode, imm as u16)
    }
    Instruction::new(opcode,table.get(ident.to_string()))
    

}
