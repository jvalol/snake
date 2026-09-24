use crate::any;
use crate::input::Input;
use crate::snake_game::*;
use crate::state::*;
use crate::util;
use crate::util::Direction::*;
use rand::Rng;

pub trait System {
    #[allow(unused_variables)]
    fn start(&mut self, game: &mut State) {}
    fn update_state(&self, input: &mut Input, state: &mut State, events: &mut Vec<Event>);
}

pub struct VisibilitySystem;
impl System for VisibilitySystem {
    fn update_state(&self, _input: &mut Input, state: &mut State, _events: &mut Vec<Event>) {
        let is_in_game = any!(state.game_state, GameState::Playing, GameState::GameOver);
        state.snake.visible = is_in_game;
        state.score.visible = is_in_game;
        state.pellet.visible = is_in_game;

        state.title_text.visible =
            state.game_state == GameState::MainMenu || state.game_state == GameState::Paused;
        state.play_button.visible =
            state.game_state == GameState::MainMenu || state.game_state == GameState::Paused;
        state.quit_button.visible = state.game_state == GameState::MainMenu;

        state.win_text.visible = state.game_state == GameState::GameOver;
    }
}

#[derive(Debug)]
pub struct MenuSystem;

impl System for MenuSystem {
    fn start(&mut self, state: &mut State) {
        state.title_text.render_text.text = String::from("SNAKE");
        state.play_button.render_text.text = String::from("Play");

        state.play_button.render_text.focused = true;
        state.quit_button.render_text.focused = false;
    }

    fn update_state(&self, input: &mut Input, state: &mut State, events: &mut Vec<Event>) {
        if input.esc_pressed {
            events.push(Event::ButtonPressed);
            state.game_state = GameState::Quitting;

            input.esc_pressed = false;
        }

        if state.play_button.focused() && input.ui_down_pressed() {
            events.push(Event::FocusChanged);
            state.play_button.set_focus(false);
            state.quit_button.set_focus(true);
        } else if state.quit_button.focused() && input.ui_up_pressed() {
            events.push(Event::FocusChanged);
            state.quit_button.set_focus(false);
            state.play_button.set_focus(true);
        }

        if state.play_button.focused() && input.enter_pressed {
            events.push(Event::ButtonPressed);
            state.game_state = GameState::Playing;
        } else if state.quit_button.focused() && input.enter_pressed {
            events.push(Event::ButtonPressed);
            state.game_state = GameState::Quitting;
        }
    }
}

#[derive(Debug)]
pub struct PlaySystem;

impl System for PlaySystem {
    fn start(&mut self, state: &mut State) {
        state.snake.reset(&state.grid);
        place_pellet(state);
    }

    fn update_state(&self, input: &mut Input, state: &mut State, events: &mut Vec<Event>) {
        if input.esc_pressed {
            input.clear();
            events.push(Event::ButtonPressed);
            state.game_state = GameState::MainMenu;

            input.esc_pressed = false;
        }

        state.score.render_text.text = format!("Score: {}", state.snake.score);

        if input.up_pressed && state.snake.direction != Down {
            state.snake.update_direction(Up);
        }
        if input.down_pressed && state.snake.direction != Up {
            state.snake.update_direction(Down);
        }
        if input.right_pressed && state.snake.direction != Left {
            state.snake.update_direction(Right);
        }
        if input.left_pressed && state.snake.direction != Right {
            state.snake.update_direction(Left);
        }

        let grid = state.grid;
        state.snake.advance(&grid, state.delta_time);

        // a crash is either leaving the playfield or running into the snake's own body
        if !grid.contains(state.snake.head_cell(&grid)) || state.snake.bites_itself(&grid) {
            events.push(Event::SnakeCrashed);
            state.game_state = GameState::GameOver;
            return;
        }

        if state.snake.head_cell(&grid) == state.pellet.cell(&grid) {
            state.snake.score += 1;
            events.push(Event::Score);

            state.snake.grow_body(&grid);
            state.snake.speed += util::SNAKE_SPEED_INC;
            place_pellet(state);
        }
    }
}

