use blitzkit::geometry::Geometry;
use blitzkit::keyboard::*;
use blitzkit::renderer::render_text::TextRenderer;
use blitzkit::sound::SoundSystem;
use blitzkit::Game;

use std::io::Cursor;

use crate::input::Input;
use crate::state::*;
use crate::system::*;

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
  FocusChanged,
  SnakeCrashed,
  Score,
}

pub struct SnakeGame {
  pub input: Input,
  events: Vec<Event>,
  state: State,
  menu_system: MenuSystem,
  visibility_system: VisibilitySystem,
  play_system: PlaySystem,
  pause_system: PauseSystem,
  game_over_system: GameOverSystem,
  sound_pack: SoundPack,
}

impl SnakeGame {
  pub fn new() -> Self {
    Self {
      input: Input::new(),
      events: Vec::new(),
      state: State::new(),
      menu_system: MenuSystem,
      visibility_system: VisibilitySystem,
      play_system: PlaySystem,
      pause_system: PauseSystem,
      game_over_system: GameOverSystem::new(),
      sound_pack: SoundPack::new(),
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
    self.menu_system.start(&mut self.state);
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

    for event in &self.events {
      match event {
        Event::FocusChanged | Event::ButtonPressed | Event::SnakeCrashed => {
          sound_system.queue(self.sound_pack.bounce());
        }
        Event::Score => {
          sound_system.queue(self.sound_pack.bounce());
        }
      }
    }
    self.events.clear();

    self
      .visibility_system
      .update_state(&mut self.input, &mut self.state, &mut self.events);

    match self.state.game_state {
      GameState::MainMenu => {
        self
          .menu_system
          .update_state(&mut self.input, &mut self.state, &mut self.events);
        if self.state.game_state == GameState::Playing {
          self.play_system.start(&mut self.state);
        }
      }
      GameState::Playing => {
        self
          .play_system
          .update_state(&mut self.input, &mut self.state, &mut self.events);

        if self.state.game_state == GameState::MainMenu {
          self.menu_system.start(&mut self.state);
        } else if self.state.game_state == GameState::GameOver {
          self.game_over_system.start(&mut self.state);
        }
      }
      GameState::Paused => {
        self
          .pause_system
          .update_state(&mut self.input, &mut self.state, &mut self.events);
      }
      GameState::GameOver => {
        self
          .game_over_system
          .update_state(&mut self.input, &mut self.state, &mut self.events);
        if self.state.game_state == GameState::MainMenu {
          self.menu_system.start(&mut self.state);
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
    assert_eq!(game.state.play_button.render_text.text, "Resume");
  }

  #[test]
  fn losing_focus_on_the_menu_does_nothing() {
    let mut game = game_in(GameState::MainMenu);
    game.focus_changed(false);

    assert_eq!(game.state.game_state, GameState::MainMenu);
    assert_eq!(game.state.title_text.render_text.text, "SNAKE");
    assert_eq!(game.state.play_button.render_text.text, "Play");
  }
}
