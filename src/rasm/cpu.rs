
use super::{Instruction,AdrMode};
use crate::DisplayStyle;
use std::{
    ops::{Index,IndexMut}
    ,fmt::{Display,Formatter,Result},
    io::{Read,Write}
};


pub struct Memory {
    min_addr : u16,
    mem : Vec<i16>,
    num_vars: usize,
}

impl Memory {
    pub fn new(min_addr : u16,num_vars : usize) -> Self {
        Memory {
            min_addr,
            mem : Vec::with_capacity(16),
            num_vars,
        }
    }
}

impl Index<usize> for Memory {
    type Output = i16;
    fn index(&self, index: usize) -> &Self::Output {
        if index as u16 > self.min_addr && self.min_addr >= 200 {
            let i = index - self.min_addr as usize;
            &self.mem[i + self.num_vars]
        }else {
            &self.mem[index]
        }
    }
}
impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
         if index as u16 > self.min_addr && self.min_addr >= 200 {
            let i = index - self.min_addr as usize;
            if i >= self.mem.len() {
                self.mem.resize(i + self.num_vars,0);
            }
            
            &mut self.mem[i + self.num_vars]
        }else {
            if index >= self.mem.len() {
                self.mem.resize(index + 1, 0);
            }
            &mut self.mem[index]
        }
       
    }
}
pub struct CPU<'a, I, O>
where
    I : Read,
    O : Write {
    acc : i16,
    ix : i16,
    pub pc : u16,
    done : bool,
    flag_register : Flags,
    pub memory : &'a mut Memory, 
    display    : O,
    input      : I
}
pub struct Computer<'a,I : Read,O : Write> {
    disp_style : DisplayStyle,
    pub cpu        : CPU<'a,I,O>
}
pub struct ComputerBuilder<'a,I : Read,O : Write>  {
    disp_style : Option<DisplayStyle>,
    cpu        : Option<CPU<'a,I,O>>
}
impl<'a,I : Read,O : Write> ComputerBuilder<'a,I ,O > {
    pub fn new() -> Self {

        ComputerBuilder {
            disp_style : None,
            cpu  : None
            
        }
    }
    pub fn display_style(mut self,style : DisplayStyle) -> Self {
        self.disp_style = Some(style);
        self
    }
    pub fn attach_cpu(mut self, cpu : CPU<'a,I,O>) -> Self {
        self.cpu = Some(cpu);
        self
    }
    pub fn build(self) -> std::result::Result<Computer<'a,I,O>,&'static str> {
        if self.cpu.is_none() || self.disp_style.is_none() {
            return Err("Parts on CPU missing")
        }
       Ok( Computer {
            cpu : self.cpu.unwrap(),
            disp_style : self.disp_style.unwrap()
        })
    }
}

