use super::{Code,Instruction};
use std::fmt::{Display,Result,Formatter};
use std::ops::{Index,IndexMut};
pub struct Memory {
    min_addr : u16,
    mem : Vec<i16>,
}

impl Memory {
    pub fn new(min_addr : u16) -> Self {
        Memory {
            min_addr,
            mem : Vec::with_capacity(16),
        }
    }
}
impl Index<usize> for Memory {
    type Output = i16;
    fn index(&self, index: usize) -> &Self::Output {
        if index as u16 > self.min_addr {
            let i = index - self.min_addr as usize;
            &self.mem[i]
        }else {
            &self.mem[index]
        }
    }
}
impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
         if index as u16 > self.min_addr {
            let i = index - self.min_addr as usize;
            if i >= self.mem.len() {
                self.mem.resize(i + 1,0);
            }
            
            &mut self.mem[i]
        }else {
            if index >= self.mem.len() {
                self.mem.resize(index + 1, 0);
            }
            &mut self.mem[index]
        }
       
    }
}
pub struct CPU {
    acc : i16,
    ix : i16,
    pub pc : u16,
    done : bool,
    flag_register : Flags,
    pub memory : Memory, 
}

impl CPU {
    pub fn new(min_addr : u16) -> Self {
        CPU {
            acc : 0,
            ix  : 0,
            pc  : 0,
            done : false,
            flag_register : Flags::new(),
            memory : Memory::new(min_addr),
        }
    }
    pub fn done(&self) -> bool {
        self.done
    }
    pub fn execute(&mut self,instr : &Instruction) {
        use Instruction::*;
        self.pc += 1;
        match *instr {
            LDD(addr) => self.ldd(addr),
            LDM(imm)  => self.ldm(imm),
            LDI(addr) => self.ldi(addr),
            LDR(imm)  => self.ldr(imm),
            LDX(addr) => self.ldx(addr),
            MOV       => self.ix = self.acc,
            IO(b)     => self.io(b),
            SUBM(imm) => self.subm(imm),
            SUBA(addr) => self.suba(addr),
            STO(addr)  => self.sto(addr),
            ADDM(imm)  => self.addm(imm),
            ADDA(addr) => self.adda(addr),
            INC(b)     => self.addn(b,1),
            DEC(b)     => self.addn(b,-1),
            LSL(imm)   => self.lsl(imm),
            LSR(imm)   => self.lsr(imm),
            XORM(imm)  => self.xorm(imm),
            XOR(addr)  => self.xora(addr),
            OR(imm)    => self.or(imm),
            ORA(addr)  => self.ora(addr),
            CMPA(addr)  => self.cmpa(addr),
            CMPM(imm)   => self.cmpm(imm),
            END         => self.end(),
            UNKNOWN     => eprintln!("Unkown command!"),
            _           => self.jmp(instr),
        }
    }
    fn lsl(&mut self, n : u16) {
        self.flag_register.set_flag(FLAGS::C,(self.acc as u16 & 0x8000u16) != 0 );
        self.acc <<= n;
    }
    fn lsr(&mut self, n : u16) {
        self.flag_register.set_flag(FLAGS::C, (self.acc & 0x1) != 0);
        self.acc >>= n;
    }
    fn ldi(&mut self,addr : u16) {
        let d = self.memory[addr as usize];
        self.acc = self.memory[d as usize];
    }
    fn ldr(&mut self,imm : i16) {
        self.ix = imm;
    }
    fn ldx(&mut self,addr : u16) {
        self.acc = self.memory[(addr + self.ix as u16) as usize];
    }
    fn xorm(&mut self, imm : i16) {
        self.flag_register.set_flags(Some(self.acc ^ imm));
        self.acc ^= imm;
    }
    fn xora(&mut self, addr : u16) {
        self.xorm(self.memory[addr as usize]);
    }
    fn or(&mut self, imm : i16) {
        self.flag_register.set_flags(Some(self.acc | imm));
        self.acc |= imm;
        
    }
    fn ora(&mut self, addr : u16) {
        self.or(self.memory[addr as usize]);
    }
    fn ldd(&mut self,addr : u16) {
        self.acc = self.memory[addr as usize];
    }
    fn ldm(&mut self, imm : i16) {
        self.acc = imm;
    }
    fn io(&mut self, inp : bool) {
        use std::io;
        if inp {
            
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
    fn addn(&mut self, acc : bool, n : i16) {
        if acc {
            self.acc += n;
        }else {
            self.ix += n;
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
            &Instruction::JPEM(x) | &Instruction::JPEA(x) => {
                if self.flag_register.get_flag(FLAGS::Z) {
                    x
                }else {
                    self.pc 
                }
            },
            &Instruction::JPNM(x) | &Instruction::JPNA(x) => {
                if self.flag_register.get_flag(FLAGS::Z) {
                    self.pc
                }else {
                    x
                }
            },
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
pub fn execute(code : Code ) {
    use std::io;
    let mut cpu = CPU::new(code.table.min_addr);
    let mut check_input = true;
    'main : while !cpu.done() {
        let (instr,actual_code) = code.get(cpu.pc as usize);
        println!("-----------    Instruction Executing : {}   ------------",actual_code.1);
        cpu.execute(&instr);
        println!("{}",cpu);
        if check_input {
            'input : loop {
                let mut buf = String::new();
                println!(r"
Press:
   enter: to step 1 instruction
   q to quit
   c to continue until end");
                io::stdin().read_line(&mut buf).unwrap();
                if buf.trim().is_empty() {
                    break 'input
                }
                match buf.chars().nth(0).unwrap() {
                    'q' => break 'main,
                    'c' => {check_input = false; break 'input},
                    _  => println!("Unrecognized command"),
                };
            }
        }
    }
}
