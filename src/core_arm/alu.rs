struct ALU {
    flags: ControlFlags,
}

enum OpCode {
    AND = 0,
    EOR = 1,
    SUB = 2,
    RSB = 3,
    ADD = 4,
    ADC = 5,
    SBC = 6,
    RSC = 7,
    TST = 8,
    TEQ = 9,
    CMP = 10,
    CMN = 11,
    ORR = 12,
    MOV = 13,
    BIC = 14,
    MVN = 15,
}

pub enum Conditions {
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
    UNDEFINED=0x0F,
}

struct ControlFlags {
    z: u32,
    n: u32,
    v: u32,
    cout: u32
}

impl ALU {
    
}