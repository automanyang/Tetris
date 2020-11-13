mod blocks;
mod boards;
mod stage;
mod tetris;

// --

fn main() {
    let app = fltk::app::App::default();
    let _tetris = tetris::TetrisWindow::init();
    app.run().unwrap();
    // while app.wait() {
    //     let k = fltk::app::event_key();
    //     dbg!(&k);
    //     let s = fltk::app::event_text();
    //     dbg!(&s);
    // }
}
