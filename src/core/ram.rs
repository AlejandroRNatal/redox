mod RAM{
    enum Mode{
        Byte,
        Halfword,
        Word,
        DoubleWord
    }
    
    enum State{
        Read,
        Write,
        Undefined
    }
    
    pub struct RAM {
        bios: [u32; 16384], //BIOS
        iwram: [u32; 32768],// 32 KBytes
        ewram: [u16; 262144],// 256 KBytes
        vram: [u16; 98304], // 96 Kbytes
        game_rom: Vector<u16>,
        game_ram: Vector<u8>,
        mode: Mode,
        state: State,
    }
    
    impl RAM {
        
        pub fn new(self) -> Self{
            RAM {
                bios: [0;16384],
                iwram: [0; 32768],
                ewram: [0; 262144],// 256 KBytes
                vram: [0; 98304], // 96 Kbytes
                game_rom: Vector<u16>::new(),
                game_ram: Vector<u8>::new(),
                mode: Mode::Word,
                state: State::Read,
  
            }
        }


        fn load_bios(self){
            let filename  = "src/gba_bios.bin";
            let mut file = File::open(filename);

            let mut buffer = [0;16384];
            let res = match &mut file {

                Ok(file) => file.read_exact(& mut buffer),
                Err(error) => panic!("Problem opening BIOS file: {:?}", error)

            };


            for (index, b) in buffer.into_iter().enumerate() {
                self.bios[index] = b;
            }
        }


        pub fn read_bytes(&self, address: u32) {
            let res = match address {
               0x0..= 0x0003fff => { /** Illegal access of Boot ROM */ panic!("Illegal BIOS Address Read: {}", address)},              
               0x02000000 ..= 0x0203FFFF => {/** EW RAM */},
               0x03000000 ..= 0x03007FFF => {/** IW RAM */},
               0x04000000 ..= 0x040003FF => {/** IO RAM */},
               0x05000000 ..= 0x050003FF => {/** Palette RAM */},
               0x06000000 ..= 0x06017FFF => {/** VRAM*/},
               0x07000000 ..= 0x070003FF => {/** Object Attributable Memory */},
               
               // ROM special Cases
               0x08000000 ..= 0x0CFFFFFF => {/** Game Pak ROMs */},

               // Cart RAM
               0x0E000000 ..= 0x0EFFFFFF | 
               0x0F000000 ..= 0x0FFFFFFF => {/** CART RAM */},

                _ => {panic!("Unknown Memory Address: {}", address)},

            };

            res
        }
    
        pub fn read_byte_ram(&self, address: u16) {
            match address {
                0x0..= 0x0003fff => { /** Illegal access of Boot ROM */
                                        panic!(format!("Illegal access at address: {:?}", address))},
                    
                _ => { self.iwram[address] },    
            }
        }
    
        pub fn read_byte_rom(&self, address: u16) {
            match address {
                _ => { panic!("Unimplemented")}
            }
        }
    }
}
