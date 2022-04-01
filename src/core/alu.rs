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

struct ControlFlags {
    z: u32,
    n: u32,
    v: u32,
    cout: u32
}

impl ALU {
    
}