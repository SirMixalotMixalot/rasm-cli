
use crate::rasm::mem::Memory;
use super::{AdrMode, Code, Instruction};

use std::{
    io::{Read,Write},
    ops::{Index,IndexMut},
    
};


pub struct IOBus<R : Read, W : Write>{
    pub reader :  Option<R>,
    pub writer :  Option<W>
}
impl<R : Read,W : Write> IOBus<R,W> {
    pub fn new() -> Self {
        Self {
            reader : None,
            writer : None
        }
    }
}
pub trait RAM : Index<usize> + IndexMut<usize>{
    fn max_addr(&self) -> usize;
    fn min_addr(&self) -> usize;
}

pub struct DataBus {
    pub memory : Memory
}

impl DataBus {
    pub fn new(memory : Memory) -> Self {
        Self {memory}
    }
    pub fn read(&self, addr : usize) -> i16 {
        self.memory[addr]
 
    }
    pub fn write(&mut self,addr : usize,data :i16) {
        self.memory[addr] = data 
    }
}
pub struct CPU<R : Read ,W : Write>
{
    acc : i16,
    ix : i16,
    pub pc : u16,
    done : bool,
    flag_register : Flags,
    pub data_bus : DataBus, 
    io_bus: IOBus<R,W>,
    pub code  : Code,
}

impl<R : Read,W : Write> CPU<R,W>
 {
    pub fn new(mem : Memory, code : Code) -> Self {
        CPU {
            acc : 0,
            ix  : 0,
            pc  : 0,
            done : false,
            flag_register : Flags::new(),
            data_bus : DataBus::new(mem),
            io_bus :  IOBus::new(),
            code
        }
    }

    pub fn acc(&self) -> i16 {
        self.acc
    }
    pub fn ix(&self) -> i16 {
        self.ix
    }
    pub fn pc(&self) -> u16 {
        self.pc
    }
    pub fn get_flag(&self,flag : FLAGS) -> bool {
        self.flag_register.get_flag(flag)
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
                self.acc = self.data_bus.read(self.data_bus.read(data as usize) as usize);
            },
            AdrMode::Direct   => {
                self.acc = self.data_bus.read(data as usize);
            },
            AdrMode::Indexed  => {
                self.acc = self.data_bus.read((data as i16 + self.ix) as usize);
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
            AdrMode::Direct    => self.data_bus.read(i as usize),
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
            self.io_bus.reader.as_mut().unwrap().read_exact(&mut buffer).unwrap();
            self.acc = buffer[0] as char as i16;
            self.flag_register.set_flags(Some(self.acc));
        }else {
            write!(&mut self.io_bus.writer.as_mut().unwrap(),"{}",self.acc as u8 as char).unwrap();
        }
    }
    fn sub(&mut self, imm : i16, adr_mode : AdrMode) {
        let imm = self.get_data(imm as u16, adr_mode);

        let res = self.acc.checked_sub(imm);
        self.flag_register.set_flags(res);
        self.acc -= imm;
    }

    fn sto(&mut self,addr : u16) {
        self.data_bus.write(addr as usize, self.acc );
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

//Bit masks
pub enum FLAGS {
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
