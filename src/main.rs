mod blocks;
mod boards;
mod stage;
mod tetris;

// --

fn main() {
    let app = fltk::app::App::default();
    let _tetris = tetris::TetrisWindow::init();
    app.run().unwrap();
}
