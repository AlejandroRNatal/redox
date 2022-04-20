use crate::RAM;
use crate::Timer;
use crate::RegisterFile;
use crate::Instruction;

// 16.78 MHz -> 16.78 x 10 ^ (6) Hz 
const FREQUENCY: f32 = 16.78 * 1_000_000;

enum Flag {
    EQ=0,
    NE=1,
    CS=2,
    CC=3,
    MI=4,
    PL=5,
    VS=6,
    VC=7,
    HI=8,
    LS=9,
    GE=10,
    LT=11,
    GT=12,
    LE=13,
    AL=14,
}

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

pub struct Flags {
    pub z: u32,
    pub v: u32,
    pub c: u32,
    pub n: u32,
}

pub struct CPU {
    pub flags: Flags,
    pub prev: State,
    pub state: State, //To know what state we are going to execute
    pub pc: u32,
    pub sp: u32,
    pub lr: u32,
    pub gp_regs: [u32; 15],// (thumb)SP -> (ARM) r13, (thumb) PC -> (ARM) r15, (thumb) LR -> (ARM) r14 
    pub cpsr: u32, // Current Prog. Status Reg
    pub spsr: u32, // Saved Prog. Status Reg
    pub memory: RAM,
    // pub timer: Timer,
    pub reg_file: RegisterFile,
}

impl CPU {

    pub fn new() -> Self {
        CPU {
            pc: 0,
            sp: 0,
            lr: 0,
            cpsr: 0,
            spsr: 0,
            gp_regs: [0;15],
            flags: Flags::new(),
            prev: State::UNDEFINED,
            state: State::UNDEFINED,
            reg_file: RegisterFile::new(),
            memory: RAM::new(),
        }
    }

    pub fn toggle_state(&mut self, instruction: T )
    where T: Instruction {
        // TODO: Implement instructions!!!
        if true || true { //instruction is BX or Interrupt
            let state_bit = 1;

            match instruction & state_bit {
                0 => { self.prev = self.state; self.state = State::THUMB;},
                1 => { self.prev = self.state; self.state = State::ARM;},
            }
        }
        
    }

    pub fn execute(&mut self, instruction: T)
    where T: Instruction {
        // TODO: Verify this increment as we generate it after executing the instruction
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
}

impl Flags {
    pub fn new() -> Self {
        Flags {
            z: 0,
            v: 0,
            c: 0,
            n: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu(){

    }
}