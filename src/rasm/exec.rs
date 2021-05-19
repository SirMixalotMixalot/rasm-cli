use super::SymbolTable;
use std::fmt::Display;

struct CPU {
    acc : u8,
    pc : u16,
    flags : Flags,
}

enum FLAGS {
    Z = (1<<3),
    N = (1<<2),
    V = (1<<1),
    C = (1<<0),
}
struct Flags {
    z : bool,    //zero
    n : bool,   // negative 
    v : bool,  //  overflow
    c : bool,//   carry
    num : u8, // low 4 bits used to represent flags in order z,n,v,c
}

impl Flags {
    pub fn new() -> Self {
        Self {z : false, n : false, v : false, c : false,num : 0}
    }
    pub fn set_flag(&mut self, flag : FLAGS, cond : bool) {
        match flag {
            FLAGS::Z => self.z = cond,
            FLAGS::N => self.n = cond,
            FLAGS::V => self.v = cond,
            FLAGS::C => self.c = cond
        };
        if cond {
            self.num |= flag as u8
        }else {
            self.num &= !(flag as u8)
        }
    }
   pub fn get_flag(&self, flag : FLAGS) -> bool {
        match flag {
            FLAGS::Z => self.z,
            FLAGS::N => self.n,
            FLAGS::V => self.v,
            FLAGS::C => self.c
        }
   }

}
impl Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //  _________
        // | N V Z C |
        // | 1 1 1 1 |
        //  ---------
       let line = "----------";
       let l2 = "| N V Z C |";
       let l3 = format!("| {} {} {} {} |",
                self.get_flag(FLAGS::N) as u8,self.get_flag(FLAGS::V) as u8,
                self.get_flag(FLAGS::Z) as u8,self.get_flag(FLAGS::C) as u8
        );
       write!(f,"{}\n{}\n{}\n{}\n",line,l2,l3,line)

    }
}
pub fn execute(code : Vec<String>,symbols : SymbolTable) {
    
}
