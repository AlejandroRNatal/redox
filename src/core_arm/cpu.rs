use crate::RAM;
use crate::Timer;
use crate::RegisterFile;

// 16.78 MHz -> 16.78 x 10 ^ (6) Hz 
const FREQUENCY: f32 = 16.78 * 1_000_000;
enum Interrupt {
    Some,
    None
}

enum HardwareInterrupt {
    V_Blank,
    H_Blank,
    Serial,
    V_Count,
    Timer,
    DMA,
    Key,
    Cassette,
}

enum BIOSInterrupt {
    SoftReset=0x00,
    RegisterRamReset=0x01,
    Halt=0x02,
    Stop=0x03,
    InterruptWait=0x04,
    VBlankInterruptWait=0x05,
    Div=0x06,
    DivArm=0x07,
    Sqrt=0x08,
    ArcTan=0x09,
    ArcTan2=0x0A,
    CPUSet=0x0B,
    CPUFastSet=0x0C,
    BiosCheckSum=0x0D,
    BgAffineSet=0x0E,
    ObjAffineSet=0x0F,
    BitUnpack=0x10,
    LZ77UnCompWRAM=0x11,
    LZ77UnCompVRAM=0x12,
    HuffUnComp=0x13,
    RLUnCompWRAM=0x14,
    RLUnCompVRAM=0x15,
    Diff8bitUnFilterWRAM=0x16,
    Diff8bitUnFilterVRAM=0x17,
    Diff16bitUnFilter=0x18,
    SoundBiasChange=0x19,
    SoundDriverInit=0x1A,
    SoundDriverMode=0x1B,
    SoundDriverMain=0x1C,
    SoundDriverVSync=0x1D,
    SoundChannelClear=0x1E,
    MIDIKey2Freq=0x1F,
    MusicPlayerOpen=0x20,
    MusicPlayerStart=0x21,
    MusicPlayerStop=0x22,
    MusicPlayerContinue=0x23,
    MusicPlayerFadeOut=0x24,
    MultiBoot=0x25,
    HardReset=0x26,
    CustomHalt=0x27,
    SoundDriverVSyncOff=0x28,
    SoundDriverVSyncOn=0x29,
    GetJumpList=0x2A,
}

enum State {
    ARM(i32),
    THUMB(i16),
    UNDEFINED(i8),
}

enum ARM_Instruction {
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

enum THUMB_Instruction {
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

pub struct CPU {
    pub prev: State,
    pub state: State, //To know what state we are going to execute
    pub pc: i32,
    pub sp: i32,
    pub lr: i32,
    pub gp_regs: [i32; 15],// (thumb)SP -> (ARM) r13, (thumb) PC -> (ARM) r15, (thumb) LR -> (ARM) r14 
    pub cpsr: i32, // Current Prog. Status Reg
    pub spsr: i32, // Saved Prog. Status Reg
    pub memory: RAM,
    pub timer: Timer,
}

impl CPU {

    pub fn new() -> CPU {
        CPU {
            prev: State::UNDEFINED,
            state: State::UNDEFINED,
            reg_file: RegisterFile::new(),
            memory: RAM::new(),
        }
    }

    pub fn toggle_state(&mut self, instruction: i32) {
        // TODO: Implement instructions!!!
        if true || true { //instruction is BX or Interrupt
            let state_bit = 1;

            match instruction & state_bit {
                0 => { self.prev = self.state; self.state = State::THUMB;},
                1 => { self.prev = self.state; self.state = State::ARM;},
            }
        }
        
    }

    pub fn execute(&mut self, instruction: Instruction) {
          //TODO: Verify this increment as we generate it after executing the instruction
        let increment = match self.prev {
          State::ARM => 2 as i32,
          State::THUMB => 4 as i32,
          State::UNDEFINED => panic!("Undefined State found before PC increment!"),
        };

        match instruction {
            _=> {}
        }

        self.increment_pc(increment)
    }

    fn increment_pc(&mut self, increment: i32){
        self.pc += increment;
    }

