extern crate tetris;

fn main() {
    //tetris::sdl2_demo::demo();
    let game = tetris::game::Game {};
    game.run();
}
