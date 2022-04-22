
pub mod RegisterFile {
    pub const PC_ADDRESS: usize = 15;
    pub const SP_ADDRESS: usize = 13;

    #[derive(Copy, Clone, Debug)]
    pub enum Mode {
        // PSR_MODE=0x1f, 
        PSR_MODE_USER=0x10,
        PSR_MODE_SYS=0x1f,
        PSR_MODE_FIQ=0x11,
        PSR_MODE_IRQ=0x12,
        PSR_MODE_SVC=0x13,
        PSR_MODE_ABT=0x17,
        PSR_MODE_UND=0x1b,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct RegisterFile {
        pub mode: Mode,
        pub gp_registers: [u32; 16],

        pub r8_user: u32,
        pub r9_user: u32,
        pub r10_user: u32,
        pub r11_user: u32,
        pub r12_user: u32,
        pub r13_user: u32,
        pub r14_user: u32,
        
        pub r8_fiq: u32,
        pub r9_fiq: u32,
        pub r10_fiq: u32,
        pub r11_fiq: u32,
        pub r12_fiq: u32,
        pub r13_fiq: u32,
        pub r14_fiq: u32,
        
        pub r13_irq: u32,
        pub r14_irq: u32,

        pub r13_svc: u32,
        pub r14_svc: u32,

        pub r13_abt: u32,
        pub r14_abt: u32,

        pub r13_und: u32,
        pub r14_und: u32,

        pub cpsr: u32,
        pub spsr: u32,
        pub spsr_fiq: u32,
        pub spsr_irq: u32,
        pub spsr_svc: u32,
        pub spsr_abt: u32,
        pub spsr_und: u32,
    }

    impl RegisterFile {
        pub fn new() -> Self {
            RegisterFile {
                mode: Mode::PSR_MODE_UND,//TODO: Change this to expected mode on init
                gp_registers: [0; 16],
                r8_user: 0,
                r9_user: 0,
                r10_user: 0,
                r11_user: 0,
                r12_user: 0,
                r13_user: 0,
                r14_user: 0,
                
                r8_fiq: 0,
                r9_fiq: 0,
                r10_fiq: 0,
                r11_fiq: 0,
                r12_fiq: 0,
                r13_fiq: 0,
                r14_fiq: 0,
                
                r13_irq: 0,
                r14_irq: 0,

                r13_svc: 0,
                r14_svc: 0,

                r13_abt: 0,
                r14_abt: 0,

                r13_und: 0,
                r14_und: 0,

                cpsr: 0,
                spsr_fiq: 0,
                spsr_irq: 0,
                spsr_svc: 0,
                spsr_abt: 0,
                spsr_und: 0,
            }
        }
        
        pub fn mode_change(&mut self, prev: Mode, current: Mode) {
            if prev == current {}

            if prev == Mode::PSR_MODE_USER && current == Mode::PSR_MODE_SYS {}

            if prev == Mode::PSR_MODE_SYS && current == Mode::PSR_MODE_USER {}

            if prev == MODE::PSR_MODE_FIQ {
                self.r8_fiq = self.gp_registers[8];
                self.r9_fiq = self.gp_registers[9];
                self.r10_fiq = self.gp_registers[10];
                self.r11_fiq = self.gp_registers[11];
                self.r12_fiq = self.gp_registers[12];

                self.gp_registers[8] = self.r8_user;
                self.gp_registers[9] = self.r9_user;
                self.gp_registers[10] = self.r10_user;
                self.gp_registers[11] = self.r11_user;
                self.gp_registers[12] = self.r12_user;
            }
            
            if current == MODE::PSR_MODE_FIQ {
                self.r8_user = self.gp_registers[8];
                self.r9_user = self.gp_registers[9];
                self.r10_user = self.gp_registers[10];
                self.r11_user = self.gp_registers[11];
                self.r12_user = self.gp_registers[12];

                self.gp_registers[8] = self.r8_fiq;
                self.gp_registers[9] = self.r9_fiq;
                self.gp_registers[10] = self.r10_fiq;
                self.gp_registers[11] = self.r11_fiq;
                self.gp_registers[12] = self.r12_fiq;
            }
            
            match(prev){
                Mode::PSR_MODE_USER => {},

                Mode::PSR_MODE_SYS => {
                    self.r13_user = self.gp_registers[13];
                    self.r14_user = self.gp_registers[14];
                },

                Mode::PSR_MODE_FIQ => {
                    self.r13_fiq = self.gp_registers[13];
                    self.r14_fiq = self.gp_registers[14];
                },

                Mode::PSR_MODE_IRQ => {
                    self.r13_irq = self.gp_registers[13];
                    self.r14_irq = self.gp_registers[14];
                },
                
                Mode::PSR_MODE_SVC => {
                    self.r13_svc = self.gp_registers[13];
                    self.r14_svc = self.gp_registers[14];
                },

                Mode::PSR_MODE_ABT => {
                    self.r13_abt = self.gp_registers[13];
                    sefl.r14_abt = self.gp_registers[14];
                },

                Mode::PSR_MODE_UND => {
                    self.r13_und = self.gp_registers[13];
                    self.r14_und = self.gp_registers[14];
                }

                _ => {panic!("Undefined Mode Received in Register File!")}
            }

            match(current){
                Mode::PSR_MODE_USER => {},

                Mode::PSR_MODE_SYS => {
                    self.gp_registers[13] = self.r13_user;
                    self.gp_registers[14] = self.r14_user;
                },

                Mode::PSR_MODE_FIQ => {
                    self.gp_registers[13] = self.r13_fiq;
                    self.gp_registers[14] = self.r14_fiq;
                },

                Mode::PSR_MODE_IRQ => {
                    self.gp_registers[13] = self.r13_irq;
                    self.gp_registers[14] = self.r14_irq;
                },
                
                Mode::PSR_MODE_SVC => {
                    self.gp_registers[13] = self.r13_svc;
                    self.gp_registers[14] = self.r14_svc;
                },

                Mode::PSR_MODE_ABT => {
                    self.gp_registers[13] = self.r13_abt;
                    self.gp_registers[14] = sefl.r14_abt;
                },

                Mode::PSR_MODE_UND => {
                    self.gp_registers[13] = self.r13_und;
                    self.gp_registers[14] = self.r14_und;
                }

                _ => {panic!("Undefined Mode Received in Register File!")}
            }
            
            self.mode = current;
        }

