// Instruction decoder based off
// ARM7tdmi Manual: https://www.dwedit.org/files/ARM7TDMI.pdf
#![allow(clippy::upper_case_acronyms)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_doc_comments)]

macro_rules! shift_right {
    ($data: tt, $amount: tt) => { { $data >> $amount} };
}

macro_rules! apply_mask {
    ($bits: tt , $mask: tt) => { $bits & $mask };
}

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
#[derive(Clone, Debug, Copy, PartialEq)]
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
    NOP,
    //TODO: ADD Missing THUMB Instructions
}

/// Generic Instruction for implementing all variants
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

pub trait Inst<T> {

    fn from_word(word: T) -> Option<Instruction<T>> ;

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

impl Inst<u32> for Instruction<u32> {
    
    fn as_word(&self) -> u32 {
        self.word
    }

    fn source(&self) -> u32 {
        self.rs
    }

    fn destination(&self) -> u32 {
        self.rd
    }

    fn conditions(&self) ->  u32 {
        self.cond
    }

    fn op_code(&self) -> OpCode {
        self.op_code
    }

    fn first_operand(&self) -> u32 {
        self.rd
    }

    fn second_operand(&self) -> u32 {
        0
    }

    fn rm(&self) -> u32 {
        self.rm
    }

    fn offset(&self) -> u32 {
        0
    }

    fn cycles(&self) -> u8 {
        self.cycles
    }

    fn from_word(word: u32) -> Option<Instruction::<u32>> {
            //bits (20-27 | 4-7)-> 12 bits
            //Filter by top 8 bits and bottom 4 | To determine which operation it is
            let shifted:u16 = shift_right!(word , 16) as u16;
            let top: u16 = apply_mask!(shifted, 0x0ff0) as u16;
            let bottom: u16 = ((word >> 4) as u16) & 0x000f;
            let filter = top | bottom; 
            
            dbg!("Received: {:?} | Extracted: {:?}", word, filter);
            
            match filter {
                0 => {Some(Instruction::<u32>::DataProcessing(word))},
                0x09 | 0x19 | 0x29 | 0x39  => {Some(Instruction::<u32>::Multiply(word))},
                0xA9 | 0xC9 | 0xE9 | 0xF9 => {Some(Instruction::<u32>::MultiplyLong(word))},
                // 0x121 => {Some(bx(word))},
                // 0xA00 | 0xB00 => {Some(branch(word))},
                // 0x400 | 0x600 | 0x700 | 0x780 | 0x7C0 | 0x7E0 | 0x7F0 => {Some(single_data_transfer(word))},
                // 0x09 => {Some(signed_data_transfer(word))}, //This instruction clashes with multiply
                // 0x800| 0x900 | 0x980 | 0x9C0 | 0x9E0 | 0x9F0 => {Some(block_data_transfer(word))},

                // 0x109 | 0x329 => {Some(swap(word))},
                // 0xF00 => {Some(swi(word))},
                // 0xE00 => {Some(cdp(word))},
                // 0xC00 | 0xD00 | 0xD80 | 0xDC0 | 0xDE0 | 0xDF0 => {Some(cdt(word))},
                // 0xE01 | 0xE21 => {Some(crt(word))},

                //TODO: Verify these numbers
                0x601 => {Some(Instruction::<u32>::Undefined(word))}, //Undefined instruction according to manual
                _ => {unimplemented!("Oh-oh! Unimplemented Instruction: {:?}", filter)},
            }
    }   
}

impl Instruction<u32> {

    pub fn Multiply(word: u32) -> Self {
        
        let _rs = shift_right!(word, 8);
        let rs: u32 = apply_mask!(_rs, 0xF);
        
        let rm: u32 = apply_mask!(word, 0xF);

        let _rn = shift_right!(word, 12);
        let rn: u32 = apply_mask!(_rn, 0xF);

        let _rd = shift_right!(word, 16);
        let rd: u32 = apply_mask!(_rd, 0xF);

        let rd_lo: u32 = 0;//Doesnt apply
        
        let _s = shift_right!(word, 20);
        let set: Bit = Bit::bit_from(apply_mask!(_s, 0x01));//TODO: Properly check this
        
        let _acc = shift_right!(word, 21);
        let accumulate = Bit::bit_from(apply_mask!(_acc, 0x01));

        let signed: Bit = Bit::Unset;//Doesnt matter here 

        let _cond = shift_right!(word, 28);
        let cond = apply_mask!(_cond, 0xF);

        Instruction::<u32> {
            rs,
            rm,
            rd_lo,
            rn,
            rd,

            // Discriminant factors
            tag: Instructions::Multiply,
            set,
            accumulate: Some(accumulate),
            signed: Some(signed),
            
            // Data for all the Instructions
            word,
            cond,
            op_code: OpCode::ARM_OpCode,
            
            //Shift relevant data
            shift_ammount: Some(0),//TODO: FIX THIS ONE
            shift: None,
            
            // Mode of the instruction (ARM or THUMB)
            mode: Mode::ARM,
            cycles: 4,
        }
    }

