
use std::{fmt::{Formatter,Result,Display}, io::{Read, Write}};
use super::{cpu::*, instr::Instruction};
use crate::DisplayStyle;

pub struct Computer<R : Read,W : Write> {
    disp_style : DisplayStyle,
    pub cpu        : CPU<R,W>,
}
impl<R : Read, W : Write> Computer<R,W> {
    pub fn get_current_instruction<'a,'b>(&'a mut self) -> Option<(Instruction,String)> {
        let s = self.cpu.code.get(self.cpu.pc() as usize);
        self.cpu.pc += 1;
        s
    }
}

pub struct ComputerBuilder<R : Read,W : Write>  {
    disp_style : Option<DisplayStyle>,
    cpu        : Option<CPU<R,W>>,
    io         : Option<IOBus<R,W>>,

}
impl<R : Read,W : Write> ComputerBuilder<R,W> {
    pub fn new() -> Self {

        ComputerBuilder {
            disp_style : None,
            cpu  : None,
            io : None,
            
        }
    }
    pub fn display_style(mut self,style : DisplayStyle) -> Self {
        self.disp_style = Some(style);
        self
    }
    pub fn attach_cpu(mut self, cpu : CPU<R,W>) -> Self {
        self.cpu = Some(cpu);
        self
    }
    
    pub fn build(self) -> std::result::Result<Computer<R,W>,&'static str> {
        if self.cpu.is_none() || self.disp_style.is_none() {
            return Err("Parts on Computer missing")
        }
       Ok( Computer {
            cpu : self.cpu.unwrap(),
            disp_style : self.disp_style.unwrap(),
        })
    }

    pub(crate) fn attach_input_source(mut self, keyboard: R) -> Self {
        let io = &mut self.io;
        match io.as_mut() {
            Some(i) => {
                i.reader = Some(keyboard);
            }
            None => {
                let mut i = IOBus::new();
                i.reader = Some(keyboard);
                *io = Some(i);
                
            }

        }
        self
    }
    pub fn attach_output_display(mut self, display : W) -> Self {
        let io = &mut self.io;
        match io.as_mut() {
            Some(i) => {
                i.writer = Some(display);
            }
            None => {
                let mut i = IOBus::new();
                i.writer = Some(display);
                *io = Some(i);
                
            }

        }
        self
    }
}
impl<R : Read,W : Write> Display for Computer<R,W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self.disp_style{

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
        
      }?;
      writeln!(f,"{}",self.cpu.data_bus.memory)
   }
}
