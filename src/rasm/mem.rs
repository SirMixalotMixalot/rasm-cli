use crate::{DisplayStyle};
use std::{fmt::Display, ops::{Index,IndexMut}};


//variables go in first couple slots
pub struct Memory<const N : usize> {
    min_addr : u16,
    mem : [i16; N],
    
    disp_style : DisplayStyle,
    num_vars: usize,
}

impl<const N : usize> Memory<N> {
    pub fn new(min_addr : u16,disp_style : DisplayStyle, num_vars: usize) -> Self {
        Memory {
            min_addr,
            mem : [0;N],
            disp_style,
            num_vars
        }
    }
    pub fn min_addr(&self) -> u16 {
        self.min_addr
    }
    pub fn max_addr(&self) -> u16 {
        self.num_vars as u16
    }
}

impl<const N: usize> Index<usize> for Memory<N> {
    type Output = i16;
    fn index(&self, index: usize) -> &Self::Output {
        &self.mem[index]
    }
}
impl<const N:usize> IndexMut<usize> for Memory<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
         &mut self.mem[index]
    }
}
impl<const N: usize> Display for Memory<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
      for addr in self.min_addr()..self.max_addr() {
        
        match self.disp_style {
            DisplayStyle::Denary => {
                writeln!(f,"|{:^width$}|{:^width$}|",addr,self[addr as usize],width=max_len as usize)?;
                
            },
            DisplayStyle::Binary => {   
                writeln!(f,"|{:^#width$b}|{:^#width$b}|",addr,self[addr as usize],width=max_len as usize)?;
            },
            DisplayStyle::Hex   => {
                writeln!(f,"|{:^#width$x}|{:^#width$x}|",addr,self[addr as usize],width=max_len as usize)?;
            }
        }
      }
      writeln!(f," {:-^width$}",'-',width = (max_len * 2 + 1 ) as usize)
      

    }
}
