use std::ops::{Index,IndexMut};
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
