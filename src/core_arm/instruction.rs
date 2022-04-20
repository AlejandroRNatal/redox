// Instruction decoder based off
// ARM7tdmi Manual: https://www.dwedit.org/files/ARM7TDMI.pdf
use std;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpCode {
    ARM_OpCode,
    THUMB_OpCode,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mode {
    ARM,
    THUMB,
}

/// Tag for matching Instructions with corresponding Exec methods
pub enum Instructions {

    // ARM Mode Instructions
    Multiply,
    MultiplyLong,
    DataProcessing,
    Branching,
    SingleDataTransfer,
    SignedDataTransfer,
    BlockDataTransfer,
    Swap,
    SoftInterrupt,
    CDT,
    CRT,
    Undefined,

    //TODO: ADD Missing THUMB Instructions
}

/// Generic Instruction for implementing all variants
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Instruction<T> {
    
    //registers
    rs: T,
    rm: T,
    rd_lo: T,
    rn: T,
    rd: T,

    // Discriminant factors
    tag: Instructions,
    set: Bit,
    accumulate: Option<Bit>,
    signed: Option<Bit>,
    
    // Data for all the Instructions
    word: T,
    cond: T,
    op_code: OpCode,
    
    //Shift relevant data
    shift_ammount: Option<T>,
    shift: Option<Shift>,
    
    // Mode of the instruction (ARM or THUMB)
    mode: Mode,
    cycles: u8,
}

pub trait Instruction<T> {

    fn from_word(word: T) -> Self ;

    fn as_word(&self) -> T ;

    fn source(&self) -> T ;

    fn destination(&self) -> T ;

    fn conditions(&self) -> T ;

    fn op_code(&self) -> OpCode ;

    fn first_operand(&self) -> T ;

    fn second_operand(&self) -> T ;

    fn rm(&self) -> T ;

    fn offset(&self) -> T ;

    fn cycles(&self) -> u8 ;
}

impl Instruction<u32> for Instruction<u32> {
    
    pub fn Multiply(word: u32) -> Self {
        let rs: u32 = 0;
        let rm: u32 = 0;

        let rn: u32 = 0;
        let rd: u32 = 0;

        let rd_lo: u32 = 0;
        
        let set: Bit = Bit::Unset;//TODO: Properly check this
        let accumulate: Bit = Bit::Unset;

        let signed: Bit = Bit::Unset;

        let cond = 0;//TODO: Filter out conditions

        Instruction<u32> {
                rs: rs,
                rm: rm,
                rd_lo: rd_lo,
                rn: rn,
                rd: rd,

                // Discriminant factors
                tag: Instructions::Multiply,
                set: set,
                accumulate: accumulate,
                signed: signed,
                
                // Data for all the Instructions
                word: word,
                cond: cond,
                op_code: OpCode,
                
                //Shift relevant data
                shift_ammount: T,
                shift: Shift,
                
                // Mode of the instruction (ARM or THUMB)
                mode: Mode::ARM,
                cycles: 4,
        }
    }

    pub fn from_word(word: u32) -> Option<Instruction> {

            //bits (20-27 | 4-7)-> 12 bits
            //Filter by top 8 bits and bottom 4 | To determine which operation it is
            let filter: u16 = (((word >> 16) && 0xFF0) | ((word >> 20) & 0x0F));
            
            
            match filter {
                0 => {/* DATA Processing / PSR Instruction here*/ Some(data_operand(word))},
                /*0x09 |*/ 0x19 | 0x39  => {Some(multiply(word))},
                0x89 | 0xC9 | 0xE9 | 0xF9 => {Some(multiply_long(word))},
                0x121 => {Some(bx(word))},
                0xA00 | 0xB00 => {Some(branch(word))},
                0x400 | 0x600 | 0x700 | 0x780 | 0x7C0 | 0x7E0 | 0x7F0 => {Some(single_data_transfer(word))},
                0x09 => {Some(signed_data_transfer(word))}, //This instruction clashes with multiply
                0x800| 0x900 | 0x980 | 0x9C0 | 0x9E0 | 0x9F0 => {Some(block_data_transfer(word))},

                0x109 | 0x329 => {Some(swap(word))},
                0xF00 => {Some(swi(word))},
                0xE00 => {Some(cdp(word))},
                0xC00 | 0xD00 | 0xD80 | 0xDC0 | 0xDE0 | 0xDF0 => {Some(cdt(word))},
                0xE01 | 0xE21 => {Some(crt(word))},
                0x301 => {None}, //Undefined instruction according to manual
                _ => {unimplemented!("Oh-oh! Unimplemented Instruction: {:?}", _)},
            }
        }   
}

/// DataProcessing Immediate Shift
// pub struct DataProcessingImmShift {
//     word: u32,
//     mode: Mode,
//     cond: u32,
//     op_code: ARM_OpCode,
//     set: Bit,
//     rn: u32,
//     rd: u32,
//     shift_ammount: u32,
//     shift: Shift,
//     rm: u32,
//     cycles: u8,
// }

// pub struct DataProcessingImmAmmountShift {
//     word: u32,
//     mode: Mode,
//     cond: u32,
//     op_code: ARM_OpCode,
//     set: Bit,
//     rn: u32,
//     rd: u32,
//     shift_ammount: u32,
//     shift: Shift,
//     rm: u32,
//     cycles: u8,
// }

// pub struct DataProcessingRegShift {
//     word: u32,
//     mode: Mode,
//     cond: u32,
//     op_code: ARM_OpCode,
//     set: Bit,
//     rn: u32,
//     rd: u32,
//     shift_ammount: u32,
//     shift: Shift,
//     rm: u32,
//     cycles: u8,
// }

// pub struct MultiplyLong {
//     word: u32,
//     signed: bool,
//     accumulate: bool,
//     mode: Mode,
//     cond: u32,
//     op_code: ARM_OpCode,
//     set: Bit,
//     rn: u32,
//     rd: u32,
//     rd_lo: u32,
//     rs: u32,
//     rm: u32,
//     cycles: u8,
// }

// pub struct Multiply {
//     word: u32,
//     accumulate: bool,
//     mode: Mode,
//     cond: u32,
//     op_code: ARM_OpCode,
//     set: Bit,
//     rn: u32,
//     rd: u32,
//     rs: u32,
//     rm: u32,
//     cycles: u8,
// }

// impl Instruction<u32> for Multiply {
    
//     fn from_word(word: u32) -> Self {
//         let check_accumulate: u8 = ((word & 0x10_0000) >> 20) as u8 ;
        
//         let cond: u8 = (((word) & 0xF000_0000) >> 28) as u8;
//         let alter = (((word) & 0x10_0000) >> 20) as u8;
        
//         let rd = (((word) & 0xF_0000) >> 16) as u8;
//         let rs = (((word) & 0xF000) >> 12) as u8;
        
//         let rm = ((word)& 0x0F) as u8;

//         match check_accumulate {
//             Bit::Set => {Multiply {
//                                     word: word,
//                                     accumulate: true,
//                                     mode: Mode::ARM,
//                                     cond: cond,
//                                     op_code: ARM_OpCode::TEQ,//TODO: Doesn't apply
//                                     set: alter,
//                                     rn: 0,
//                                     rd: rd,
//                                     rs: rs,
//                                     rm: rm,
//                                     cycles: 4,}},
//             Bit::Unset => {Multiply {
//                                     word: word,
//                                     accumulate: false,
//                                     mode: Mode::ARM,
//                                     cond: cond,
//                                     op_code: ARM_OpCode::TEQ,
//                                     set: alter,
//                                     rn: rn,
//                                     rd: rd,
//                                     rs: rs,
//                                     rm: rm,
//                                     cycles: 4,}},
//         }

//     }
// }

// impl Instruction<u32> for MultiplyLong {
    
//     fn from_word(word: u32) -> Self {
//         let check_accumulate: u8 = ((word & 0x10_0000) >> 20) as u8 ;
        
//         let cond: u8 = (((word) & 0xF000_0000) >> 28) as u8;
        
//         let signed = (((word) & 0x10_0000) >> 20) as u8;
//         let alter = (((word) & 0x10_0000) >> 21) as u8;
        
//         let rd = (((word) & 0xF_0000) >> 16) as u8;
//         let rd_lo = (((word) & 0xF000) >> 12) as u8;
//         let rs = (((word) & 0xF00) >> 8) as u8;
        
//         let rm = ((word)& 0x0F) as u8;

//         match check_accumulate {
//             Bit::Set => {MultiplyLong {
//                                     word: word,
//                                     accumulate: true,
//                                     mode: Mode::ARM,
//                                     cond: cond,
//                                     op_code: ARM_OpCode::TEQ,//TODO: Doesn't apply
//                                     set: alter,
//                                     rn: 0,
//                                     rd: rd,
//                                     rd_lo: rd_lo,
//                                     rs: rs,
//                                     rm: rm,
//                                     cycles: 4,}},
//             Bit::Unset => {MultiplyLong {
//                                     word: word,
//                                     accumulate: false,
//                                     mode: Mode::ARM,
//                                     cond: cond,
//                                     op_code: ARM_OpCode::TEQ,//TODO: Doesn't apply
//                                     set: alter,
//                                     rn: rn,
//                                     rd: rd,
//                                     rd_lo: rd_lo,
//                                     rs: rs,
//                                     rm: rm,
//                                     cycles: 4,}},
//         }

//     }
// }

// impl Instruction<u32> for DataProcessingImmShift {

//     /// Create instruction from u32 word.
//     fn from_word(word: u32) -> Self {

//         let set = (((byte) && ( 1 << 25 )) >> 25 ) as u8;
//         //Immediate Rotate is always a rotate right
//         let op_code = ((byte && 0x1E0_0000 ) >> 21) as u8;
//         let flags = ((byte) && (0xF000_0000_0000_0000) >> 28);
        
//         //This may be wrong
//         let rotate: u8 = (( byte && (0xF00) ) >> 8) as u8;
//         let immediate: u8 = (( byte ) && (0xFF)) as u8;

        
//         let rn: u8 = ( ( byte && 0xF_0000) >> 16) as u8 ;
//         let rd: u8 = ( ( byte && 0xF000) >> 12) as u8;

//         DataProcessingImmShift{
//             word: word,
//             mode: Mode::ARM,
//             cond: flags,
//             op_code: op_code,
//             set: set,
//             rn: rn,
//             rd: rd,
//             shift: immediate,
//             shift_ammount: rotate,
//             rm: immediate >> (2 * rotate),
//             cycles: 4,
//         }
//     }

//     /// Return original word representation
//     fn as_word(&self) -> u32 {
//         self.word
//     }

//     fn source(&self) -> u32 {
//         self.rn
//     }

//     fn destination(&self) -> u32 {
//         self.rd
//     }

//     fn conditions(&self) -> u32 {
//         self.cond
//     }

//     fn op_code(&self) -> OpCode {
//         self.op_code
//     }

//     fn first_operand(&self) -> u32 {
//         self.rn
//     }

//     fn second_operand(&self) -> u32 {
//         self.rm 
//     }

//     fn rm(&self) -> u32 {
//         self.rm
//     }

//     /// This instruction doesn't implement the Offset attribute
//     fn offset(&self) -> u32 {
//         0//No Offset for this instruction
//     }
// }

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shift {
    LSL=0x00,
    LSR=0x01,
    ARS=0x02,
    RR=0x03,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ARM_OpCode {
    ADC =  5,
    ADD =  4,
    AND =  0,
    BIC =  0xe,
    CMN =  0xb,
    CMP =  0xa,
    EOR =  1,
    MOV =  0xd,
    MVN =  0xf,
    ORR =  0xc,
    RSB =  3,
    RSC =  7,
    SBC =  6,
    SUB =  2,
    TEQ =  9,
    TST =  8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum THUMB_OpCode {
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Bit {
    Set(u8)=1,
    Unset(u8)=0,
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode_u32(){
        let word: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0000;
        
        assert_eq!(decode_u32(word), Instruction<u32>);
        
    }

}