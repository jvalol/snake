use blitzkit::start;

mod coords;
mod input;
mod pellet;
mod snake;
mod snake_game;
mod state;
mod system;
mod util;

use snake_game::SnakeGame;

/// Whether this run is only here to be photographed, for `refresh-screenshots`
/// in the project above.
pub fn staged() -> bool {
    std::env::args().any(|arg| arg == "--screenshot")
}

fn main() {
    let snake_game = SnakeGame::new();
    start("Snake", Box::new(snake_game));
}
