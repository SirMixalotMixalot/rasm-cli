use std::path::Path;
use std::collections::HashMap;
struct SymbolTable {
    table : HashMap<String,usize>,
    pub offset : usize,

    
}
impl SymbolTable {
    pub fn new() -> Self {
        Self {
            table : HashMap::new(),
            offset : 0,
        }
    }
    pub fn add_var(&mut self,ident : String) {
        let l = self.table.len();
        self.table.insert(ident, l - self.offset + 1);
    }
    pub fn add_label(&mut self,label : String,line : usize) {
        self.table.insert(label,line);
        self.offset += 1;
    }
    pub fn get(&self,key : String) -> usize {
        *self.table.get(&key).unwrap()
    }
}

pub fn run(file : &Path) {
    let table = build_table(&file);    
    let code : Vec<String> = replace(&file,&table);
    execute(code);
}
fn execute(code : Vec<String>) {
    
}
fn replace(file : &Path,table : &SymbolTable) -> Vec<String> {
    let code = std::fs::read_to_string(file).unwrap();
    let n = code.len() - table.offset;
    let mut res = Vec::new();
    for line in code.lines() {
        if !line.starts_with("\t") {
            continue;
        }
        let line = line.split_at(5);
        res.push(format!("{}#{}",line.0,table.get(line.1.to_string()) + n));
    }
    res
}
fn build_table(file : &Path) -> SymbolTable {
    let mut table = SymbolTable::new();
    let raw_bytes = std::fs::read(file).unwrap();
    let file_contents = String::from_utf8_lossy(&raw_bytes);
    for (index,line) in file_contents.lines().enumerate() {
       //remove comments
       //find symbols and map them to address
        let mut line = line;
        if let Some(n) = line.find("//") {
            if n == 0 {
                eprintln!("Comment found on line {}",index);
                continue;
            }
            line = line.split_at(n).0;
        }
        if !line.starts_with("\t") {
            table.add_label(line.to_string(),index);
            continue;
        }
        //tabxxxspace---
        let x = "\txxx ".len();
        let ident = line.split_at(x).1;
        table.add_var(ident.to_string());

    }
    table
}
