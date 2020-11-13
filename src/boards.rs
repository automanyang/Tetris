use crate::blocks::Block;
use fltk::{draw, widget::*, WidgetBase, WidgetExt};
use std::ops::{Deref, DerefMut};

// --

pub(crate) trait UnsafeFrom {
    fn unsafe_from<T>(p: *const T) -> &'static T {
        unsafe { &*p }
    }
    fn unsafe_mut_from<T>(p: *mut T) -> &'static mut T {
        unsafe { &mut *p }
    }
}

// --

pub(crate) struct TextBoard;
impl TextBoard {
    pub(crate) fn new(x: i32, y: i32, w: i32, h: i32, text: String) -> Self {
        let mut wid = Widget::new(x, y, w, h, "");
        wid.draw2(move |w| {
            draw::draw_box(
                FrameType::FlatBox,
                w.x(),
                w.y(),
                w.width(),
                w.height(),
                Color::FrameDefault,
            );
            draw::set_draw_color(Color::ForeGround);
            draw::set_font(draw::font(), 16);
            draw::draw_text2(
                &text,
                w.x(),
                w.y(),
                w.width(),
                w.height(),
                Align::Left | Align::Top,
            );
        });
        Self
    }
}

// --

pub(crate) struct ValueBoard {
    wid: Widget,
    value: i32,
}
impl ValueBoard {
    pub(crate) fn new_box(x: i32, y: i32, w: i32, h: i32, label: &str) -> Box<Self> {
        let mut wid = Widget::new(x, y, w, h, label).with_align(Align::Top | Align::Left);
        wid.set_label_size(24);

        let mut ptr = Box::new(Self { wid, value: 0 });
        let vb = ptr.as_mut() as *mut Self;
        ptr.wid.draw(move || {
            let vb = Self::unsafe_mut_from(vb);
            vb.draw();
        });
        ptr
    }
    fn draw(&mut self) {
        draw::draw_box(
            FrameType::FlatBox,
            self.wid.x(),
            self.wid.y(),
            self.wid.width(),
            self.wid.height(),
            Color::FrameDefault,
        );
        draw::set_draw_color(Color::ForeGround);
        draw::set_font(draw::font(), 36);
        let str = format!("{}", self.value);
        draw::draw_text2(
            &str,
            self.wid.x(),
            self.wid.y(),
            self.wid.width(),
            self.wid.height(),
            Align::Center,
        );
    }
    pub(crate) fn set_value(&mut self, v: i32) {
        self.value = v;
        self.wid.redraw();
    }
    pub(crate) fn value(&self) -> i32 {
        self.value
    }
}
impl Deref for ValueBoard {
    type Target = Widget;

    fn deref(&self) -> &Self::Target {
        &self.wid
    }
}
impl DerefMut for ValueBoard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wid
    }
}
impl UnsafeFrom for ValueBoard {}

// --

pub(crate) struct BlockBoard {
    wid: Widget,
    block: Option<Block>,
}

impl BlockBoard {
    pub(crate) fn new_box(x: i32, y: i32, w: i32, h: i32, label: &str) -> Box<Self> {
        let mut wid = Widget::new(x, y, w, h, label);
        wid.set_label_size(24);
        wid.set_align(Align::Top | Align::Left);
        let block = Some(Block::new());
        let mut ptr = Box::new(Self { wid, block });

        let bb = ptr.as_mut() as *mut Self;
        ptr.wid.draw(move || {
            let bb = Self::unsafe_mut_from(bb);
            bb.draw();
        });
        ptr.wid.redraw();
        
        ptr
    }
    fn draw(&mut self) {
        let x = self.x() + (self.width() - Block::size()) / 4;
        let y = self.y() + (self.height() - Block::size()) / 2;
        draw::draw_box(
            FrameType::FlatBox,
            self.x(),
            self.y(),
            self.width(),
            self.height(),
            Color::FrameDefault,
        );
        self.block.as_ref().and_then(|b| Some(b.draw(x, y, None)));
    }
    pub(crate) fn next(&mut self) -> Block {
        let block = self.block.replace(Block::new()).unwrap();
        self.wid.redraw();
        block
    }
}
impl Deref for BlockBoard {
    type Target = Widget;

    fn deref(&self) -> &Self::Target {
        &self.wid
    }
}
impl DerefMut for BlockBoard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wid
    }
}
impl UnsafeFrom for BlockBoard {}
