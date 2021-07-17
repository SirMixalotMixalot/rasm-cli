use crate::DisplayStyle;
use super::{Code,cpu::*,mem::Memory,computer::ComputerBuilder};

pub fn execute(code : Code, style : DisplayStyle ) {
    use std::io;

    let mut mem  = Memory::new(code.table.min_addr,code.table.num_vars);
    let cpu = CPU::new(&mut mem,io::stdout(),io::stdin());
    let mut computer = ComputerBuilder::new()
    .attach_cpu(cpu)
    .display_style(style)
    .build()
    .unwrap();
    //let mut cpu = CPU::new(code.table.min_addr,code.table.num_vars,style,io::stdout(),io::stdin());
    let mut check_input = true;
    'main : while !computer.cpu.done() {
        let (instr,actual_code) = code.get(computer.cpu.pc() as usize)
            .expect("Program ending...Remember to add 'END' to the end of your program");
        println!("-----------    Instruction Executing : {}   ------------",actual_code.1);
        computer.cpu.execute(&instr);
        
        println!("{}",computer);
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
