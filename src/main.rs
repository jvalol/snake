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

fn main() {
  let snake_game = SnakeGame::new();
  start("Snake", Box::new(snake_game));
}