    pub fn MultiplyLong(word: u32) -> Self {
        let _rs = shift_right!(word, 8);
        let rs: u32 = apply_mask!(_rs, 0xF);
        
        let rm: u32 = apply_mask!(word, 0xF);

        // let _rn = shift_right!(word, 12);
        // let rn: u32 = apply_mask!(_rn, 0xF);

        let _rd = shift_right!(word, 16);
        let rd: u32 = apply_mask!(_rd, 0xF);

        let _rd_lo: u32 = shift_right!(word, 12);
        let rd_lo: u32 = apply_mask!(_rd_lo, 0xf);
        
        let _s = shift_right!(word, 20);
        let set: Bit = Bit::bit_from(apply_mask!(_s, 0x01));//TODO: Properly check this
        
        let _acc = shift_right!(word, 21);
        let accumulate = Bit::bit_from(apply_mask!(_acc, 0x01));

        let _signed = shift_right!(word, 22);
        let signed: Bit = Bit::bit_from(apply_mask!(_signed, 0x01));//Doesnt matter here 

        let _cond = shift_right!(word, 28);
        let cond = apply_mask!(_cond, 0xF);

        Instruction::<u32> {
            rs,
            rm,
            rd_lo,
            rn: 0,
            rd,

            // Discriminant factors
            tag: Instructions::MultiplyLong,
            set,
            accumulate: Some(accumulate),
            signed: Some(signed),
            
            // Data for all the Instructions
            word,
            cond,
            op_code: OpCode::ARM_OpCode,
            
            //Shift relevant data
            shift_ammount: Some(0),//TODO: FIX THIS ONE
            shift: None,
            
            // Mode of the instruction (ARM or THUMB)
            mode: Mode::ARM,
            cycles: 4,
        }
    }

    pub fn DataProcessing(word: u32) -> Self {
        let _rs = shift_right!(word, 8);
        let rs: u32 = apply_mask!(_rs, 0xF);
        
        let rm: u32 = apply_mask!(word, 0xF);

        let _rn = shift_right!(word, 12);
        let rn: u32 = apply_mask!(_rn, 0xF);

        let _rd = shift_right!(word, 16);
        let rd: u32 = apply_mask!(_rd, 0xF);

        let rd_lo: u32 = 0;//Doesnt apply
        
        let _s = shift_right!(word, 20);
        let set: Bit = Bit::bit_from(apply_mask!(_s, 0x01));//TODO: Properly check this
        
        let _acc = shift_right!(word, 21);
        let accumulate = Bit::bit_from(apply_mask!(_acc, 0x01));

        let signed: Bit = Bit::Unset;//Doesnt matter here 

        let _cond = shift_right!(word, 28);
        let cond = apply_mask!(_cond, 0xF);


        Instruction::<u32> {
                rs,
                rm,
                rd_lo,
                rn,
                rd,

                // Discriminant factors
                tag: Instructions::DataProcessing,
                set,
                accumulate: Some(accumulate),
                signed: Some(signed),
                
                // Data for all the Instructions
                word,
                cond,
                op_code: OpCode::ARM_OpCode,
                
                //Shift relevant data
                shift_ammount: Some(0),//TODO: FIX THIS ONE
                shift: Some(Shift::LSL),
                
                // Mode of the instruction (ARM or THUMB)
                mode: Mode::ARM,
                cycles: 4,
        }
    }

    pub fn Undefined(word: u32) -> Self {
        let rs: u32 = 0;
        let rm: u32 = 0;

        let rn: u32 = 0;
        let rd: u32 = 0;

        let rd_lo: u32 = 0;
        
        let set: Bit = Bit::Unset;//TODO: Properly check this
        let accumulate: Bit = Bit::Unset;

        let signed: Bit = Bit::Unset;

        let cond = 0;//TODO: Filter out conditions

        Instruction::<u32> {
                rs: 0,
                rm: 0,
                rd_lo: 0,
                rn: 0,
                rd: 0,

                // Discriminant factors
                tag: Instructions::NOP,
                set : Bit::Unset,
                accumulate: None,
                signed: None,
                
                // Data for all the Instructions
                word,
                cond: 0,
                op_code: OpCode::ARM_OpCode,
                
                //Shift relevant data
                shift_ammount: None,//TODO: FIX THIS ONE
                shift: None,
                
                // Mode of the instruction (ARM or THUMB)
                mode: Mode::ARM,
                cycles: 1,
        }
    }
}

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
    UNDEFINED = 0xff,
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
    Set = 1,
    Unset = 0,
}

impl Bit {
    pub fn bit_from(word:u32)-> Bit{
        match word {
            0 => Bit::Unset,
            1 => Bit::Set,
            _ => {panic!("Illegal word value passed to Bit!");},
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_data_processing_from_word(){
        let word: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0000;
        let inst = Instruction::<u32>::from_word(word).unwrap();
        assert_eq!(inst.tag, Instructions::DataProcessing); 
        assert_eq!(inst.mode, Mode::ARM);
    }

    #[test]
    fn test_nop_from_word(){
        let word: u32 = 0b0000_0110_0000_0000_0000_0000_0001_0000;
        let inst = Instruction::<u32>::from_word(word).unwrap();
        assert_eq!(inst.tag, Instructions::NOP);
        assert_eq!(inst.mode, Mode::ARM);
        assert_eq!(inst.cycles, 1);
    }

    #[test]
    fn test_multiply_from_word(){
        let word: u32 = 0b0000_0000_0011_0011_0000_0000_1001_0000;
        let inst = Instruction::<u32>::from_word(word).unwrap();
        assert_eq!(inst.tag, Instructions::Multiply);
        assert_eq!(inst.mode, Mode::ARM);
        // assert_eq!(inst.cycles, 1); TODO: Determine proper clock cycles taken
    }

    #[test]
    fn test_multiply_long_from_word(){
        let word: u32 = 0b0000_0000_1010_1111_1010_0000_1001_0000;
        let inst = Instruction::<u32>::from_word(word).unwrap();
        assert_eq!(inst.tag, Instructions::MultiplyLong);
        assert_eq!(inst.mode, Mode::ARM);
        assert_eq!(inst.rd, 0xF);// Rd hi -> 15
        assert_eq!(inst.rd_lo, 0xA);// Rd Lo -> 10
        // assert_eq!(inst.accumulate.unwrap(), Bit::Set);
    }

}