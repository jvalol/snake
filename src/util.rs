#![macro_use]

/// Playfield height in cells. The width takes as many cells as fit.
pub const GRID_ROWS: i32 = 32;

/// Starting speed in cells per second.
pub const STARTING_SNAKE_SPEED: f32 = 8.0;
/// Added to the snake's speed for every pellet eaten.
pub const SNAKE_SPEED_INC: f32 = 0.5;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
  None,
  Up,
  Down,
  Left,
  Right,
}

#[macro_export]
macro_rules! any {
    ($x:expr, $($y:expr),+ $(,)?) => {
        {
            false $(|| $x == $y)+
        }
    };
}
