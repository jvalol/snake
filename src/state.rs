use crate::coords::Grid;
use crate::pellet::Pellet;
use crate::snake::Snake;
use crate::util;
use blitzkit::geometry::quad::Quad;
use blitzkit::geometry::Geometry;
use blitzkit::notice;
use blitzkit::renderer::render_text::{RenderText, TextRenderer, UNBOUNDED_F32};
use glam::Vec2;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameState {
    Playing,
    Paused,
    GameOver,
    Quitting,
}

pub struct SnakeText {
    pub render_text: RenderText,
    pub visible: bool,
}

pub struct State {
    pub game_state: GameState,
    pub walls: Vec<Quad>,
    pub snake: Snake,
    pub pellet: Pellet,
    pub pause_text: SnakeText,
    pub score: SnakeText,
    pub win_text: SnakeText,
    /// The playfield, rebuilt whenever the window changes size.
    pub grid: Grid,
    /// Seconds since the previous update.
    pub delta_time: f32,
}

/// Where the score sits and how tall it is, in cells.
///
/// **In cells, because everything else in this game is.** They were 32
/// points at (120, 20), which at the window this ships in happens to be
/// about one cell tall, four cells in and two thirds of a cell down. Fixed
/// points do not follow the window the way the playfield does, so at 320 by
/// 240 "Score: 0" ran wider than the window and the number was past the
/// right edge. Held by `the_score_fits_the_window`. Spec 0006.
const SCORE_TALL: f32 = 1.1;
const SCORE_IN: f32 = 4.0;
const SCORE_DOWN: f32 = 0.7;

impl State {
    pub fn new() -> Self {
        Self {
            game_state: GameState::Playing,
            // all built by layout()
            walls: Vec::new(),
            snake: Snake::new(),
            pellet: Pellet::new(),
            pause_text: SnakeText {
                visible: false,
                render_text: RenderText {
                    position: (20.0, 20.0).into(),
                    color: (1.0, 1.0, 1.0, 1.0).into(),
                    size: 24.0,
                    centered: true,
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
    pub fn layout(&mut self, size: Vec2) {
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
        self.pause_text.render_text.position = size * 0.5;

        self.score.render_text.size = grid.cell * SCORE_TALL;
        self.score.render_text.position = Vec2::new(
            grid.origin.x + grid.cell * SCORE_IN,
            grid.origin.y + grid.cell * SCORE_DOWN,
        );
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

        // last, so the panel covers the field rather than the other way round
        for text in [&self.pause_text, &self.win_text] {
            if text.visible {
                for quad in notice::framing(&text.render_text).iter() {
                    geometry.push_quad(quad);
                }
            }
        }
    }

    fn update_text(&self, text_renderer: &mut TextRenderer) {
        for text in [&self.pause_text, &self.score, &self.win_text].iter() {
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

    /// Spec 0006: the readout fits whatever window it is in.
    ///
    /// **Measured, not eyeballed.** `blitzkit::text::room_for` is what the
    /// renderer lays a string out with, so this asks the same question the
    /// drawing answers rather than a guess about how wide a glyph is.
    ///
    /// The engine's own picture size is in the list because that is where
    /// this was found: snake's own reference showed "Score:" filling the
    /// top and the number past the edge, which no window anybody plays in
    /// would have shown.
    #[test]
    fn the_score_fits_the_window() {
        // three digits, because 32 rows of pellets is more than two
        let longest = String::from("Score: 999");

        for (wide, tall) in [
            (
                blitzkit::picture::SIZE.0 as f32,
                blitzkit::picture::SIZE.1 as f32,
            ),
            (640.0, 480.0),
            (1200.0, 948.0),
        ] {
            let mut state = State::new();
            state.layout(Vec2::new(wide, tall));
            state.score.render_text.text = longest.clone();

            let room = blitzkit::text::room_for(&longest, state.score.render_text.size);
            let right = state.score.render_text.position.x + room.x;
            let bottom = state.score.render_text.position.y + room.y;

            assert!(
                right <= wide,
                "at {wide} by {tall} the score runs to {right}, which is {:.0} past the edge",
                right - wide
            );
            assert!(
                bottom <= tall,
                "at {wide} by {tall} the score runs to {bottom}, which is {:.0} below the window",
                bottom - tall
            );
        }
    }

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

    #[test]
    fn the_win_line_is_framed() {
        let mut state = State::new();
        state.layout(glam::vec2(800.0, 600.0));

        let mut bare = Geometry::new();
        state.win_text.visible = false;
        state.update_geometry(&mut bare);

        let mut framed = Geometry::new();
        state.win_text.visible = true;
        state.update_geometry(&mut framed);

        assert_eq!(framed.num_quads, bare.num_quads + 2);
    }
}
