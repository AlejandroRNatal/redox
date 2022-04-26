pub mod decoder;

struct Decoder {

}


pub fn decode_u32(word: u32) -> Option<Instruction<u32>> {
    let filter: u32 = (((word >> 16) & 0xFF0) | ((word >> 20) & 0x0F));

    match filter {
        0 => {/* DATA Processing / PSR Instruction here*/ data_operand(word)},
        /*0x09 |*/ 0x19 | 0x39  => {multiply(word)},
        0x89 | 0xC9 | 0xE9 | 0xF9 => {multiply_long(word)},
        0x121 => {bx(word)},
        0xA00 | 0xB00 => {branch(word)},
        0x400 | 0x600 | 0x700 | 0x780 | 0x7C0 | 0x7E0 | 0x7F0 => {single_data_transfer(word)},
        0x09 => {signed_data_transfer(word)}, //This instruction clashes with multiply
        0x800| 0x900 | 0x980 | 0x9C0 | 0x9E0 | 0x9F0 => {block_data_transfer(word)},
        0x109 | 0x329 => {swap(word)},
        0xF00 => {swi(word)},
        0xE00 => {cdp(word)},
        0xC00 | 0xD00 | 0xD80 | 0xDC0 | 0xDE0 | 0xDF0 => {cdt(word)},
        0xE01 | 0xE21 => {crt(word)},
        0x301 => {None}, //Undefined instruction according to manual
        _ => {None},//Unimplemented instruction
    }
}

 /// These instructions take only 1 CPU Cycle
fn data_operand( byte: u32) {
    let condition: u8 = ((byte) & (0x0F << 28));
    let immediate_op: u8 = (((byte) && ( 1 << 25 )) >> 25 ) as u8;
    match immediate_op {
        Bit::Set => {/*Not immediate Operation*/ non_immediate(byte)}, // Op2 this is a register
        Bit::Unset => {/* Immediate Operation */ immediate(byte)}, //Op2 Immediate value
        _ => { None },
    }
}

fn non_immediate( byte: u32) {
    let shift: u8 = ((byte & 0xff0) >> 4) as u8;// byte from bits 4-11 applied to RM
    let rm: u8 = ((byte) & 0x0f ) as u8; // nibble from bits 0-3 | 2nd operand register
    let rd = (byte & 0xF000) >> 12;//destination
    let rn = (byte & 0xF0000) >> 16;//operand 1
    
    
    let check: u8 = (shift & 0x01) as u8;// first bit of shift applied to RM

    let op_code = ((byte & 0x1E00000) >> 21) as u8;


    match check {
        Bit::Unset => {Some(DataProcessingImmShift::from_word(byte))},
        Bit::Set => {Some(DataProcessingRegShift::from_word(byte))},
        _ => None,
    }
}

fn immediate( byte: u32) {
    Some(DataProcessingImmAmountShift::from_word(byte))
}

fn multiply(word: u32) -> Option<Instruction>{
    Some(Multiply::from_word(word))
}

fn multiply_long(word: u32) -> Option<Instruction>{
    Some(MultiplyLong::from_word(word))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_u32() {
        let word: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0000;

        let actual = decode_u32(word).unwrap();
        // assert_eq!()
    }
}