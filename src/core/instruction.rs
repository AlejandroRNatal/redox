// Instruction decoder based off
// ARM7tdmi Manual: https://www.dwedit.org/files/ARM7TDMI.pdf

use std;

pub enum Shift {
    LSL=0x00,
    LSR=0x01,
    ARS=0x02,
    RR=0x03,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IncDecTarget {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    CPSR,
    SPSR,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum JumpTest {
    NotZero,
    Zero,
    Carry,
    NotCarry,
    Always,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ARM_Instruction {
    AND =  0,
    EOR =  1,
    SUB =  2,
    RSB =  3,
    ADD =  4,
    ADC =  5,
    SBC =  6,
    RSC =  7,
    TST =  8,
    TEQ =  9,
    CMP =  0xa,
    CMN =  0xb,
    ORR =  0xc,
    MOV =  0xd,
    MVN =  0xf,
    BIC =  0xe,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum THUMB_Instruction {
    AND = 0,
    EOR  =  1,
    LSL  =  2,
    LSR  =  3,
    ASR  =  4,
    ADC  =  5,
    SBC  =  6,
    ROR  =  7,
    TST  =  8,
    NEG  =  9,
    CMP  =  0xa,
    CMN  =  0xb,
    ORR  =  0xc,
    MUL  =  0xd,
    BIC  =  0xe,
    MVN  =  0xf,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction { 
    // Arithmetic
    INC(IncDecTarget),
    DEC(IncDecTarget),

    // Control
    NOP,
    EI,
    DI,
    HALT,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Condition {
    EQ=0x00,
    NE=0x01,
    CS=0x02,
    CC=0x03,
    MI=0x04,
    PL=0x05,
    VS=0x06,
    VC=0x07,
    HI=0x08,
    LS=0x09,
    GE=0x0A,
    LT=0x0B,
    GT=0x0C,
    LE=0x0D,
    AL=0x0E,
}

pub enum Bit {
    Set(u8)=1,
    Unset(u8)=0,
}

pub enum Flags {

}

impl Instruction {

    pub fn from_byte(byte: u32) -> Option<Instruction> {
        //bits (20-27 | 4-7)-> 12 bits
        //Filter by top 8 bits and bottom 4 | To determine which operation it is
        let filter: u16 = (((byte >> 16) && 0xFF0) | ((byte >> 20) & 0x0F));
        
        
        match filter {
            0 => {/* DATA Processing / PSR Instruction here*/ Some(data_operand(byte))},
            /*0x09 |*/ 0x19 | 0x39  => {Some(multiply(byte))},
            0x89 | 0xC9 | 0xE9 | 0xF9 => {Some(multiply_long(byte))},
            0x121 => {Some(bx(byte))},
            0xA00 | 0xB00 => {Some(branch(byte))},
            0x400 | 0x600 | 0x700 | 0x780 | 0x7C0 | 0x7E0 | 0x7F0 => {Some(single_data_transfer(byte))},
            // 0x09 => {Some(signed_data_transfer(byte))}, //This instruction clashes with multiply
            0x800| 0x900 | 0x980 | 0x9C0 | 0x9E0 | 0x9F0 => {Some(block_data_transfer(byte))},

            0x109 | 0x329 => {Some(swap(byte))},
            0xF00 => {Some(swi(byte))},
            0xE00 => {Some(cdp(byte))},
            0xC00 | 0xD00 | 0xD80 | 0xDC0 | 0xDE0 | 0xDF0 => {Some(cdt(byte))},
            0xE01 | 0xE21 => {Some(crt(byte))},
            0x301 => {None}, //Undefined instruction according to manual
            _ => {unimplemented!("Oh-oh! Unimplemented Instruction: {:?}", _)},
        }
    }

    /// These instructions take only 1 CPU Cycle
    fn data_operand(byte: u32) -> Option<Instruction> {
        // let operand_2: u16 = 0;
        let immediate: u8 = (((byte) && ( 1 << 25 )) >> 25 ) as u8;
        match immediate {
            Bit::Set => {/*Not immediate Operation*/},
            Bit::Unset => {/* Immediate Operation */ immediate(byte)},
            _ => { None },
        }
    }

    fn multiply(byte: u32) -> Option<Instruction> {
        
    }

    fn multiply_long(byte: u32) -> Option<Instruction> {
        
    }

    fn single_data_transfer(byte: u32) -> Option<Instruction> {

    }

    fn block_data_transfer(byte: u32) -> Option<Instruction> {

    }

    fn swap(byte: u32) -> Option<Instruction> {

    }

    fn swi(byte: u32) -> Option<Instruction> {

    }

    fn cdp(byte: u32){

    }

    fn cdt(byte: u32){

    }

    fn crt(byte: u32){

    }

    fn bx(byte: u32){

    }

    fn branch(byte: u32){

    }

    fn immediate(byte: u32) {
        //Immediate Rotate is always a rotate right
        let op_code = ((byte && 0x1E0_0000 ) >> 21) as u8;
        let flags = ((byte) && (0xF000_0000_0000_0000) >> 28);
        
        //This may be wrong
        let rotate: u8 = (( byte && (0xF00) ) >> 8) as u8;
        let immediate: u8 = (( byte ) && (0xFF)) as u8;

        
        let rn: u8 = ( ( byte && 0xF_0000) >> 16) as u8 ;
        let rd: u8 = ( ( byte && 0xF000) >> 12) as u8;

        match op_code {
            //TODO AND may be wrong here, blog says it is supposed to be AND Rd, Rn, Rm LSL #5;
            ARM_Instruction::AND => {arm_imm_and(flags, rn, rd, immediate >> (2 * rotate) )}, //Logical
            ARM_Instruction::EOR => {}, //Logical
            ARM_Instruction::SUB => {}, 
            ARM_Instruction::RSB => {}, 
            ARM_Instruction::ADD => {}, 
            ARM_Instruction::ADC => {}, 
            ARM_Instruction::SBC => {}, 
            ARM_Instruction::RSC => {}, 
            ARM_Instruction::TST => {}, //Logical
            ARM_Instruction::TEQ => {}, //Logical
            ARM_Instruction::CMP => {},
            ARM_Instruction::CMN => {},
            ARM_Instruction::ORR => {}, //Logical
            ARM_Instruction::MOV => {}, //Logical
            ARM_Instruction::MVN => {}, //Logical
            ARM_Instruction::BIC => {}, //Logical
        }
    }

    fn arm_imm_and(flags: u8, source: u8, destination: u8, operand_2: u8) {
        //Store result of source & operand_2 into destination
        //cpu.gp_reg[destination] = cpu.gp_reg[source] & operand_2
    }
}