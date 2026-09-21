# snake

Eat pellets, grow, don't hit the walls or yourself. Built on `blitzkit`, which
lives at `../rust/blitzkit` and owns the window, rendering, input, and sound.

## Build and test

Requires Rust 1.87 or newer, the engine's MSRV.

```
cargo build
cargo test
cargo run
cargo clippy
cargo fmt
```

## How work happens here

Behavior changes are spec driven:

1. **Write the spec first.** Copy `specs/TEMPLATE.md` to `specs/NNNN-short-name.md`
   and fill it in. Say what the game does, not how the code does it.
2. **Make the acceptance criteria testable.** Each one names the test that proves
   it, or goes under "Verified by hand" when it needs a window or a speaker.
3. **Write the tests, then the code.** `cargo test` passes before a commit.
4. **Update the spec when behavior changes.** A spec that disagrees with the game
   is a bug in the spec.

## Layout

- `src/main.rs` — hands a `SnakeGame` to `blitzkit::start`.
- `src/snake_game.rs` — the `Game` impl, the event list, sound.
- `src/coords.rs` — `Grid`, the playfield, and every pixel-to-cell conversion.
- `src/state.rs` — everything the game knows, and `layout()` which builds the grid.
- `src/system.rs` — one system per game state, each stepping the state.
- `src/snake.rs`, `src/pellet.rs` — the moving pieces.
- `src/input.rs` — engine key events to held flags.
- `src/util.rs` — grid size, speeds, and the direction enum.

## Conventions

- **Pixels for drawing, cells for rules.** The engine draws in pixels with the
  origin at the top-left and y down. Everything the game decides, such as crashes,
  eating, and growth, compares cells through `Grid`, never pixel distances.
- **`Grid` owns the conversions.** Nothing else divides by a cell size.
- **Speeds are per second**, in cells, multiplied by `State::delta_time`.
- **Systems are pure game logic.** They take input, state, and events, and touch no
  GPU, window, or audio. That is what makes them testable, so keep it that way.
- Tests live next to the code in `#[cfg(test)] mod tests`.
