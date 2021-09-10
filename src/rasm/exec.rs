use crate::DisplayStyle;
use super::{Code, SymbolTable, computer::ComputerBuilder, cpu::*, mem::Memory};

pub fn execute(code : Code, table : SymbolTable, style : DisplayStyle ) {
    use std::io;
    let display = io::stdout();
    let keyboard = io::stdin();
    let mem = Memory::new(0,table,style);
    
    let cpu = CPU::new(mem,code);
    let mut computer = ComputerBuilder::new()
                        .attach_cpu(cpu)
                        .attach_input_source(keyboard)
                        .attach_output_display(display)
                        .display_style(style)
                        .build()
                        .unwrap();
    //let mut cpu = CPU::new(code.table.min_addr,code.table.num_vars,style,io::stdout(),io::stdin());
    let mut check_input = true;
    'main : while !computer.cpu.done() {
        let (instr,actual_code) = match computer.get_current_instruction() {
            Some(i) => i,
            None => {
                eprintln!("Program ending");
                std::process::exit(0);
            }
        };
        println!("-----------    Instruction Executing : {}   ------------",actual_code);
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
