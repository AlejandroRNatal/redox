#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::{File};
use std::io::{prelude::*};
mod core_arm;

mod prelude {
    pub use crate::core_arm::*;
}


fn load_bios() -> [u8; 16384] {
    let filename  = "src/gba_bios.bin";
    let mut file = File::open(filename);

    let mut buffer = [0;16384];
    let res = match &mut file {

        Ok(file) => file.read_exact(& mut buffer),
        Err(error) => panic!("Problem opening BIOS file: {:?}", error)

    };

    print!("{:?}",res);

    buffer
}

fn load_rom() -> Vec<u8> {
    let memory_bus = core_arm::ram::RAM::new();

    let result = memory_bus.load_rom("src/pokemon_emerald.GBA");

    let v = match result {

        Ok(result) => {result},
        Err(error) => {print!("Something went wrong: {error}"); Vec::<u8>::new()},
    };

    v
}

fn main() -> Result<(), ()> {
    
    // let bios = load_bios();
    // println!("BIOS:");
    // for i in bios.into_iter() {
    //     print!("{:#01x}\t", i);
    // }
    // print!("{:?}", bios);
    // println!("-------\n");

    // println!("ROM:");
    // let rom = load_rom();

    // for i in rom.into_iter() {
    //     print!("{:#01x}\t", i);
    // }
    // print!("{:?}", rom);
    // println!("-------\n");
    Ok(())
}
