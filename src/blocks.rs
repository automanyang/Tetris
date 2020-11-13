// use crate::tetris::TetrisWindow;
use fltk::{draw, Color};

// --
#[derive(Copy, Clone)]
pub(crate) struct Posture(u8);

impl Posture {
    pub(crate) const COUNT: u8 = 4;
    pub(crate) fn new() -> Self {
        Self(rand::random::<u8>() % Self::COUNT)
    }
    pub(crate) fn clockwise(&mut self) {
        if self.0 == Self::COUNT - 1 {
            self.0 = 0;
        } else {
            self.0 += 1;
        }
    }
    pub(crate) fn anticlockwise(&mut self) {
        if self.0 == 0 {
            self.0 = Self::COUNT - 1;
        } else {
            self.0 -= 1;
        }
    }
    pub(crate) fn index(&self) -> usize {
        self.0 as usize
    }
}

// --
#[derive(Clone, Copy)]
pub(crate) struct Block {
    index: usize,
    pub(crate) posture: Posture,
}

impl Block {
    const BLOCKS_COUNT: usize = 7;
    const COLORS: [Color; Self::BLOCKS_COUNT] = [
        Color::Red,         // I
        Color::DarkBlue,    // O
        Color::DarkCyan,       // Z
        Color::DarkMagenta, // S
        Color::Blue,        // L
        Color::DarkGreen,   // J
        Color::DarkRed,     // T
    ];
    const DATA: [[u16; 4]; Self::BLOCKS_COUNT] = [
        // I block
        [
            0b0100_0100_0100_0100,
            0b0000_1111_0000_0000,
            0b0100_0100_0100_0100,
            0b0000_1111_0000_0000,
        ],
        // O block
        [
            0b0000_1100_1100_0000,
            0b0000_1100_1100_0000,
            0b0000_1100_1100_0000,
            0b0000_1100_1100_0000,
        ],
        // Z block
        [
            0b0000_1100_0110_0000,
            0b0010_0110_0100_0000,
            0b0000_1100_0110_0000,
            0b0010_0110_0100_0000,
        ],
        // S block
        [
            0b0000_0110_1100_0000,
            0b1000_1100_0100_0000,
            0b0000_0110_1100_0000,
            0b1000_1100_0100_0000,
        ],
        // L block
        [
            0b0100_0100_0110_0000,
            0b0000_1110_1000_0000,
            0b1100_0100_0100_0000,
            0b0010_1110_0000_0000,
        ],
        // J block
        [
            0b0100_0100_1100_0000,
            0b1000_1110_0000_0000,
            0b0110_0100_0100_0000,
            0b0000_1110_0010_0000,
        ],
        // T block
        [
            0b0000_1110_0100_0000,
            0b0100_1100_0100_0000,
            0b0100_1110_0000_0000,
            0b0100_0110_0100_0000,
        ],
    ];
    pub(crate) const CELLS_COUNT: i32 = 4;
    pub(crate) const CELL_INNER: i32 = 28;
    pub(crate) const CELL_EDGE: i32 = 1;

    pub(crate) const fn cell_size() -> i32 {
        Self::CELL_EDGE + Self::CELL_INNER + Self::CELL_EDGE
    }
    pub(crate) const fn size() -> i32 {
        Self::cell_size() * Self::CELLS_COUNT
    }

    pub(crate) fn new() -> Self {
        Self {
            index: rand::random::<usize>() % Self::BLOCKS_COUNT,
            // index: 2,
            posture: Posture::new(),
        }
    }
    pub(crate) fn color(&self) -> Color {
        Self::COLORS[self.index]
    }
    pub(crate) fn data(&self) -> u16 {
        Self::DATA[self.index][self.posture.index()]
    }
    // pub(crate) fn 
    pub(crate) fn draw(&self, x: i32, y: i32, c: Option<Color>) {
        let mut cell_x = x + Self::cell_size() * 3 + Self::CELL_EDGE;
        let mut cell_y = y + Self::cell_size() * 3 + Self::CELL_EDGE;
        let mut data = self.data();

        if let Some(color) = c {
            draw::set_draw_color(color);
        } else {
            draw::set_draw_color(self.color());
        }
        for _ in 0..Self::CELLS_COUNT {
            for _ in 0..Self::CELLS_COUNT {
                if data & 1 == 1 {
                    draw::draw_rectf(cell_x, cell_y, Self::CELL_INNER, Self::CELL_INNER);
                }
                data >>= 1;
                cell_x -= Self::cell_size();
            }
            cell_x = x + Self::cell_size() * 3 + Self::CELL_EDGE;
            cell_y -= Self::cell_size();
        }
    }
}

mod tests {
    #[test]
    fn v1() {
        for i in 11..10 {
            dbg!(i);
        }
    }
}