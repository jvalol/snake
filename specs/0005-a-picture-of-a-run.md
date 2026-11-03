# 0005 A picture of a run

**Status:** implemented
**Date:** 2026-11-03

## Goal

The frame snake puts on its readme is held against a picture of what that
frame should be.

## Why it belongs here

Specs 0001 to 0004 are arithmetic: where the head is, what it eats, when it
crashes, what the state machine does next. None of them can see the board.

**The engine's first harness could not either**, and nor could its
tolerance. snake pushes every quad into `Geometry` and never touches
`Scene`, so the 3D call photographs a black frame of it. The engine's spec
0078 is the call that takes the overlay and the tighter share that goes
with a flat game.

## Behavior

**`SnakeGame::new` takes `staged` rather than reading the arguments.** A
picture test asks for the photographed state without `--screenshot` in its
own command line.

**The staged pellet has a place.** `place_pellet` puts it on a random free
cell, so no two staged runs agreed and every regenerated screenshot of
snake had the pellet somewhere new. The pose now puts it three quarters
across and a third down, clear of a snake that starts in the middle going
right. This is a change to what `refresh-screenshots` photographs, so
snake's committed screenshot moves the next time it is taken.

**The pose is driven rather than held still.** The other games stage
themselves inside one update and stop. This one is on a clock: it grows the
snake, turns it up, turns it right and holds at `HOLDS_AT`, after which it
sets `delta_time` to nought itself. So the test hands it the same fixed
sixtieths the staged run gets, and the same count always draws the same
frame. Nothing in it is a real second.

**It is the engine's size**, 320 by 240, and the engine's `SPARSE` share,
which the overlay call picks on its own.

## Acceptance criteria

- A run mid-way matches its
  picture. — `snake_game::tests::the_snake_mid_run_looks_right`

### Verified by hand

- **The picture itself.** The snake with both its turns in it, the pellet
  up and to the right where the pose puts it, the score along the top.
- **A control.** One segment off the staged body fails it, at 0.1%. The
  same control passed under the engine's 3D share, and so did two
  segments; it took four before that one noticed, which is what spec 0078
  was written from.
- **That it is repeatable.** Two fresh runs came back identical byte for
  byte, and identical to the committed reference.

## Out of scope

**The score line's size.** At 320 by 240 the readout is laid out for a
larger window and "Score:" runs most of the way across, with the number
past the edge. The picture shows that honestly. It is a layout question
about small windows rather than anything this test should paper over.

**The menu, the pause screen and the game over screen.** Each is a state
spec 0004 already tests.
