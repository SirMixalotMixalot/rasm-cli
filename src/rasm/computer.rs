use std::{
    
    io::{Read,Write},
    fmt::{Formatter,Result,Display},
};
use super::{
    cpu::CPU,cpu::FLAGS
};
use crate::DisplayStyle;

pub struct Computer<I : Read,O : Write, const N: usize> {
    disp_style : DisplayStyle,
    pub cpu        : CPU<I,O,N>
}
pub struct ComputerBuilder<I : Read,O : Write, const N: usize>  {
    disp_style : Option<DisplayStyle>,
    cpu        : Option<CPU<I,O,N>>
}
impl<'a,I : Read,O : Write, const N: usize> ComputerBuilder<I ,O , N> {
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
    pub fn attach_cpu(mut self, cpu : CPU<I,O,N>) -> Self {
        self.cpu = Some(cpu);
        self
    }
    pub fn build(self) -> std::result::Result<Computer<I,O,N>,&'static str> {
        if self.cpu.is_none() || self.disp_style.is_none() {
            return Err("Parts on CPU missing")
        }
       Ok( Computer {
            cpu : self.cpu.unwrap(),
            disp_style : self.disp_style.unwrap()
        })
    }
}
impl<'a,I : Read,O : Write, const N: usize> Display for Computer<I,O,N> {
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
                
               self.cpu.pc(),
               self.cpu.get_flag(FLAGS::N) as u8,
               self.cpu.get_flag(FLAGS::V) as u8,
               self.cpu.get_flag(FLAGS::Z) as u8,
               self.cpu.get_flag(FLAGS::C) as u8,
               self.cpu.acc(),
               self.cpu.ix() )


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
                
 self.cpu.pc(),
 self.cpu.get_flag(FLAGS::N) as u8,
 self.cpu.get_flag(FLAGS::V) as u8,
 self.cpu.get_flag(FLAGS::Z) as u8,
 self.cpu.get_flag(FLAGS::C) as u8,
 self.cpu.acc(),
 self.cpu.ix())


        }
        
      }
   }
}
