
use std::fmt::{Formatter,Result,Display};
use super::{
    cpu::CPU,cpu::FLAGS
};
use crate::DisplayStyle;

pub struct Computer<'a> {
    disp_style : DisplayStyle,
    pub cpu        : CPU<'a>,
}

pub struct ComputerBuilder<'a>  {
    disp_style : Option<DisplayStyle>,
    cpu        : Option<CPU<'a>>,

}
impl<'a> ComputerBuilder<'a> {
    pub fn new() -> Self {

        ComputerBuilder {
            disp_style : None,
            cpu  : None,
            
        }
    }
    pub fn display_style(mut self,style : DisplayStyle) -> Self {
        self.disp_style = Some(style);
        self
    }
    pub fn attach_cpu(mut self, cpu : CPU<'a>) -> Self {
        self.cpu = Some(cpu);
        self
    }
    
    pub fn build(self) -> std::result::Result<Computer<'a>,&'static str> {
        if self.cpu.is_none() || self.disp_style.is_none() {
            return Err("Parts on Computer missing")
        }
       Ok( Computer {
            cpu : self.cpu.unwrap(),
            disp_style : self.disp_style.unwrap(),
        })
    }
}
impl<'a> Display for Computer<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let x = match self.disp_style{

    DisplayStyle::Denary => {   writeln!(f, 
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
                
               self.cpu.pc(),
               self.cpu.get_flag(FLAGS::N) as u8,
               self.cpu.get_flag(FLAGS::V) as u8,
               self.cpu.get_flag(FLAGS::Z) as u8,
               self.cpu.get_flag(FLAGS::C) as u8,
               self.cpu.acc(),
               self.cpu.ix() )


    },
    DisplayStyle::Binary => {
       writeln!(f, 
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
                
             self.cpu.pc(),
             self.cpu.get_flag(FLAGS::N) as u8,
             self.cpu.get_flag(FLAGS::V) as u8,
             self.cpu.get_flag(FLAGS::Z) as u8,
             self.cpu.get_flag(FLAGS::C) as u8,
             self.cpu.acc(),
             self.cpu.ix())



    },
    DisplayStyle::Hex => 
    {
       writeln!(f, 
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
                
 self.cpu.pc(),
 self.cpu.get_flag(FLAGS::N) as u8,
 self.cpu.get_flag(FLAGS::V) as u8,
 self.cpu.get_flag(FLAGS::Z) as u8,
 self.cpu.get_flag(FLAGS::C) as u8,
 self.cpu.acc(),
 self.cpu.ix())


        }
        
      };
      //  _______________________
      // | Addr : Name   | Value {16 bits/ 5 digits/ 4 higits (hex digits)}|
      //
      let max_len : u16 = match self.disp_style {
            DisplayStyle::Denary =>  {
                5
            },
            DisplayStyle::Binary => {
                16
            },
            DisplayStyle::Hex => {
                4
            }
      } ;
      let max_len = std::cmp::max(max_len,10);
      writeln!(f," {:-^width$}",'-',width = (max_len * 2 + 1) as usize)?;
      write!(f,"|{:^width$}|","Addresses",width=max_len as usize)?;
      writeln!(f,"{:^width$}|","Contents",width=max_len as usize)?;
      writeln!(f," {:-^width$}",'-',width = (max_len * 2 + 1) as usize)?;
      writeln!(f," {:-^width$}",'-',width = (max_len * 2 + 1) as usize)?;
      for addr in self.cpu.min_addr()..self.cpu.max_addr() {
        
        match self.disp_style {
            DisplayStyle::Denary => {
                writeln!(f,"|{:^width$}|{:^width$}|",addr,self.cpu.read(addr as usize),width=max_len as usize)?;
                
            },
            DisplayStyle::Binary => {   
                writeln!(f,"|{:^#width$b}|{:^#width$b}|",addr,self.cpu.read(addr as usize),width=max_len as usize)?;
            },
            DisplayStyle::Hex   => {
                writeln!(f,"|{:^#width$x}|{:^#width$x}|",addr,self.cpu.read(addr as usize),width=max_len as usize)?;
            }
        }
      }
      writeln!(f," {:-^width$}",'-',width = (max_len * 2 + 1 ) as usize)?;
      x
   }
}
