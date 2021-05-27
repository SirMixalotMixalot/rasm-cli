#[derive(Clone, Copy)]
pub enum AdrMode {
    Indexed,
    Immediate,
    Direct,
    Indirect,
}



//#[derive(Debug)]
pub enum Instruction {
    IO(bool),
    LOAD {data : u16, adr_mode : AdrMode},
    LDR(i16), //Load into index register <imm> 
    MOV     , //ACC -> IX lol 
    SUB {rhs : u16, adr_mode : AdrMode},
    STO(u16),
    ADD {rhs : u16, adr_mode : AdrMode},
    INC(bool), //true:ACC, false:IX
    DEC(bool), //true:ACC, false:IX
    LSL(u16),  //logical shift left 
    LSR(u16),  //logical shift right
    XOR {rhs : u16, adr_mode : AdrMode},
    OR {rhs : u16, adr_mode : AdrMode},
    CMP {rhs : u16, adr_mode : AdrMode},
    JPE {addr : u16, adr_mode : AdrMode},
    JPN {addr : u16, adr_mode : AdrMode},
    END,
    UNKNOWN,


}
impl Instruction {
    pub fn new(opcode : String,rhs : u16) -> Self {
        //handle sto
        //all addresses have to be offset
        //TODO:handle acc and ix in a better way?
        if &opcode == "STO" {
            return Self::STO(rhs)
        }
        let adr_mode = AdrMode::Direct;
        match opcode.chars().nth(0).unwrap() {
            'L' => Instruction::load_instruction(opcode,rhs),
            'S' => Instruction::SUB {rhs,adr_mode},
            'A' => Instruction::ADD {rhs,adr_mode},
            'I' => Instruction::INC(rhs == 0), //Acc is 0 in symtable
            'D' => Instruction::DEC(rhs == 0),
            'C' => Instruction::CMP{rhs,adr_mode},
            'X' => Instruction::XOR{rhs,adr_mode},
            'O' => Instruction::OR {rhs,adr_mode},
            'M' => Instruction::MOV,
            'J' => Instruction::jmp_instruction(opcode,rhs,adr_mode),
            'E' => Instruction::END,
             _  => Instruction::UNKNOWN,

        }
    }
    pub fn with_imm(opcode : String, rhs : u16) -> Self {
        let first_char = opcode.chars().nth(0).unwrap();
        let adr_mode = AdrMode::Immediate;
        match first_char {
            'L' => {
                if opcode.chars().nth(1).unwrap() == 'S' {
                    if opcode.chars().nth(2).unwrap() == 'R' {
                        Instruction::LSR(rhs)
                    }else {
                        Instruction::LSL(rhs)
                    }   
                }else {
                    Self::load_instruction(opcode,rhs)
                }
            },

            'S' => Instruction::SUB{rhs,adr_mode},
            'A' => Instruction::ADD{rhs,adr_mode},
            'C' => Instruction::CMP{rhs,adr_mode},
            'X' => Instruction::XOR{rhs,adr_mode},
            'O' => Instruction::OR{rhs,adr_mode},
            'J' => Self::jmp_instruction(opcode,rhs,adr_mode),
             _  => Instruction::UNKNOWN,

        }
    }
    fn load_instruction(opcode : String,data : u16) -> Self {
        match opcode.chars().last().expect("Error on load instruction") {
            'D' => Instruction::LOAD{data,adr_mode : AdrMode::Direct},
            'M' => Instruction::LOAD{data,adr_mode : AdrMode::Immediate},
            'R' => Instruction::LDR(data as i16), //Loads into register
            'I' => Instruction::LOAD{data, adr_mode : AdrMode::Indirect},
            'X' => Instruction::LOAD{data, adr_mode : AdrMode::Indexed},
             _  => Instruction::UNKNOWN,
        }
    }
    fn jmp_instruction(opcode : String,addr : u16,adr_mode : AdrMode) -> Self {
    
        match opcode.chars().nth(2).expect("Error on Jump instruction") {
            'E' => Instruction::JPE {addr,adr_mode},
            'N' => Instruction::JPN {addr,adr_mode},
            
             _  => Instruction::UNKNOWN,
        }
    }
    
}
