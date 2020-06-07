use crate::game::Game;

pub fn run() {
    let mut game = Game::with_defaults();
    game.run();
}