impl<'a,I,O> CPU<'a,I,O>
where
    I : Read,
    O : Write {
    pub fn new(memory : &'a mut Memory, display : O, input : I) -> Self {
        CPU {
            acc : 0,
            ix  : 0,
            pc  : 0,
            done : false,
            flag_register : Flags::new(),
            memory,
            display,
            input
        }
    }
    pub fn done(&self) -> bool {
        self.done
    }
    pub fn execute(&mut self,instr : &Instruction) {
        use Instruction::*;
        self.pc += 1;
        match *instr {
            LOAD{data,adr_mode} => self.load(data, adr_mode),
            LDR(x)              => self.ldr(x),
            MOV                 => self.ix = self.acc,
            IO(b)               => self.io(b),
            SUB{rhs,adr_mode}   => self.sub(rhs as i16,adr_mode),
            STO(addr)           => self.sto(addr),
            ADD{rhs,adr_mode}   => self.add(rhs as i16,adr_mode),
            INC(b)              => self.addn(b,1),
            DEC(b)              => self.addn(b,-1),
            LSL(imm)            => self.lsl(imm),
            LSR(imm)            => self.lsr(imm),
            XOR {rhs,adr_mode}  => self.xor(rhs,adr_mode),
            OR {rhs,adr_mode}   => self.or(rhs,adr_mode),
            CMP {rhs,adr_mode}  => self.cmp(rhs as i16,adr_mode), 
            END                 => self.end(),
            UNKNOWN             => eprintln!("Unkown command!"),
            _                   => self.jmp(instr),
        }
    }
    fn load(&mut self,data : u16,addressing_mode : AdrMode) {
        match addressing_mode {
            AdrMode::Indirect => {
                self.acc = self.memory[self.memory[data as usize] as usize];
            },
            AdrMode::Direct   => {
                self.acc = self.memory[data as usize];
            },
            AdrMode::Indexed  => {
                self.acc = self.memory[(data as i16 + self.ix) as usize];
            },
            AdrMode::Immediate => {
                self.acc = data as i16;
            },

        }
        self.flag_register.set_flags(Some(self.acc));
    }
    fn lsl(&mut self, n : u16) {
        self.flag_register.set_flag(FLAGS::C,(self.acc as u16 & 0x8000u16) != 0 );
        let x = (self.acc as u16) << n;
        self.acc = x as i16;
    }
    fn lsr(&mut self, n : u16) {
        self.flag_register.set_flag(FLAGS::C, (self.acc & 0x1) != 0);
        let x = self.acc as u16 >> n;
        self.acc = x as i16;
    }
    fn ldr(&mut self,imm : i16) {
        self.ix = imm;
    }
    fn xor(&mut self, imm : u16,adr_mode : AdrMode) {
        let imm = self.get_data(imm, adr_mode);
        self.flag_register.set_flags(Some(self.acc ^ imm));
        self.acc ^= imm ;
    }
    fn get_data(&self,i : u16,adr_mode : AdrMode) -> i16 {
        match adr_mode {
            AdrMode::Immediate => i as i16,
            AdrMode::Direct    => self.memory[i as usize],
            _                  => unreachable!(),
        }
    }
    fn or(&mut self, imm : u16,adr_mode : AdrMode) {
        let imm = self.get_data(imm,adr_mode);
        self.flag_register.set_flags(Some(self.acc | imm));
        self.acc |= imm;
        
    }
    fn io(&mut self, inp : bool) {
        
        if inp {
            
            let mut buffer = [0;3];
            self.input.read_exact(&mut buffer).unwrap();
            self.acc = buffer[0] as char as i16;
            self.flag_register.set_flags(Some(self.acc));
        }else {
            write!(&mut self.display,"{}",self.acc as u8 as char).unwrap();
        }
    }
    fn sub(&mut self, imm : i16, adr_mode : AdrMode) {
        let imm = self.get_data(imm as u16, adr_mode);

        let res = self.acc.checked_sub(imm);
        self.flag_register.set_flags(res);
        self.acc -= imm;
    }

    fn sto(&mut self,addr : u16) {
        self.memory[addr as usize] = self.acc;
    }
    fn add(&mut self, imm : i16,adr_mode : AdrMode) {
        let imm = self.get_data(imm as u16, adr_mode);
        self.flag_register.set_flags(
                self.acc.checked_add(imm)
            );
        self.acc += imm;
    }
    
    fn addn(&mut self, acc : bool, n : i16) {
        if acc {
            self.flag_register.set_flags(self.acc.checked_add(n));
            self.acc += n;
        }else {
            self.ix += n;
        }
    }
    fn cmp(&mut self,imm : i16,adr_mode : AdrMode) {
        let imm = self.get_data(imm as u16, adr_mode);
        self
            .flag_register.
            set_flags(Some(self.acc - imm));
    }

    fn end(&mut self) {
        println!("-------Program ending---------");
        self.done = true;
    }
    fn jmp(&mut self, instr : &Instruction) {
        let ni = match *instr {
            Instruction::JPE {addr,..} => {
                if self.flag_register.get_flag(FLAGS::Z) {
                    addr
                }else {
                    self.pc 
                }
            },
            Instruction::JPN {addr,..} => {
                if self.flag_register.get_flag(FLAGS::Z) {
                    self.pc
                }else {
                    addr
                }
            },
            _ => unreachable!()

        };
        self.pc = ni;
    }

}

impl<'a,I : Read,O : Write> Display for Computer<'a,I,O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self.disp_style{

    DisplayStyle::Denary => {   write!(f, 
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
                
               self.cpu.pc,
               self.cpu.flag_register.get_flag(FLAGS::N) as u8,
               self.cpu.flag_register.get_flag(FLAGS::V) as u8,
               self.cpu.flag_register.get_flag(FLAGS::Z) as u8,
               self.cpu.flag_register.get_flag(FLAGS::C) as u8,
               self.cpu.acc,
               self.cpu.ix )


    },
    DisplayStyle::Binary => {
       write!(f, 
r"
 ---------------------------------------------------
|                   CPU                             |
|    FLAGS                          PC              |
|    ---------                    _______           |
|   | N V Z C |                  | {:^5} |          |
|   | {} {} {} {} |                   -------           | 
|    ---------                                      |
|    ACC                               IX           |
|  __________________          __________________   |
| |{:#018b}|        |{:#018b}|  |
|  ------------------          ------------------   |  
 --------------------------------------------------- ",
                
               self.cpu.pc,
               self.cpu.flag_register.get_flag(FLAGS::N) as u8,
               self.cpu.flag_register.get_flag(FLAGS::V) as u8,
               self.cpu.flag_register.get_flag(FLAGS::Z) as u8,
               self.cpu.flag_register.get_flag(FLAGS::C) as u8,
               self.cpu.acc as u16,
               self.cpu.ix as u16)



    },
    DisplayStyle::Hex => 
    {
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
| |{:#08x}|        |{:#08x} |    |
|  --------          ---------     |  
 ---------------------------------- ",
                
               self.cpu.pc,
               self.cpu.flag_register.get_flag(FLAGS::N) as u8,
               self.cpu.flag_register.get_flag(FLAGS::V) as u8,
               self.cpu.flag_register.get_flag(FLAGS::Z) as u8,
               self.cpu.flag_register.get_flag(FLAGS::C) as u8,
               self.cpu.acc as u16,
               self.cpu.ix as u16)


        }
        
      }
   }
}
//Bit masks
enum FLAGS {
    Z = 0b1000,
    N = 0b0100,
    V = 0b0010,
    C = 0b0001,
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
