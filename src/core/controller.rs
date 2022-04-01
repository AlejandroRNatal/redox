

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Column {
    Zero,
    One,
}

pub mod Controller{
    const REG_KEY: u16 = 0x4000130;//read only register for state of inputs
    const KEY_CONTROL_REGISTER: u32 =0x4000132;//write register for GBA inputs
}


pub struct Joypad{
    pub start: bool,
    pub select: bool,
    pub a: bool,
    pub b: bool,
    pub down: bool,
    pub up: bool,
    pub left: bool,
    pub right: bool,
    pub column: Column,
}

impl Joypad{
    pub fn new() -> Self {
        Joypad {
            column: Column::Zero,
            start: false,
            select: false,
            b: false,
            a: false,
            down: false,
            up: false,
            left: false,
            right: false,
        }
    }

    // pub fn to_byte(&self) -> u32 {
    //     let column_bit = if self.column == Column::Zero {
    //         1 << 5
    //     } else {
    //         1 << 4
    //     };
    //     let bit_4 =
    //         bit(!((self.down && self.reading_column_0())
    //             || (self.start && self.reading_column_1())))
    //             << 3;
    //     let bit_3 = bit(
    //         !((self.up && self.reading_column_0()) || (self.select && self.reading_column_1()))
    //     ) << 2;
    //     let bit_2 =
    //         bit(!((self.left && self.reading_column_0()) || (self.b && self.reading_column_1())))
    //             << 1;
    //     let bit_1 =
    //         bit(!((self.right && self.reading_column_0()) || (self.a && self.reading_column_1())));

    //     let row_bits = bit_4 | bit_3 | bit_2 | bit_1;
    //     column_bit | row_bits
    // }

    fn reading_column_0(&self) -> bool {
        self.column == Column::Zero
    }

    fn reading_column_1(&self) -> bool {
        self.column == Column::One
    }
}