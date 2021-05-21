use super::{Code,Instruction};
use std::fmt::{Display,Result,Formatter};

struct CPU {
    acc : i16,
    ix : i16,
    pub pc : u16,
    done : bool,
    flag_register : Flags,
    memory : [i16;10], 
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            acc : 0,
            ix  : 0,
            pc  : 0,
            done : false,
            flag_register : Flags::new(),
            memory : [0;10],
        }
    }
    pub fn done(&self) -> bool {
        self.done
    }
    pub fn execute(&mut self,instr : &Instruction) {
        use Instruction::*;
        self.pc += 1;
        match instr {
            &LDD(addr) => self.ldd(addr),
            &LDM(imm)  => self.ldm(imm),
            &IO(b)     => self.io(b),
            &SUBM(imm) => self.subm(imm),
            &SUBA(addr) => self.suba(addr),
            &STO(addr)  => self.sto(addr),
            &ADDM(imm)  => self.addm(imm),
            &ADDA(addr) => self.adda(addr),
            &INC(b)     => self.inc(b),
            &CMPA(addr)  => self.cmpa(addr),
            &CMPM(imm)   => self.cmpm(imm),
            END         => self.end(),
            UNKNOWN     => eprintln!("Unkown command!"),
            _           => self.jmp(instr),
        }
    }
    fn ldd(&mut self,addr : u16) {
        self.acc = self.memory[addr as usize];
    }
    fn ldm(&mut self, imm : i16) {
        self.acc = imm;
    }
    fn io(&mut self, inp : bool) {
        use std::io::{self,Write};
        if inp {
            
            let stdio = io::stdout();
            let mut handle = stdio.lock();
            handle.write_all(b">").unwrap();
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            self.acc = buffer.chars().nth(0).unwrap() as u32 as i16;
        }else {
            println!("{}",self.acc as u8 as char)
        }
    }
    fn subm(&mut self, imm : i16) {
        let res = self.acc.checked_sub(imm);
        self.flag_register.set_flags(res);
        self.acc -= imm;
    }
    fn suba(&mut self,addr: u16) {
        self.subm(self.memory[addr as usize]);
    }
    fn sto(&mut self,addr : u16) {
        self.memory[addr as usize] = self.acc;
    }
    fn addm(&mut self, imm : i16) {
        
        self.flag_register.set_flags(
                self.acc.checked_add(imm)
            );
        self.acc += imm;
    }
    fn adda(&mut self, addr : u16) {
        self.addm(self.memory[addr as usize]);
    }
    fn inc(&mut self, acc : bool) {
        if acc {
            self.acc += 1;
        }else {
            self.ix += 1;
        }
    }
    fn cmpm(&mut self,imm : i16) {
        self
            .flag_register.
            set_flags(Some(self.acc - imm));
    }
    fn cmpa(&mut self,addr : u16) {
        self.cmpm(self.memory[addr as usize]);
    }
    fn end(&mut self) {
        println!("-------Program ending---------");
        self.done = true;
    }
    fn jmp(&mut self, instr : &Instruction) {
        let ni = match instr {
            &Instruction::JPEM(x) => {
                if self.flag_register.get_flag(FLAGS::Z) {
                    x
                }else {
                    self.pc 
                }
            },
            &Instruction::JPNM(x) => {
                if self.flag_register.get_flag(FLAGS::Z) {
                    self.pc
                }else {
                    x
                }
            },
            &Instruction::JPEA(x) => {
                if self.flag_register.get_flag(FLAGS::Z) {
                    self.memory[x as usize] as u16
                }else {
                    self.pc
                }

            },
            &Instruction::JPNA(x) => {
                if self.flag_register
                    .get_flag(FLAGS::Z) {
                        self.pc
                    }else {
                        self.memory[x as usize] as u16
                    }
            }
            _ => unreachable!()

        };
        self.pc = ni;
    }

}

impl Display for CPU {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
       write!(f, 
r"
 ----------------------------------
|              CPU                 |
|    FLAGS                PC       |
|    ---------           _______   |
|   | N V Z C |         |{:>5}  |  |
|   | {} {} {} {} |          -------   | 
|    ---------                     |
|    ACC                 IX        |
|  ________          _________     |
| |{:>8}|        |{:>8} |    |
|  --------          ---------     |  
 ---------------------------------- ",
                
               self.pc,
               self.flag_register.get_flag(FLAGS::N) as u8,
               self.flag_register.get_flag(FLAGS::V) as u8,
               self.flag_register.get_flag(FLAGS::Z) as u8,
               self.flag_register.get_flag(FLAGS::C) as u8,
               self.acc,
               self.ix)

    }

}



enum FLAGS {
    Z = (1<<3),
    N = (1<<2),
    V = (1<<1),
    C = (1<<0),
}
struct Flags { 
     //zero
    // negative 
   //  overflow
   //   carry                                                   4 3 2 1
     flags : u8, // low 4 bits used to represent flags in order z,n,v,c

}

impl Flags {
    pub fn new() -> Self {
        Self {flags : 0}
    }
    pub fn set_flag(&mut self, flag : FLAGS, cond : bool) {
        
        if cond {
            self.flags |= flag as u8
        }else {
            self.flags &= !(flag as u8)
        }
    }
   pub fn get_flag(&self, flag : FLAGS) -> bool {
        (self.flags & (flag as u8)) != 0 
   }
   pub fn set_flags(&mut self,res : Option<i16>) {
        self.set_flag(FLAGS::V, res.is_none());
        self.set_flag(FLAGS::C, res.is_none());
        self.set_flag(FLAGS::N, res.is_some() && res.unwrap() < 0);
        self.set_flag(FLAGS::Z, res.is_some() && res.unwrap() == 0)

   }

}
//find some use for this
impl Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //  _________
        // | N V Z C |
        // | 1 1 1 1 |
        //  ---------
       let line1 =      " _________";
       let line2 =      " ---------";
       let l2 =         "| N V Z C |";
       let l3 = format!("| {} {} {} {} |",
                self.get_flag(FLAGS::N) as u8,self.get_flag(FLAGS::V) as u8,
                self.get_flag(FLAGS::Z) as u8,self.get_flag(FLAGS::C) as u8
        );
       write!(f,"{}\n{}\n{}\n{}",line1,l2,l3,line2)

    }
}
pub fn execute(code : Code ) {
    use std::io;
    let mut cpu = CPU::new();
    while !cpu.done() {
        let instr = code.get(cpu.pc as usize);
        cpu.execute(&instr);
        println!("{}",cpu);

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
    }
}
