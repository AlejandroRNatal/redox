
///This Reg controls the screen
const REG_DISPCNT: u32 = 0x4000_0000;

///
const REG_DISPSTAT: u32 = 0x4000_0004;
const REG_VCOUNT: u32 = 0x4000_0006;

enum Color {

}

enum BitMapMode {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
}

enum CharBlockType {
    Sprite,
    Background,
}

struct Pixel<T> {
    data: T,
    x: usize,
    y: usize,
}

struct Tile {
    // 8* 8 pixelmaps
    data: [64; Pixel],
}

/// This is used to tell the screen where each tiles goes
struct TileMap {
    indices: [600; usize],//30*20 Tile-Map
}

struct Sprite {
    data: [64; u16],
}

struct CharBlock {
    char_type: CharBlockType,
}

struct PPU {
    vram: Vec::<u32>,
    pal_ram: Vec::<u32>,
    oam: Vec::<u32>,
}