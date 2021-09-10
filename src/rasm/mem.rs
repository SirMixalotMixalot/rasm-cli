use crate::{DisplayStyle, rasm::cpu::RAM};
use std::{fmt::Display, ops::{Index,IndexMut}};

use super::SymbolTable;
pub struct Memory {
    min_addr : u16,
    mem : Vec<i16>,
    table: SymbolTable,
    disp_style : DisplayStyle,
}

impl Memory {
    pub fn new(min_addr : u16,table : SymbolTable, disp_style : DisplayStyle) -> Self {
        Memory {
            min_addr,
            mem : vec![0;table.len()],
            table,
            disp_style
        }
    }
    pub fn min_addr(&self) -> u16 {
        self.min_addr
    }
    pub fn max_addr(&self) -> u16 {
        self.min_addr + self.table.len() as u16
    }
}

impl Index<usize> for Memory {
    type Output = i16;
    fn index(&self, index: usize) -> &Self::Output {
        if index as u16 > self.min_addr && self.min_addr >= 200 {
            let i = index - self.min_addr as usize;
            &self.mem[i + self.table.len()]
        }else {
            &(self.mem[index])
        }
    }
}
impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
         if index as u16 > self.min_addr && self.min_addr >= 200 {
            let i = index - self.min_addr as usize;
            if i >= self.mem.len() {
                self.mem.resize(i + self.table.len(),0);
            }
            
            &mut (self.mem[i + self.table.len()])
        }else {
            if index >= self.mem.len() {
                self.mem.resize(index + 1, 0);
            }
            &mut (self.mem[index] )
        }
       
    }
}
impl Display for Memory {
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
impl RAM for Memory {
    fn max_addr(&self) -> usize {
        self.max_addr() as usize
    }
    fn min_addr(&self) -> usize {
        self.min_addr() as usize
    }
}