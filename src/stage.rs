use crate::blocks::Block;
use crate::boards::UnsafeFrom;
use crate::tetris::TetrisWindow;
use fltk::{draw, widget::*, Event, WidgetBase, WidgetExt};
use std::ops::{Deref, DerefMut};

// --

/*

       first column is 0
           |           last colume is 15
           |                 |
           V                 v
    20  0b_1110_0000_0000_0111      <-- row number: 20
    19  0b_1110_0000_0000_0111
    18  0b_1110_0000_0000_0111
    17  0b_1110_0000_0000_0111
    16  0b_1110_0000_0000_0111
    15  0b_1110_0000_0000_0111
    14  0b_1110_0000_0000_0111
    13  0b_1110_0000_0000_0111
    12  0b_1110_0000_0000_0111
    11  0b_1110_0000_0000_0111          dropping block
    10  0b_1110_0010_0000_0111      <-- row: 10
    9   0b_1110_0011_0000_0111          col: 6
    8   0b_1110_0001_0000_0111
    7   0b_1110_0000_0000_0111
    6   0b_1110_0000_0000_0111
    5   0b_1110_0000_0000_0111
    4   0b_1110_0000_0000_0111
    3   0b_1110_0000_0000_0111
    2   0b_1110_0000_0000_0111
    1   0b_1110_0000_0000_0111
    0   0b_1111_1111_1111_1111      < --- row number: 0


*/

pub(crate) struct Stage {
    wid: Widget,
    rows: [u16; Stage::ROWS_COUNT as usize],
    dropping: Option<Block>,
    col: i32,
    row: i32,
    shadow_row: i32,
}

impl Stage {
    const COLS_COUNT: i32 = 16;
    const ROWS_COUNT: i32 = 21;
    const DEFAULT_ROW_DATA: u16 = 0b1110_0000_0000_0111;
    const GROUND_ROW_DATA: u16 = 0b1111_1111_1111_1111;
    const LEFT_EDGE_COL: i32 = 3;

