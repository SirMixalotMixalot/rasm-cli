use crate::DisplayStyle;

use super::{Code,cpu::CPU};

pub fn execute(code : Code, style : DisplayStyle ) {
    use std::io;
    let mut cpu = CPU::new(code.table.min_addr,code.table.num_vars,style);
    let mut check_input = true;
    'main : while !cpu.done() {
        let (instr,actual_code) = code.get(cpu.pc as usize)
            .expect("Program ending...Remember to add 'END' to the end of your program");
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
                match buf.chars().nth(0).unwrap().to_ascii_lowercase() {
                    'q' => break 'main,
                    'c' => {check_input = false; break 'input},
                    _  => println!("Unrecognized command"),
                };
            }
        }
    }
}