    pub fn from_byte(&mut self, byte: u32) -> Option<Instruction> {
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
    fn data_operand(&mut self, byte: u32) -> Option<Instruction> {
        let condition: u8 = ((byte) & (0x0F << 28));
        // let operand_2: u16 = 0;
        let immediate: u8 = (((byte) && ( 1 << 25 )) >> 25 ) as u8;
        match immediate {
            Bit::Set => {/*Not immediate Operation*/ non_immediate(byte)},
            Bit::Unset => {/* Immediate Operation */ immediate(byte)},
            _ => { None },
        }
    }

    fn multiply(&mut self, byte: u32) -> Option<Instruction> {
        
    }

    fn multiply_long(&mut self, byte: u32) -> Option<Instruction> {
        
    }

    fn single_data_transfer(&mut self, byte: u32) -> Option<Instruction> {

    }

    fn block_data_transfer(&mut self, byte: u32) -> Option<Instruction> {

    }

    fn swap(&mut self, byte: u32) -> Option<Instruction> {

    }

    fn swi(&mut self, byte: u32) -> Option<Instruction> {

    }

    fn cdp(&mut self, byte: u32){

    }

    fn cdt(&mut self, byte: u32){

    }

    fn crt(&mut self, byte: u32){

    }

    fn bx(&mut self, byte: u32){
        let conditions: u8 = ( (byte & 0xF0000000) >> 28) as u8;
        let link: u8 = ( (byte & (0x01 << 24)) >> 24)  as u8;

        let offset: u32 =((byte) & (0xFF_FFFF)) <<  2;// Shifted 2 as specified by Docs
        if conditions == self.cpsr {//this check is wrong but whatever
            if link == (Bit::Set) {
                    // write old PC -> R14 of current bank
                    // Must adjust PC to compensate for prefetch operation
                    // CPSR is not saved with PC, R14[1:0] are cleared always
                    
                    self.gp_regs[14] = self.pc - 8;//TODO: Fix the bug here with prefetching for pipeline
                    self.gp_regs[14] = self.gp_regs[14] & 0xFFFF_FFFC;//clears bits [1;0] as specified
            }

            self.pc += (offset - 8);//TODO: This might be the problem ; subtract 8 bytes?
            
        }
     
    }

    fn branch(&mut self, byte: u32){

    }

    fn non_immediate(&mut self, byte: u32) {
        let shift: u8 = ((byte & 0xff0) >> 4) as u8;// byte from bits 4-11
        let rm: u8 = ((byte) & 0x0f ) as u8; // nibble from bits 0-3
        let rd = (byte & 0xF000) >> 12;//destination
        let rn = (byte & 0xF0000) >> 16;//operand 1
        let check: u8 = (shift & 0x01) as u8;

        let op_code = ((byte & 0x1E00000) >> 21) as u8;


        match check {
            Bit::Unset => {
                let sh_type: u8 = (shift & 0x06) as u8;
                let sh_amount: u8 = (shift & 0xF8) as u8;
                
                match sh_type {
                    Shift::LSL => {
                        let LSB = (self.gp_regs[rm as u32] & (1 << (31 - sh_amount))) >> (31-sh_amount) ;//TODO: Check if bug here with storing LSB
                        let res = self.gp_regs[rm as u32]  << sh_amount ;
                        
                        match op_code {
                            ARM_Instruction::AND => {self.gp_regs[rd] = self.gp_regs[rn] & rm;},
                            ARM_Instruction::EOR => {self.gp_regs[rd] = self.gp_regs[rn] ^ rm;},
                            ARM_Instruction::SUB => {self.gp_regs[rd] = self.gp_regs[rn] - rm;},
                            ARM_Instruction::RSB => {self.gp_regs[rd] = rm - self.gp_regs[rn];},
                            ARM_Instruction::ADD => {self.gp_regs[rd] = self.gp_regs[rn] + rm;},
                            ARM_Instruction::ADC => {self.gp_regs[rd] = self.gp_regs[rn] + rm + LSB},
                            ARM_Instruction::SBC => {self.gp_regs[rd] = self.gp_regs[rn] - rm + LSB - 1},
                            ARM_Instruction::RSC => {self.gp_regs[rd] =  rm - self.gp_regs[rn] + LSB - 1},
                            ARM_Instruction::TST => {},
                            ARM_Instruction::TEQ => {},
                            ARM_Instruction::CMP => {},
                            ARM_Instruction::CMN => {},
                            ARM_Instruction::ORR => {},
                            ARM_Instruction::MOV => {},
                            ARM_Instruction::MVN => {},
                            ARM_Instruction::BIC => {},
                        }
                        
                        //now we  OR LSB to CPSR's C bit
                        if sh_amount != 0 {//Only if shift amount is not 0! Special case
                            self.cpsr |= LSB << 28;// C-bit  in CPSR is #29
                        }
                    },
                    Shift::LSR => {},
                    Shift::ARS => {},
                    Shift::RR  => {},
                }
            },
            Bit::Set => {},
            _ => {panic!("Something went wrong: {:?}", _)}
        }

    }

    fn immediate(&mut self, byte: u32) {
        //Immediate Rotate is always a rotate right
        let op_code = ((byte && 0x1E0_0000 ) >> 21) as u8;
        let flags = ((byte) && (0xF000_0000_0000_0000) >> 28);
        
        //This may be wrong
        let rotate: u8 = (( byte && (0xF00) ) >> 8) as u8;
        let immediate: u8 = (( byte ) && (0xFF)) as u8;

        
        let rn: u8 = ( ( byte && 0xF_0000) >> 16) as u8 ;
        let rd: u8 = ( ( byte && 0xF000) >> 12) as u8;
        
        let operand2: u8 = immediate >> (2 * rotate);
        match op_code {
            //TODO AND may be wrong here, blog says it is supposed to be AND Rd, Rn, Rm LSL #5;
            ARM_Instruction::AND => {arm_imm_and(flags, rn, rd, operand2 )}, //Logical
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

    fn arm_imm_and(&mut self, flags: u8, source: u8, destination: u8, operand_2: u8) -> Result<Ok, Err> {
        //Store result of source & operand_2 into destination
        if flags {
            //TODO apply overflow logic here
            self.gp_reg[destination as u32] = cpu.gp_reg[source as u32].overflowing_and(operand_2 as u32);
            Ok()
        }
        else{
            Err()
        }
    }

}


mod tests {

}