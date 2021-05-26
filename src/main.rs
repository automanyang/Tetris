// -- main.rs --

#![windows_subsystem = "windows"]

// --

mod blocks;
mod boards;
mod stage;
mod tetris;

// --

fn main() {
    let app = fltk::app::App::default();
    let _tetris = tetris::TetrisWindow::new_box();
    app.run().unwrap();
}
