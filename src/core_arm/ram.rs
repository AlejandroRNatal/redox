use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::result::Result;
// use std::path::Path;

// use std::error::Error;

#[derive(Clone, Copy, Debug)]
pub enum Mode{
    Byte,
    Halfword,
    Word,
    DoubleWord
}

#[derive(Clone, Copy, Debug)]
pub enum State{
    Read,
    Write,
    Undefined
}

#[derive( Debug)]
pub struct RAM {
    pub bios:  Vec::<u32>, //BIOS
    pub iwram:  Vec::<u32>,// 32 KBytes
    pub ewram:  Vec::<u32>,// 256 KBytes
    pub vram:   Vec::<u32>, // 96 Kbytes
    pub game_rom:  Vec::<u32>,
    pub game_ram:  Vec::<u32>,
    pub mode:  Mode,
    pub state:  State,
}

impl RAM{

    pub fn new() -> Self{
        RAM {
            bios: Vec::<u32>::with_capacity(16_384),
            iwram: Vec::<u32>::with_capacity(32_768),
            ewram: Vec::<u32>::with_capacity(262_144),// 256 KBytes
            vram:  Vec::<u32>::with_capacity(98_304), // 96 Kbytes
            game_rom:   Vec::<u32>::with_capacity(1024),
            game_ram: Vec::<u32>::new(),
            mode:  Mode::Word,
            state: State::Read,

        }
    }


    pub fn load_bios(&mut self){
        let filename  = "src/gba_bios.bin";
        let mut file = File::open(filename);

        let mut buffer = [0;16384];
        let res = match &mut file {

            Ok(file) => file.read_exact(& mut buffer),
            Err(error) => panic!("Problem opening BIOS file: {:?}", error)

        };


        for b in buffer.iter() {
            self.bios.push(*b as u32);
        }
    }

    /// Copies buffer representation of ROM into structs ROM member.
    /// ROM stores data in u32 size and may not be inline with what CPU expects.
    pub fn load_rom_to_internal(&mut self, buffer: Vec::<u8>){
        // we need to pile up 4 at a time

        let mut i: u32 = 0;
        let mut curr: u32 = 0;

        
        for byte in buffer.iter() {
            if i == 3 {
                //write into internal rom repr
                curr |= (0xff) & (*byte as u32);
                self.game_rom.push(curr);
                curr = 0;

                i = 0;
            }

            else{
                let shift = (0xff) & (*byte as u32);
                curr |= shift << ( 24 - (8 * i));
                i += yeah1;
            }
            
        }
    }


    /// Loads a ROM given a file if it exists, else returns an error
    pub fn load_rom(&self, game_path: &str) -> Result<Vec::<u8>, io::Error>{
        let path = std::path::Path::new(game_path);
        let exists = path.is_file();
        
        if exists {
            let file = File::open(path)?;
            let mut reader = BufReader::new(file);

            let mut rom = Vec::<u8>::new();

            let total = reader.read_to_end(&mut rom);
            
            Ok(rom)
        }
        else{
            Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to load ROM file"))
        }
    }

    pub fn read_bytes(&self, address: u32) -> u32 {
        let res = match address {
            0x0..= 0x0003fff => { /* Illegal access of Boot ROM */ panic!("Illegal BIOS Address Read: {}", address)},              
            // 0x02000000 ..= 0x0203FFFF => {/* EW RAM */},
            // 0x03000000 ..= 0x03007FFF => {/* IW RAM */},
            // 0x04000000 ..= 0x040003FF => {/* IO RAM */},
            // 0x05000000 ..= 0x050003FF => {/* Palette RAM */},
            // 0x06000000 ..= 0x06017FFF => {/* VRAM*/},
            // 0x07000000 ..= 0x070003FF => {/* Object Attributable Memory */},
            
            // ROM special Cases
            0x08000000 ..= 0x0CFFFFFF => {/* Game Pak ROMs */self.read_word_rom((address-0x08000000).try_into().unwrap())},//apply offset to correctly index inside vec repr

            // Cart RAM
            // 0x0E000000 ..= 0x0EFFFFFF | 
            // 0x0F000000 ..= 0x0FFFFFFF => {/* CART RAM */},

            _ => {panic!("Unknown Memory Address: {}", address)},

        };

        res
    }

    pub fn read_byte_ram(&self, address: u16)  -> u32{
        match address {
            0x0..= 0x0003fff => { /** Illegal access of Boot ROM */
                                    panic!("Illegal access at address: {:?}", address)},
                
            _ => { self.iwram[address as usize] },    
        }
    }

    pub fn read_word_rom(&self, address: u16) -> u32 {
        // match address {
        //     0x00 => { self.game_rom[address as usize]}
        //     _ => { panic!("Unimplemented")}
        // }
        self.game_rom[address as usize]
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_rom() {
        let rom = "src/pokemon_emerald.GBA";
        let mut ram: RAM = RAM::new();
        
        let buff = ram.load_rom(&rom);

        let buff =  match buff {
            Ok(res) => res,
            Err(error) => panic!("Could not open ROM: {:?}", error),
        };

        (&mut ram).load_rom_to_internal(buff);

        for word in ram.game_rom.iter() {
            assert_eq!(word, word, "Check byte is byte xD");
            print!("{:#32b}", word);
            break;
        }
    }

    #[test]
    fn test_load_bios() {
        
        let mut ram: RAM = RAM::new();


        ram.load_bios();

        for word in ram.bios.iter() {
            print!("{:#x}", word);
            break;
        }
    }


}