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

struct CPU {
    prev: State,
    state: State, //To know what state we are going to execute
    pc: i32,
    sp: i32,
    lr: i32,
    gp_regs: [i32; 15],// (thumb)SP -> (ARM) r13, (thumb) PC -> (ARM) r15, (thumb) LR -> (ARM) r14 
    cpsr: i32, // Current Prog. Status Reg
    spsr: i32, // Saved Prog. Status Reg
    memory: RAM,
    timer: Timer,
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
}


mod