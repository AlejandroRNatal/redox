
pub struct Timer {
    pub frequency: usize,
    cycles: usize,
    pub value: u32,
    pub modulo: u32,
    pub on: bool,
}

impl Timer {
    pub fn new(frequency: usize) -> Self {
        Timer {
            frequency,
            cycles: 0,
            value: 0,
            modulo: 0,
            on: false,
        }
    }

    pub fn step(&mut self, cycles: u32) -> bool {
        if ! self.on {
            return false;
        }
    
        self.cycles += cycles as usize;

        let cycles_per_tick : f32 = 16.78 * 1_000_000;

        let has_overflowed = if self.cycles > cycles_per_tick {
            self.cycles = self.cycles % cycles_per_tick;
            let (new, has_overflowed) = self.value.overflowing_add(1);
            self.value = new;
            has_overflowed
        } else{
            false
        };

        if has_overflowed {
            self.value = self.modulo;
        }
        has_overflowed
    }
}