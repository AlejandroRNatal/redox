
use std::fs::{File};
// use std::env;
use std::io::{prelude::*};
// use std::path::Path;

mod core_arm;

mod prelude {
    pub use crate::core_arm::*;
}

use prelude::*;

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

// fn load_rom() -> Vec<u8> {
//     let mut ram = core_arm::ram::RAM::RAM::new();

//     let result = ram.load_rom("src/pokemon_emerald.GBA");

//     let v = match result {

//         Ok(result) => {result},
//         Err(error) => print!("Something went wrong: {}", error),
//     };

//     v
// }

fn main() -> Result<(), ()>{
    
    let bios = load_bios();

    for i in bios.into_iter() {
        print!("{:#01x}\t", i);
    }
    Ok(())
}