    pub(crate) fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Box<Self> {
        let mut rows = [Self::DEFAULT_ROW_DATA; Self::ROWS_COUNT as usize];
        rows[0] = Self::GROUND_ROW_DATA;
        let wid = Widget::new(x, y, w, h, label);
        let mut ptr = Box::new(Self {
            wid,
            rows,
            dropping: None,
            col: 7,
            row: Self::ROWS_COUNT - 1,
            shadow_row: 0,
        });

        let bb = ptr.as_mut() as *mut Self;
        ptr.wid.handle(move |ev| {
            let bb = Self::unsafe_mut_from(bb);
            bb.handle(ev)
        });
        let bb = ptr.as_mut() as *mut Self;
        ptr.wid.draw(move || {
            let bb = Self::unsafe_mut_from(bb);
            bb.draw();
        });

        ptr.wid.redraw();
        ptr
    }
    fn draw(&self) {
        draw::draw_box(
            FrameType::FlatBox,
            self.wid.x(),
            self.wid.y(),
            self.wid.width(),
            self.wid.height(),
            Color::Black,
        );
        self.draw_background();
        if let Some(b) = self.dropping.as_ref() {
            let x = (self.col - Self::LEFT_EDGE_COL) * Block::cell_size();
            let y = (Self::ROWS_COUNT - 1 - self.row) * Block::cell_size();
            let y2 = (Self::ROWS_COUNT - 1 - self.shadow_row) * Block::cell_size();

            b.draw(x, y2, Some(Color::from_rgb(30, 30, 30)));
            b.draw(x, y, None);
        };
    }
    pub(crate) fn need_block(&self) -> bool {
        self.dropping.is_none()
    }
    pub(crate) fn next_block(&mut self, b: Block) {
        self.dropping = Some(b);
        self.row = Self::ROWS_COUNT - 1;
        self.col = 7;
        self.shadow_row = 0;
    }
    pub(crate) fn tick(&mut self) -> i32 {
        let mut removed = 0;
        if let Some(block) = self.dropping.as_ref() {
            if self.collide_at(self.col, self.row - 1, block.data()) {
                removed = self.freeze();
            } else {
                self.row -= 1;
            }
            self.wid.redraw();
        }
        if self.shadow_row == 0 {
            self.shadow();
        }
        if self.row == Self::ROWS_COUNT - 1 {
            let s = fltk::app::screen_size();
            match fltk::dialog::choice(
                (s.0 / 2.0) as i32 - 100,
                (s.1 / 2.0) as i32 - 100,
                "Boomed!!!\n\nDo you want to try again?",
                "&Yes",
                "&No",
                "",
            ) {
                0 => self.reset(),
                1 => std::process::exit(0),
                _ => {}
            }
        }
        removed
    }
    fn reset(&mut self) {
        self.rows
            .iter_mut()
            .skip(1)
            .for_each(|r| *r = Self::DEFAULT_ROW_DATA);
        self.wid.redraw();
        TetrisWindow::get_mut().clean();
    }
    fn shadow(&mut self) {
        if let Some(block) = self.dropping {
            self.shadow_row = self.row;
            while !self.collide_at(self.col, self.shadow_row - 1, block.data()) {
                self.shadow_row -= 1;
            }
        }
    }
    fn freeze(&mut self) -> i32 {
        let mut removed = 0;
        if let Some(b) = self.dropping.as_ref() {
            let mut d = b.data();
            for i in 0..Block::CELLS_COUNT {
                self.rows[std::cmp::max(self.row - i, 0) as usize] |= (d & 0xF000) >> self.col;
                d <<= Block::CELLS_COUNT;
            }

            // remove the lines which are full
            let v = self.rows.to_vec();
            let mut it = v
                .iter()
                .skip(1)
                .filter(|r| {
                    if **r == Self::GROUND_ROW_DATA {
                        removed += 1;
                    }
                    **r != Self::GROUND_ROW_DATA
                })
                .map(|r| *r);
            for r in self.rows.iter_mut().skip(1) {
                if *r == Self::DEFAULT_ROW_DATA {
                    break;
                }
                *r = it.next().or(Some(Self::DEFAULT_ROW_DATA)).unwrap();
            }
        }
        self.dropping.take();
        TetrisWindow::get_mut().recount();
        
        removed
    }
    fn collide_at(&self, col: i32, row: i32, data: u16) -> bool {
        let mut pos_data = 0_u16;
        for i in 0..Block::CELLS_COUNT {
            pos_data <<= Block::CELLS_COUNT;
            pos_data |= (self.rows[std::cmp::max(row - i, 0) as usize]
                >> (Self::COLS_COUNT - col - Block::CELLS_COUNT))
                & 0x000F;
        }
        pos_data & data > 0
    }
    fn on_keydown(&mut self, s: String) -> bool {
        let mut ret = true;
        if let Some(block) = self.dropping.as_ref() {
            match s.as_str() {
                "a" => {
                    if !self.collide_at(self.col - 1, self.row, block.data()) {
                        self.col -= 1;
                    }
                }
                "d" => {
                    if !self.collide_at(self.col + 1, self.row, block.data()) {
                        self.col += 1;
                    }
                }
                "q" => {
                    let mut b2 = *block;
                    b2.posture.anticlockwise();
                    if !self.collide_at(self.col, self.row, b2.data()) {
                        self.dropping
                            .as_mut()
                            .and_then(|b| Some(b.posture.anticlockwise()));
                    }
                }
                "e" => {
                    let mut b2 = *block;
                    b2.posture.clockwise();
                    if !self.collide_at(self.col, self.row, b2.data()) {
                        self.dropping
                            .as_mut()
                            .and_then(|b| Some(b.posture.clockwise()));
                    }
                }
                "s" => {
                    if !self.collide_at(self.col, self.row - 1, block.data()) {
                        self.row -= 1;
                    }
                }
                " " => {
                    self.row = self.shadow_row; // fall down
                    TetrisWindow::get_mut().recount(); // reset the timeout count
                }
                "\u{1b}" => ret = false, // ESC received
                _ => {}
            }
        }

        if ret {
            self.shadow();
            self.wid.redraw();
        }
        ret
    }
    fn handle(&mut self, ev: Event) -> bool {
        match ev {
            Event::Focus | Event::Unfocus => true,
            Event::KeyDown => self.on_keydown(fltk::app::event_text()),
            _ => false,
        }
    }
    fn draw_background(&self) {
        // we draw these cells from bottom to top.
        let mut cell_x = self.wid.x() + Block::CELL_EDGE;
        let mut cell_y =
            self.wid.y() + Block::CELL_EDGE + Block::cell_size() * (Self::ROWS_COUNT - 2);

        draw::set_draw_color(Color::Dark3);
        for row in self.rows.iter().skip(1) {
            for i in 3..13 {
                let mask = 0b1000_0000_0000_0000 >> i;
                if row & mask > 0 {
                    draw::draw_rectf(cell_x, cell_y, Block::CELL_INNER, Block::CELL_INNER);
                }
                cell_x += Block::cell_size();
            }
            cell_x = self.wid.x() + Block::CELL_EDGE;
            cell_y -= Block::cell_size();
        }
    }
}
impl Deref for Stage {
    type Target = Widget;

    fn deref(&self) -> &Self::Target {
        &self.wid
    }
}
impl DerefMut for Stage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wid
    }
}
impl UnsafeFrom for Stage {}
