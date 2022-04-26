
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use core_arm::*;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![read_rom])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
fn read_rom() -> Vec<u32> {
  let rom = "../../src/pokemon_emerald.GBA";
  let mut ram = RAM::new();
  let buff = ram.load_rom(&rom);

  let buff =  match buff {
    Ok(res) => res,
    Err(error) => panic!("Could not open : {:?}", error),
  };

 (&mut ram).load_rom_to_internal(buff);

  ram.game_rom
}