use crate::blocks::Block;
use crate::boards::{BlockBoard, TextBoard, ValueBoard};
use crate::stage::Stage;
use fltk::{group::Pack, window::DoubleWindow, GroupExt, WidgetBase, WidgetExt, WindowExt};

// --

fn tick() {
    let t = TetrisWindow::get_mut();
    t.tick();
    fltk::app::repeat_timeout(t.interval, tick);
}

// --

static mut TETRIS_PTR: *mut TetrisWindow = std::ptr::null_mut();
pub(crate) struct TetrisWindow {
    pub(crate) stage: Box<Stage>,
    pub(crate) generator: Box<BlockBoard>,
    pub(crate) lines: Box<ValueBoard>,
    pub(crate) level: Box<ValueBoard>,
    pub(crate) score: Box<ValueBoard>,

    pub(crate) interval: f64,
    pub(crate) count: usize,
}

impl TetrisWindow {
    const DEFAULT_COUNT: usize = 10;
    const DEFAULT_INTERVAL: f64 = 0.1;
    fn new() -> Self {
        let mut wind = DoubleWindow::default()
            .with_label("Tetris")
            .with_size(460, 600)
            .center_screen();
        let stage = Stage::new(0, 0, 300, 600, "");
        let mut pack = Pack::new(311, 40, 150, 600, "");
        let generator = BlockBoard::new_box(1, 1, 1, Block::size(), "Next:");
        let lines = ValueBoard::new_box(1, 1, 1, 50, "Lines:");
        let mut level = ValueBoard::new_box(1, 1, 1, 50, "Level:");
        let score = ValueBoard::new_box(1, 1, 1, 50, "Score:");
        TextBoard::new(
            1,
            1,
            1,
            150,
            String::from(
                r#"q: anticlockwise
e: clockwise
a: left
d: right
s: down
space: drop"#,
            ),
        );
        pack.end();
        wind.end();
        wind.show();

        pack.set_spacing(40);
        level.set_value(1);

        Self {
            stage,
            generator,
            lines,
            level,
            score,
            interval: Self::DEFAULT_INTERVAL,
            count: 0,
        }
    }
    pub(crate) fn new_box() -> Box<Self> {
        unsafe {
            assert!(TETRIS_PTR.is_null());
        }

        let mut tetris = Box::new(Self::new());
        fltk::app::add_timeout(tetris.interval, tick);

        unsafe {
            TETRIS_PTR = tetris.as_mut();
        }
        tetris
    }
    pub(crate) fn get_mut() -> &'static mut Self {
        unsafe {
            assert!(!TETRIS_PTR.is_null());
            let ptr = TETRIS_PTR;
            &mut *ptr as &mut Self
        }
    }
    pub(crate) fn recount(&mut self) {
        // after falling down, there should be about 0.2 ~ 0.4 second to move the block left or right.
        let percent: f64 = 0.7 * (Self::DEFAULT_COUNT as f64);
        self.count = (self.interval / Self::DEFAULT_INTERVAL * percent) as usize;
    }
    pub(crate) fn clean(&mut self) {
        self.lines.set_value(0);
        self.level.set_value(1);
        self.score.set_value(0);
        self.interval = Self::DEFAULT_INTERVAL;
    }
    fn tick(&mut self) {
        if self.count < Self::DEFAULT_COUNT {
            self.count += 1;
            return;
        }
        self.count = 0;

        if self.stage.need_block() {
            self.stage.next_block(self.generator.next())
        }

        let removed = self.stage.tick();
        self.lines.set_value(self.lines.value() + removed);

        const LINES_PER_LEVEL: i32 = 10;
        let level = self.level.value();
        if level * LINES_PER_LEVEL < self.lines.value() {
            self.level.set_value(level + 1);
            self.interval *= 0.9;
        }

        self.score.set_value(self.score.value() + removed * removed);
    }
}
