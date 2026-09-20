use crate::coords::Grid;
use crate::pellet::Pellet;
use crate::snake::Snake;
use crate::util;
use cgmath::Vector2;
use blitkit::geometry::quad::Quad;
use blitkit::geometry::Geometry;
use blitkit::renderer::render_text::{RenderText, TextRenderer, UNBOUNDED_F32};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameState {
  MainMenu,
  Playing,
  Paused,
  GameOver,
  Quitting,
}

pub struct SnakeText {
  pub render_text: RenderText,
  pub visible: bool,
}

impl SnakeText {
  pub fn focused(&self) -> bool {
    self.render_text.focused
  }

  pub fn set_focus(&mut self, focused: bool) {
    self.render_text.focused = focused;
  }
}

pub struct State {
  pub game_state: GameState,
  pub walls: Vec<Quad>,
  pub snake: Snake,
  pub pellet: Pellet,
  pub title_text: SnakeText,
  pub play_button: SnakeText,
  pub quit_button: SnakeText,
  pub score: SnakeText,
  pub win_text: SnakeText,
  /// The playfield, rebuilt whenever the window changes size.
  pub grid: Grid,
  /// Seconds since the previous update.
  pub delta_time: f32,
}

impl State {
  pub fn new() -> Self {
    Self {
      game_state: GameState::MainMenu,
      // all built by layout()
      walls: Vec::new(),
      snake: Snake::new(),
      pellet: Pellet::new(),
      title_text: SnakeText {
        visible: false,
        render_text: RenderText {
          position: (20.0, 20.0).into(),
          color: (1.0, 1.0, 1.0, 1.0).into(),
          text: String::from("SNAKE"),
          size: 64.0,
          ..Default::default()
        },
      },
      play_button: SnakeText {
        visible: false,
        render_text: RenderText {
          position: (40.0, 100.0).into(),
          color: (1.0, 1.0, 1.0, 1.0).into(),
          text: String::from("Play"),
          size: 32.0,
          ..Default::default()
        },
      },
      quit_button: SnakeText {
        visible: false,
        render_text: RenderText {
          position: (40.0, 160.0).into(),
          color: (1.0, 1.0, 1.0, 1.0).into(),
          text: String::from("Quit"),
          size: 32.0,
          ..Default::default()
        },
      },
      score: SnakeText {
        visible: false,
        render_text: RenderText {
          // position: (render.width() * 0.75, 20.0).into(),
          position: (120.0, 20.0).into(),
          color: (1.0, 1.0, 1.0, 1.0).into(),
          text: String::from("0"),
          size: 32.0,
          ..Default::default()
        },
      },
      win_text: SnakeText {
        visible: false,
        render_text: RenderText {
          // centered in the window by layout()
          position: (0.0, 0.0).into(),
          bounds: (UNBOUNDED_F32, UNBOUNDED_F32).into(),
          size: 32.0,
          centered: true,
          ..Default::default()
        },
      },
      grid: Grid::new((0.0, 0.0).into(), util::GRID_ROWS),
      delta_time: 0.0,
    }
  }

  /// Rebuilds the playfield for a window of `size` pixels. Mid-game the snake and
  /// pellet keep their cells, so play continues where it was.
  pub fn layout(&mut self, size: Vector2<f32>) {
    let old = self.grid;
    self.grid = Grid::new(size, util::GRID_ROWS);
    let grid = self.grid;
    let field = grid.size();
    let thickness = (grid.cell * 0.2).max(2.0);

    self.walls = vec![
      Quad::new(
        (grid.origin.x + field.x * 0.5, grid.origin.y).into(),
        (field.x + thickness, thickness).into(),
      ),
      Quad::new(
        (grid.origin.x + field.x * 0.5, grid.origin.y + field.y).into(),
        (field.x + thickness, thickness).into(),
      ),
      Quad::new(
        (grid.origin.x, grid.origin.y + field.y * 0.5).into(),
        (thickness, field.y + thickness).into(),
      ),
      Quad::new(
        (grid.origin.x + field.x, grid.origin.y + field.y * 0.5).into(),
        (thickness, field.y + thickness).into(),
      ),
    ];

    if self.snake.body.is_empty() {
      self.snake.reset(&grid);
      self.pellet.place(&grid, (grid.cols / 4, grid.rows / 2));
    } else {
      let pellet_cell = self.pellet.cell(&old);
      self.snake.regrid(&old, &grid);
      self.pellet.place(
        &grid,
        (
          pellet_cell.0.max(0).min(grid.cols - 1),
          pellet_cell.1.max(0).min(grid.rows - 1),
        ),
      );
    }

    self.win_text.render_text.position = size * 0.5;
  }

  pub fn initialize(&mut self, geometry: &mut Geometry, text_renderer: &mut TextRenderer) {
    self.update_geometry(geometry);
    self.update_text(text_renderer);
  }

  pub fn update(&self, geometry: &mut Geometry, text_renderer: &mut TextRenderer) {
    self.update_geometry(geometry);
    self.update_text(text_renderer);
  }

  fn update_geometry(&self, geometry: &mut Geometry) {
    if self.snake.visible {
      for quad in self.walls.iter() {
        geometry.push_quad(quad);
      }

      for quad in self.snake.body.iter() {
        geometry.push_quad(quad);
      }
    }

    if self.pellet.visible {
      geometry.push_quad(&self.pellet.quad);
    }
  }

  fn update_text(&self, text_renderer: &mut TextRenderer) {
    for text in [
      &self.title_text,
      &self.play_button,
      &self.quit_button,
      &self.score,
      &self.win_text,
    ]
    .iter()
    {
      if text.visible {
        text_renderer.push_render_text(text.render_text.clone());
      }
    }
  }

  pub fn pause_game(&mut self) {
    if self.game_state == GameState::Playing {
      self.game_state = GameState::Paused;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn state_of(width: f32, height: f32) -> State {
    let mut state = State::new();
    state.layout((width, height).into());
    state
  }

  #[test]
  fn layout_makes_four_walls() {
    let state = state_of(800.0, 600.0);

    assert_eq!(state.walls.len(), 4);
  }

  #[test]
  fn resize_keeps_cells() {
    let mut state = state_of(800.0, 600.0);
    let snake_cell = state.snake.head_cell(&state.grid);
    let pellet_cell = state.pellet.cell(&state.grid);

    state.layout((1600.0, 1200.0).into());

    assert_eq!(state.snake.head_cell(&state.grid), snake_cell);
    assert_eq!(state.pellet.cell(&state.grid), pellet_cell);
  }
}