/// Puts the pellet on a random cell the snake is not already using.
fn place_pellet(state: &mut State) {
    let grid = state.grid;
    let mut rng = rand::thread_rng();
    let taken: Vec<(i32, i32)> = state
        .snake
        .body
        .iter()
        .map(|segment| grid.cell_at(segment.position))
        .collect();

    for _ in 0..100 {
        let cell = (rng.gen_range(0..grid.cols), rng.gen_range(0..grid.rows));
        if !taken.contains(&cell) {
            state.pellet.place(&grid, cell);
            return;
        }
    }

    // the playfield is essentially full, so anywhere will do
    state.pellet.place(&grid, (grid.cols / 2, grid.rows / 2));
}

#[derive(Debug)]
pub struct PauseSystem;

impl System for PauseSystem {
    fn start(&mut self, state: &mut State) {
        state.title_text.render_text.text = String::from("Paused");
        state.play_button.render_text.text = String::from("Resume");
        state.play_button.render_text.focused = true;
    }

    fn update_state(&self, input: &mut Input, state: &mut State, events: &mut Vec<Event>) {
        if state.play_button.focused() && input.enter_pressed {
            events.push(Event::ButtonPressed);
            state.game_state = GameState::Playing;
        }
    }
}

pub struct GameOverSystem {
    last_time: std::time::Instant,
}

impl GameOverSystem {
    pub fn new() -> Self {
        Self {
            last_time: std::time::Instant::now(),
        }
    }
}

impl System for GameOverSystem {
    fn start(&mut self, state: &mut State) {
        self.last_time = std::time::Instant::now();

        state.win_text.render_text.text = String::from("Game Over")
    }

