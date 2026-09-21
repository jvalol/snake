use crate::coords::Grid;
use crate::util;
use crate::util::Direction;
use crate::util::Direction::*;
use glam::Vec2;
use blitkit::geometry::quad::Quad;

pub struct Snake {
  /// Head first. One quad per cell the snake fills.
  pub body: Vec<Quad>,
  /// The head's position in pixels, which moves smoothly between cells.
  pub position: Vec2,
  pub direction: Direction,
  /// Cells per second.
  pub speed: f32,
  pub score: u32,
  pub visible: bool,
}

impl Snake {
  pub fn new() -> Snake {
    Snake {
      body: Vec::new(),
      position: (0.0, 0.0).into(),
      direction: None,
      speed: util::STARTING_SNAKE_SPEED,
      score: 0,
      visible: false,
    }
  }

  /// Puts a one segment snake back in the middle of the playfield, stopped.
  pub fn reset(&mut self, grid: &Grid) {
    self.score = 0;
    self.speed = util::STARTING_SNAKE_SPEED;
    self.direction = None;
    self.position = grid.center_of((grid.cols / 2, grid.rows / 2));
    self.body = vec![Quad::new(self.position, (grid.cell, grid.cell).into())];
  }

  pub fn update_direction(&mut self, direction: Direction) {
    self.direction = direction;
  }

  pub fn direction(&self) -> Vec2 {
    match self.direction {
      None => (0.0, 0.0).into(),
      Up => (0.0, -1.0).into(),
      Down => (0.0, 1.0).into(),
      Left => (-1.0, 0.0).into(),
      Right => (1.0, 0.0).into(),
    }
  }

  pub fn head_cell(&self, grid: &Grid) -> (i32, i32) {
    grid.cell_at(self.position)
  }

  /// Moves the head by one frame's worth of travel. The body only follows when
  /// the head crosses into a new cell.
  pub fn advance(&mut self, grid: &Grid, delta_time: f32) {
    let step = self.direction() * self.speed * grid.cell * delta_time;
    self.update_position(grid, self.position + step);
  }

  pub fn update_position(&mut self, grid: &Grid, position: Vec2) {
    let old_cell = grid.cell_at(self.head().position);
    let new_cell = grid.cell_at(position);
    self.position = position;

    if new_cell != old_cell {
      let head = Quad::new(grid.center_of(new_cell), self.head().size);
      let mut body = vec![head];
      body.extend_from_slice(&self.body[..self.body.len() - 1]);
      self.body = body;
    }
  }

  /// Adds a segment behind the tail, in the cell the snake came from.
  pub fn grow_body(&mut self, grid: &Grid) {
    let tail = self.body[self.body.len() - 1];
    let tail_cell = grid.cell_at(tail.position);
    let direction = self.direction();
    let cell = (
      tail_cell.0 - direction.x as i32,
      tail_cell.1 - direction.y as i32,
    );

    self.body.push(Quad::new(grid.center_of(cell), tail.size));
  }

  /// True when the head shares a cell with any segment behind it.
  pub fn bites_itself(&self, grid: &Grid) -> bool {
    let head_cell = self.head_cell(grid);

    self.body[1..]
      .iter()
      .any(|segment| grid.cell_at(segment.position) == head_cell)
  }

  /// Moves every segment to the same cell in a resized grid.
  pub fn regrid(&mut self, old: &Grid, grid: &Grid) {
    for segment in self.body.iter_mut() {
      let cell = old.cell_at(segment.position);
      let cell = (
        cell.0.max(0).min(grid.cols - 1),
        cell.1.max(0).min(grid.rows - 1),
      );
      *segment = Quad::new(grid.center_of(cell), (grid.cell, grid.cell).into());
    }
    self.position = self.head().position;
  }

  fn head(&self) -> Quad {
    self.body[0]
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::coords::Grid;

  fn grid() -> Grid {
    Grid::new((800.0, 600.0).into(), 32)
  }

  fn snake(grid: &Grid) -> Snake {
    let mut snake = Snake::new();
    snake.reset(grid);
    snake
  }

  #[test]
  fn starts_in_the_middle() {
    let grid = grid();
    let snake = snake(&grid);

    assert_eq!(snake.body.len(), 1);
    assert_eq!(snake.direction, Direction::None);
    assert_eq!(snake.head_cell(&grid), (grid.cols / 2, grid.rows / 2));
    assert_eq!(snake.speed, util::STARTING_SNAKE_SPEED);
  }

  #[test]
  fn does_not_move_without_a_direction() {
    let grid = grid();
    let mut snake = snake(&grid);
    let start = snake.position;
    snake.advance(&grid, 1.0);

    assert_eq!(snake.position, start);
  }

  #[test]
  fn advances_at_its_speed() {
    let grid = grid();
    let mut snake = snake(&grid);
    let start = snake.position;
    snake.update_direction(Direction::Right);
    snake.speed = 4.0;
    snake.advance(&grid, 0.5);

    // four cells a second for half a second is two cells
    assert_eq!(snake.position.x - start.x, grid.cell * 2.0);
  }

  #[test]
  fn body_follows_on_cell_change() {
    let grid = grid();
    let mut snake = snake(&grid);
    let start_cell = snake.head_cell(&grid);
    snake.update_direction(Direction::Right);

    // a third of a cell is not enough to move the body
    snake.advance(&grid, 1.0 / snake.speed / 3.0);
    assert_eq!(grid.cell_at(snake.body[0].position), start_cell);

    // crossing into the next cell moves it
    snake.advance(&grid, 1.0 / snake.speed);
    assert_eq!(grid.cell_at(snake.body[0].position), (start_cell.0 + 1, start_cell.1));
    assert_eq!(snake.body.len(), 1);
  }

  #[test]
  fn growing_adds_a_segment_behind() {
    let grid = grid();
    let mut snake = snake(&grid);
    snake.update_direction(Direction::Right);
    let tail_cell = grid.cell_at(snake.body[0].position);
    snake.grow_body(&grid);

    assert_eq!(snake.body.len(), 2);
    assert_eq!(grid.cell_at(snake.body[1].position), (tail_cell.0 - 1, tail_cell.1));
  }

  #[test]
  fn bites_itself_when_the_head_meets_the_body() {
    let grid = grid();
    let mut snake = snake(&grid);
    snake.update_direction(Direction::Right);
    for _ in 0..3 {
      snake.grow_body(&grid);
    }
    assert!(!snake.bites_itself(&grid));

    // the segment right behind the head moves out of the way as the snake moves,
    // so a bite means reaching a segment further back
    let further_back = grid.cell_at(snake.body[2].position);
    snake.update_position(&grid, grid.center_of(further_back));

    assert!(snake.bites_itself(&grid));
  }

  #[test]
  fn regrid_keeps_cells() {
    let grid = grid();
    let mut snake = snake(&grid);
    snake.update_direction(Direction::Right);
    snake.grow_body(&grid);
    let cells: Vec<(i32, i32)> = snake
      .body
      .iter()
      .map(|segment| grid.cell_at(segment.position))
      .collect();

    let bigger = Grid::new((1600.0, 1200.0).into(), 32);
    snake.regrid(&grid, &bigger);

    let moved: Vec<(i32, i32)> = snake
      .body
      .iter()
      .map(|segment| bigger.cell_at(segment.position))
      .collect();

    assert_eq!(moved, cells);
    assert_eq!(snake.body[0].size.x, bigger.cell);
  }
}
