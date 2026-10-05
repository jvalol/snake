use blitzkit::geometry::Geometry;
use blitzkit::keyboard::*;
use blitzkit::renderer::render_text::TextRenderer;
use blitzkit::sound::SoundSystem;
use blitzkit::Game;

use std::io::Cursor;

use crate::input::Input;
use crate::state::*;
use crate::system::*;
use crate::util::Direction;

const BOUNCE_BYTES: &[u8] = include_bytes!("../res/sounds/4362__noisecollector__pongblipa-4.wav");

pub struct SoundPack {
    bounce: Cursor<&'static [u8]>,
}

impl SoundPack {
    pub fn new() -> Self {
        Self {
            bounce: Cursor::new(BOUNCE_BYTES),
        }
    }

    pub fn bounce(&self) -> rodio::Decoder<Cursor<&'static [u8]>> {
        rodio::Decoder::new(self.bounce.clone()).unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Event {
    ButtonPressed,
    SnakeCrashed,
    Score,
}

pub struct SnakeGame {
    pub input: Input,
    events: Vec<Event>,
    state: State,
    visibility_system: VisibilitySystem,
    play_system: PlaySystem,
    pause_system: PauseSystem,
    game_over_system: GameOverSystem,
    sound_pack: SoundPack,
    /// Whether this run is only here to be photographed, and how long it has
    /// been posing. See `refresh-screenshots` in the project above.
    ///
    /// A picture of the main menu says nothing about snake, and a picture of a
    /// snake that has just started playing is two squares in a line. This
    /// plays the game: it starts, grows to nine, turns twice, and then holds
    /// still to be photographed.
    staged: bool,
    posing: f32,
}

impl SnakeGame {
    /// How long the posed snake plays before it holds still, and when it turns.
    ///
    /// Two turns, so the body reads as a snake that has been somewhere rather
    /// than a bar. It holds by having no time pass rather than by skipping the
    /// systems, so everything else still draws.
    ///
    /// A nine segment body covers the last nine cells travelled, so to show
    /// two turns each leg has to be about three cells. At eight cells a second
    /// that is four tenths of a second. The first try turned every seven cells
    /// and the body could only hold one of the turns.
    const GROWN: usize = 8;
    const FIRST_TURN: f32 = 0.42;
    const SECOND_TURN: f32 = 0.80;
    const HOLDS_AT: f32 = 1.18;

    /// Plays the game for the camera. See `refresh-screenshots`.
    fn pose(&mut self, dt: f32) {
        let was = self.posing;
        self.posing += dt;

        if was == 0.0 {
            self.state.game_state = GameState::Playing;
            self.play_system.start(&mut self.state);
            self.state.snake.update_direction(Direction::Right);
            for _ in 0..Self::GROWN {
                self.state.snake.grow_body(&self.state.grid);
            }
        }

        let now = self.posing;
        let crossed = |at: f32| was < at && now >= at;
        if crossed(Self::FIRST_TURN) {
            self.state.snake.update_direction(Direction::Up);
        }
        if crossed(Self::SECOND_TURN) {
            self.state.snake.update_direction(Direction::Right);
        }
        if self.posing >= Self::HOLDS_AT {
            self.state.delta_time = 0.0;
        }
    }

    pub fn new() -> Self {
        Self {
            input: Input::new(),
            events: Vec::new(),
            state: State::new(),
            visibility_system: VisibilitySystem,
            play_system: PlaySystem,
            pause_system: PauseSystem,
            game_over_system: GameOverSystem,
            sound_pack: SoundPack::new(),
            staged: crate::staged(),
            posing: 0.0,
        }
    }
}

impl Game for SnakeGame {
    fn initialize(
        &mut self,
        geometry: &mut Geometry,
        text_renderer: &mut TextRenderer,
        _sound_system: &SoundSystem,
        window_size: (f32, f32),
    ) {
        self.resized(window_size);
        self.play_system.start(&mut self.state);
        self.state.initialize(geometry, text_renderer);
    }

    fn update(
        &mut self,
        dt: f32,
        geometry: &mut Geometry,
        text_renderer: &mut TextRenderer,
        sound_system: &SoundSystem,
    ) {
        self.state.delta_time = dt;

        if self.staged {
            self.pose(dt);
        }

        for event in &self.events {
            match event {
                Event::ButtonPressed | Event::SnakeCrashed => {
                    sound_system.queue(self.sound_pack.bounce());
                }
                Event::Score => {
                    sound_system.queue(self.sound_pack.bounce());
                }
            }
        }
        self.events.clear();

        self.visibility_system
            .update_state(&mut self.input, &mut self.state, &mut self.events);

        match self.state.game_state {
            GameState::Playing => {
                self.play_system
                    .update_state(&mut self.input, &mut self.state, &mut self.events);

                if self.state.game_state == GameState::GameOver {
                    self.game_over_system.start(&mut self.state);
                }
            }
            GameState::Paused => {
                self.pause_system
                    .update_state(&mut self.input, &mut self.state, &mut self.events);
            }
            GameState::GameOver => {
                self.game_over_system.update_state(
                    &mut self.input,
                    &mut self.state,
                    &mut self.events,
                );
                if self.state.game_state == GameState::Playing {
                    self.play_system.start(&mut self.state);
                }
            }
            GameState::Quitting => {}
        }

        geometry.reset();
        text_renderer.reset();

        self.state.update(geometry, text_renderer);
    }

    fn process_keyboard(&mut self, input: KeyboardInput) {
        self.input.update(input);
    }

    fn is_quitting(&self) -> bool {
        self.state.game_state == GameState::Quitting
    }

    fn focus_changed(&mut self, focus: bool) {
        // a staged run is photographed from behind the terminal, so it never
        // has focus and pausing on losing it would photograph the pause screen
        if self.staged {
            return;
        }

        // only a run can be paused; losing focus on the menu or the game over
        // screen leaves the screen alone
        if !focus && self.state.game_state == GameState::Playing {
            self.pause_system.start(&mut self.state);
            self.state.pause_game();
        }
    }

    fn resized(&mut self, window_size: (f32, f32)) {
        self.state.layout(window_size.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game_in(game_state: GameState) -> SnakeGame {
        let mut game = SnakeGame::new();
        game.state.layout((800.0, 600.0).into());
        game.state.game_state = game_state;
        game
    }

    #[test]
    fn losing_focus_while_playing_pauses() {
        let mut game = game_in(GameState::Playing);
        game.focus_changed(false);

        assert_eq!(game.state.game_state, GameState::Paused);
        assert_eq!(
            game.state.pause_text.render_text.text,
            crate::system::PAUSED
        );
    }

    #[test]
    fn escaping_out_of_a_pause_quits() {
        let mut game = game_in(GameState::Playing);
        game.focus_changed(false);
        assert_eq!(game.state.game_state, GameState::Paused);

        game.input.esc_pressed = true;
        let mut geometry = Geometry::new();
        let mut text_renderer = TextRenderer::new();
        let sound_system = SoundSystem::new();
        game.update(0.016, &mut geometry, &mut text_renderer, &sound_system);

        assert_eq!(game.state.game_state, GameState::Quitting);
    }

    #[test]
    fn losing_focus_while_already_over_does_nothing() {
        let mut game = game_in(GameState::GameOver);
        game.focus_changed(false);

        assert_eq!(game.state.game_state, GameState::GameOver);
    }
}