    fn update_state(&self, input: &mut Input, state: &mut State, events: &mut Vec<Event>) {
        if input.esc_pressed {
            events.push(Event::ButtonPressed);
            state.game_state = GameState::Quitting;

            input.esc_pressed = false;
        }

        let current_time = std::time::Instant::now();
        let delta_time = current_time - self.last_time;
        if delta_time.as_secs_f32() > 5.0 {
            state.game_state = GameState::MainMenu;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Direction;

    fn playing_state() -> State {
        let mut state = State::new();
        state.layout((800.0, 600.0).into());
        state.game_state = GameState::Playing;
        state.delta_time = 1.0 / 60.0;
        state
    }

    fn play(input: &mut Input, state: &mut State) -> Vec<Event> {
        let mut events = Vec::new();
        PlaySystem.update_state(input, state, &mut events);
        events
    }

    /// Puts the pellet in the cell the snake is about to enter.
    fn pellet_ahead(state: &mut State) {
        let grid = state.grid;
        let cell = state.snake.head_cell(&grid);
        state.pellet.place(&grid, (cell.0 + 1, cell.1));
    }

    #[test]
    fn leaving_the_playfield_crashes() {
        let mut state = playing_state();
        let grid = state.grid;
        state.snake.update_direction(Direction::Left);
        state.snake.update_position(&grid, grid.center_of((0, 0)));
        state.delta_time = 1.0;

        play(&mut Input::new(), &mut state);

        assert_eq!(state.game_state, GameState::GameOver);
    }

    #[test]
    fn biting_itself_crashes() {
        let mut state = playing_state();
        let grid = state.grid;
        state.snake.update_direction(Direction::Right);
        // a body long enough to turn back into
        for _ in 0..4 {
            state.snake.grow_body(&grid);
        }
        let behind = grid.cell_at(state.snake.body[1].position);
        state.snake.update_position(&grid, grid.center_of(behind));

        play(&mut Input::new(), &mut state);

        assert_eq!(state.game_state, GameState::GameOver);
    }

    #[test]
    fn cannot_reverse_onto_itself() {
        let mut state = playing_state();
        state.snake.update_direction(Direction::Left);
        let mut input = Input::new();
        input.right_pressed = true;

        play(&mut input, &mut state);

        assert_eq!(state.snake.direction, Direction::Left);
    }

    #[test]
    fn turns_to_a_new_direction() {
        let mut state = playing_state();
        state.snake.update_direction(Direction::Left);
        let mut input = Input::new();
        input.up_pressed = true;

        play(&mut input, &mut state);

        assert_eq!(state.snake.direction, Direction::Up);
    }

    #[test]
    fn eating_scores_and_grows() {
        let mut state = playing_state();
        state.snake.update_direction(Direction::Right);
        pellet_ahead(&mut state);
        state.delta_time = 1.0 / state.snake.speed;

        play(&mut Input::new(), &mut state);

        assert_eq!(state.snake.score, 1);
        assert_eq!(state.snake.body.len(), 2);
        assert_eq!(state.game_state, GameState::Playing);
    }

    #[test]
    fn eating_speeds_the_snake_up() {
        let mut state = playing_state();
        state.snake.update_direction(Direction::Right);
        pellet_ahead(&mut state);
        let before = state.snake.speed;
        state.delta_time = 1.0 / before;

        play(&mut Input::new(), &mut state);

        assert_eq!(state.snake.speed, before + util::SNAKE_SPEED_INC);
    }

    #[test]
    fn new_pellet_avoids_the_snake() {
        let mut state = playing_state();
        let grid = state.grid;
        state.snake.update_direction(Direction::Right);
        for _ in 0..20 {
            state.snake.grow_body(&grid);
        }

        for _ in 0..50 {
            place_pellet(&mut state);
            let pellet = state.pellet.cell(&grid);
            assert!(state
                .snake
                .body
                .iter()
                .all(|segment| grid.cell_at(segment.position) != pellet));
        }
    }

    #[test]
    fn pellet_fills_one_cell() {
        let state = playing_state();

        assert_eq!(state.pellet.quad.size.x, state.grid.cell);
        assert_eq!(state.pellet.quad.size.y, state.grid.cell);
    }

    #[test]
    fn escape_returns_to_the_menu() {
        let mut state = playing_state();
        let mut input = Input::new();
        input.esc_pressed = true;

        play(&mut input, &mut state);

        assert_eq!(state.game_state, GameState::MainMenu);
    }

    #[test]
    fn escape_quits_from_the_menu() {
        let mut state = State::new();
        state.layout((800.0, 600.0).into());
        let mut input = Input::new();
        input.esc_pressed = true;

        MenuSystem.update_state(&mut input, &mut state, &mut Vec::new());

        assert_eq!(state.game_state, GameState::Quitting);
    }

    #[test]
    fn resuming_returns_to_playing() {
        let mut state = playing_state();
        PauseSystem.start(&mut state);
        state.game_state = GameState::Paused;
        assert_eq!(state.play_button.render_text.text, "Resume");

        let mut input = Input::new();
        input.enter_pressed = true;
        PauseSystem.update_state(&mut input, &mut state, &mut Vec::new());

        assert_eq!(state.game_state, GameState::Playing);
    }

    #[test]
    fn returning_to_the_menu_restores_its_text() {
        let mut state = playing_state();
        PauseSystem.start(&mut state);

        MenuSystem.start(&mut state);

        assert_eq!(state.title_text.render_text.text, "SNAKE");
        assert_eq!(state.play_button.render_text.text, "Play");
    }

    #[test]
    fn starting_a_run_resets_the_snake() {
        let mut state = playing_state();
        let grid = state.grid;
        state.snake.score = 7;
        state.snake.speed = 20.0;
        state.snake.update_direction(Direction::Left);
        state.snake.grow_body(&grid);

        PlaySystem.start(&mut state);

        assert_eq!(state.snake.score, 0);
        assert_eq!(state.snake.speed, util::STARTING_SNAKE_SPEED);
        assert_eq!(state.snake.body.len(), 1);
        assert_eq!(state.snake.direction, Direction::None);
        assert_eq!(state.snake.head_cell(&grid), (grid.cols / 2, grid.rows / 2));
    }
}