        pub fn init_registers(&mut self, skip_bios: bool) {
            if skip_bios{
                self.write_sp(0x03007f00);
                self.write_pc(0x08000000);
                self.r13_irq = 0x03007fa0;
                self.r13_svc = 0x03007fe0;
                self.cpsr = Mode::PSR_MODE_SYS;
            }else {
                self.write_pc(0);//TODO: write reset vector
                self.cpsr = Mode::PSR_MODE_SVC; //TODO: Set Flags heer I | F | MODE_SVC
            }
        }

        pub fn stack_pointer(&self) -> u32{
            self.gp_registers[SP_ADDRESS]
        }

        pub fn program_counter(&self) -> u32{
            self.gp_registers[PC_ADDRESS]
        }

        pub fn lr(&self) -> u32 {
            self.gp_registers[14]
        }

        pub fn write_gp(&mut self, destination: usize, data: u32){
            self.gp_registers[destination] = data;
        }

        pub fn write_sp(&mut self, data: u32){
            self.gp_registers[SP_ADDRESS] = data;
        }

        pub fn write_pc(&mut self, data: u32){
            self.gp_registers[PC_ADDRESS] = data;
        }

        pub fn write_lr(&mut self, data: u32){
            self.gp_registers[14] = data;
        }

        pub fn read_gp(&self, source: usize) -> u32 {
            self.gp_registers[source]
        }

        pub fn read_spsr(&self) -> u32 {
            use super::Mode;
            let mode = self.cpsr & PSR_MODE;

            match(mode) {
                PSR_MODE_USER => {self.cpsr},
                PSR_MODE_FIQ => {self.spsr_fiq},
                PSR_MODE_IRQ => {self.spsr_irq},
                PSR_MODE_SVC => {self.spsr_svc},
                PSR_MODE_ABT => {self.spsr_abt},
                PSR_MODE_UND => {self.spsr_und},
                PSR_MODE_SYS => {self.cpsr},
                _ => {panic!("Something went wrong wit PSR!")}
            }
        }
    
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_register() {
        let register_file: RegisterFile = RegisterFile::new();

        assert_eq!(register_file.mode, Mode::PSR_MODE_SYS);
        assert_eq!(register_file.gp_registers[0], 0);

        assert_eq!(register_file.cpsr, 0);
        assert_eq!(register_file.spsr, 0);
       
        assert_eq!(register_file.r8_user, 0);
        assert_eq!(register_file.r8_fiq, 0);

        assert_eq!(register_file.r13_irq, 0);
        assert_eq!(register_file.r13_abt, 0);
        assert_eq!(register_file.r13_svc, 0);
        assert_eq!(register_file.r13_und, 0);
        
        assert_eq!(register_file.spsr_fiq, 0 );
        assert_eq!(register_file.spsr_irq, 0 );
        assert_eq!(register_file.spsr_svc, 0 );
        assert_eq!(register_file.spsr_abt, 0 );
        assert_eq!(register_file.spsr_und, 0 );
    }

    // #[test]
    // fn test_init_use_bios(){
    //     let mut register_file = RegisterFile::new();
    //     register_file.init_registers(true);

    //     assert_eq!(register_file.cpsr, Mode::PSR_MODE_SVC);
    //     assert_eq!(register_file.program_counter(), 0 as u32);
    // }

    #[test]
    fn test_init_no_bios(){
        let mut register_file = RegisterFile::new();
        register_file.init_registers(false);

        assert_eq!(register_file.cpsr, Mode::PSR_MODE_SYS);
        assert_eq!(register_file.program_counter(), 0x08000000 as u32);
        assert_eq!(register_file.stack_pointer(), 0x03007f00 as u32);

        assert_eq!(register_file.r13_irq, 0x03007fa0 as u32);
        assert_eq!(register_file.r13_svc, 0x03007fe0 as u32);
    }

}