#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        for val in [&mut self.r, &mut self.g, &mut self.b, &mut self.a].iter_mut() {
            **val = swap_val(**val, first, second);
        }
        self
    }
}

fn swap_val(val: u8, first: u8, second: u8) -> u8 {
    if val == first {
        second
    } else if val == second {
        first
    } else {
        val
    }
}
