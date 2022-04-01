
use std::fs::{File, self};
use std::env;
use std::io::{prelude::*, BufReader};
use std::path::Path;


fn load_bios() -> [u8; 16384] {
    let filename  = "src/gba_bios.bin";
    let mut file = File::open(filename);

    let mut buffer = [0;16384];
    let res = match &mut file {

        Ok(file) => file.read_exact(& mut buffer),
        Err(error) => panic!("Problem opening BIOS file: {:?}", error)

    };


    buffer
}

fn main() -> Result<(), ()>{
    
    let bios = load_bios();

    for i in bios.into_iter() {
        print!("{:#01x}\t", i);
    }
    Ok(())
}
