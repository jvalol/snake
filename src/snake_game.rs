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

            // **And the pellet, which is otherwise wherever the dice said.**
            // A staged run exists to be photographed, and `place_pellet`
            // puts it on a random free cell, so every screenshot of snake
            // had the pellet somewhere else and no two runs agreed. Three
            // quarters across and a third down is clear of the snake, which
            // starts in the middle going right.
            let grid = self.state.grid;
            self.state
                .pellet
                .place(&grid, (grid.cols * 3 / 4, grid.rows / 3));
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

    /// `staged` rather than a call to `crate::staged()`, so that a picture
    /// test can ask for the photographed state without putting
    /// `--screenshot` in its own arguments. `main` passes what the
    /// arguments say.
    pub fn new(staged: bool) -> Self {
        Self {
            input: Input::new(),
            events: Vec::new(),
            state: State::new(),
            visibility_system: VisibilitySystem,
            play_system: PlaySystem,
            pause_system: PauseSystem,
            game_over_system: GameOverSystem,
            sound_pack: SoundPack::new(),
            staged,
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

    /// How big the picture is: the engine's own size, said once there and
    /// read here, per its spec 0062.
    const WIDE: u32 = blitzkit::picture::SIZE.0;
    const TALL: u32 = blitzkit::picture::SIZE.1;

    /// The snake mid-run, held against a picture of it. Spec 0005.
    ///
    /// **Through the overlay call, because snake is all overlay.** Every
    /// quad it has goes into `Geometry` and it never touches `Scene`, so
    /// the 3D-only harness photographs a blank frame of it. The engine's
    /// spec 0078 is that call.
    ///
    /// **Driven rather than held still.** The other games stage
    /// themselves inside one update and then stop, but this pose is on a
    /// clock: it grows the snake, turns it up, turns it right again and
    /// holds. So the test gives it the same fixed sixtieths the staged
    /// run gets, until `HOLDS_AT` has passed and the pose stops the clock
    /// itself. Nothing here is a real second, and the same count always
    /// draws the same frame.
    #[test]
    fn the_snake_mid_run_looks_right() {
        let reference =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/pictures/mid-run.png");

        blitzkit::picture::drawn_with_overlay_looks_right(
            &reference,
            (WIDE, TALL),
            |renderer, geometry, text| {
                let mut game = SnakeGame::new(true);
                let sound = SoundSystem::new();
                let mut scene = blitzkit::renderer::scene::Scene::new();
                let mut camera = blitzkit::camera::Camera::new();

                game.load(renderer);
                game.initialize(geometry, text, &sound, (WIDE as f32, TALL as f32));

                let step = 1.0 / 60.0;
                let steps = (SnakeGame::HOLDS_AT / step).ceil() as u32 + 1;
                for _ in 0..steps {
                    game.update(step, geometry, text, &sound);
                }

                game.draw(&mut scene, &mut camera);

                (scene, camera)
            },
        );
    }

    fn game_in(game_state: GameState) -> SnakeGame {
        let mut game = SnakeGame::new(false);
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
