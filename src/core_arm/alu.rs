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
    
    pub fn arm_imm_op(&self, source: T, destination: T, operation:&dyn for<'a> Fn(&'a u32, &'a u32) -> u32, operand2: T){

    }

    pub fn main(&self) {
        // closure for addition
        self.arm_imm_op(x, y, &| x , y | x.overflowing_add(y))
    }
}

   fn arm_imm_and(&mut self, flags: u8, source: u8, destination: u8, operand_2: u8, mod_conds: u8) {
        //Store result of source & operand_2 into destination
        if self.check_flags(flags) {
            
            self.reg_file.write_gp(destination as usize,
                                   self.reg_file.read_gp(source as usize) & (operand_2 as u32));
            if mod_conds == 1 {
                // modify cpsr condition bits
                if destination != 15 {// v flag unaffected 
                    // C will be set from carry-out of barrel shifter

                }
            }
        }
        
    }

    fn arm_imm_eor(&mut self, flags: u8, source: u8, destination: u8, operand2: u8, mod_conds: u8) {
        if self.check_flags(flags) {
            self.reg_file.write_gp(destination as usize,
                                    self.reg_file.read_gp(source as usize) ^ (operand2 as u32));
        }
    }

    fn arm_imm_sub(&mut self, flags: u8, source: u8, destination: u8, operand2: u8, mod_conds: u8) {
        if self.check_flags(flags){
            self.reg_file.write_gp(destination as usize,
                                    self.reg_file.read_gp(source as usize)
                                                  .overflowing_sub(operand2 as u32));
        }
    }

    fn arm_imm_rsb(&mut self, flags: u8, source: u8, destination: u8, operand2: u8, mod_conds: u8){
        if self.check_flags(flags){
            self.reg_file.write_gp(destination as usize,
                                    (operand2 as usize).overflowing_sub(self.reg_file.read_gp(source as usize)));
        }
    }

    fn arm_imm_add(&mut self, flags: u8, source: u8, destination: u8, operand2: u8, mod_conds: u8){
        if self.check_flags(flags){
            self.reg_file.write_gp(destination as usize,
                                   self.reg_file.read_gp(source as usize)
                                                .overflowing_add(operand2));
        }
    }

    fn arm_imm_adc(&mut self, flags: u8, source: u8, destination: u8, operand2: u8, mod_conds: u8){
        if self.check_flags(flags){
            let carry = self.flags.c as u32;
            self.reg_file.write_gp(destination as usize,
                                    self.reg_file.read_gp(source as usize)
                                                 .overflowing_add(operand2 as u32)
                                                 .overflowing_add(carry));
        }
    }
    
    fn arm_imm_sbc(&mut self, flags: u8, source: u8, destination: u8, operand2: u8, mod_conds: u8){
        if self.check_flags(flags){
            let carry = (! self.flags.c as u32) & 0x01;
            self.reg_file.write_gp(destination as usize,
                                    self.reg_file.read_gp(source as usize)
                                                 .overflowing_sub(operand2 as u32)
                                                 .overflowing_sub(carry));
        }
    }

    fn arm_imm_rsc(&mut self, flags: u8, source: u8, destination: u8, operand2: u8, mod_conds: u8){
        if self.check_flags(flags){
            let carry = (! self.flags.c as u32) & 0x01;
            self.reg_file.write_gp(destination as usize,
                                    (operand2 as u32)
                                                 .overflowing_sub(self.reg_file.read_gp(source as usize))
                                                 .overflowing_sub(carry));
        }
    }

    fn arm_imm_tst(&mut self, flags: u8, source: u8, destination: u8, operand2: u8, mod_conds: u8){

    }

    
    fn check_flags(&self, flags: u8) -> bool {
              use super::Flag;
        //CPSR has condition flags bits 28-31
        let conditions = ((self.cpsr  & 0xF000_0000) >> 28) as u8;
        
        let n: u8 = (8 & conditions) >> 3;
        let z: u8 = (4 & conditions) >> 2;
        let c: u8 = (2 & conditions) >> 1;
        let v: u8 = (1 & conditions) ;

        match flags {
            EQ => { z == 1 },
            NE => { z == 0 },
            CS => { c == 1 },
            CC => { c == 0 },
            MI => { n == 1 },
            PL => { n == 0 },
            VS => { v == 1 },
            VC => { v == 0 },
            HI => { c == 1 && z == 0 },
            LS => { c == 0 && z == 1 },
            GE => { n == v },
            LT => { n != v },
            GT => { z == 0 && n == v },
            LE => { z == 1 || n != v },
            AL => { true },
            _ => { panic!("Unexpected flag: {:?}", _ )},
        }
    }
