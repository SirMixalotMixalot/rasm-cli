use std::path::Path;
use std::collections::HashMap;
pub mod exec;
pub struct SymbolTable {
    table : HashMap<String,usize>,
    pub labels : usize,

    
}
impl SymbolTable {
    pub fn new() -> Self {
        Self {
            table : HashMap::new(),
            labels : 0,
        }
    }
    pub fn add_var(&mut self,ident : String) {
        let l = self.table.len();
        if !self.table.contains_key(&ident) {
            self.table.insert(ident, l);
        }
    }
    pub fn add_label(&mut self,label : String,line : usize) {
        self.table.insert(label,line);
        self.labels += 1;
    }
    pub fn get(&self,key : String) -> usize {
        *self.table.get(&key).unwrap()
    }
}

pub fn run(file : &Path) {
    let (table,code) = build_table(&file); 
    println!("{:?}",code);
    exec::execute(code,table);
}

fn build_table(file : &Path) -> (SymbolTable,Vec<String>) {
    let mut table = SymbolTable::new();
    let mut code = Vec::new();
    let raw_bytes = std::fs::read(file).unwrap();
    let file_contents = String::from_utf8_lossy(&raw_bytes);
    for (index,line) in file_contents.lines().enumerate() {
        // println!("{}",line);
       //remove comments
       //find symbols and map them to address
       //add actual lines of code 
       //LABELS:
       //   Labels need to be added to the table with the line they are on
       //Variables:
       //   address aliases need to be added to table with offset to bottom of code 
       //Address literals:
       //   Address literals start with # and are left as normal
       //Comments:
       //   Comments should start with ';' and are removed from the code
       
        let mut line = line;
        if let Some(n) = line.find(";") {
            if n == 0 {
                eprintln!("Comment found on line {}",index);
                continue;
            }
            line = line.split_at(n).0;
        }
        if (!line.starts_with("\t") && !line.starts_with(" ")) && line.ends_with(":") {
            println!("Adding label");
            table.add_label(line.to_string(),index);
            continue;
        }
        code.push(line.chars().skip_while(|c| !c.is_alphabetic()).collect());
        //tabxxxspace---
        let x = line.chars()
            .take_while(|c| !c.is_alphabetic())
            .count();
        if line.contains("IN") || line.contains("OUT") {
            continue;
        }
        let n = line.chars()
                .skip(x)
                .take_while(|c| c.is_alphabetic())
                .count();
        let ident = line.split_at(x + n).1;
            
        println!("Ident found {}",ident);
    
        if !ident.starts_with("#") {
            
            table.add_var(ident.to_string());
        }

    }
    (table,code)
}
