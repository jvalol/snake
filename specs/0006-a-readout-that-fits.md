# 0006 A readout that fits

**Status:** implemented
**Date:** 2026-11-03

## Goal

The score fits whatever window it is in.

## Why it belongs here

The playfield follows the window: `Grid::new` works a cell out of the
window's height and lays the field out in cells. The score did not. It was
32 points at (120, 20), which are fixed pixels, so the smaller the window
the larger the readout was against everything else in it.

At 320 by 240 the result is "Score:" running most of the way across the top
with the number past the right edge, which spec 0005's picture showed
honestly and nothing else in the repo could see. No window anybody plays in
is that small, which is why it sat there.

## Behavior

**The score is placed in cells, like everything else here.** One and a tenth
cells tall, four cells in from the field's left edge, seven tenths of a cell
down from its top. At the window this game ships in those are within a pixel
or two of the 32, 120 and 20 they replace, which is what says the old
numbers were a cell-sized readout written down in pixels.

## Acceptance criteria

- The score fits at the engine's picture size, at 640 by 480 and at the
  window the screenshot is taken in. — `state::tests::the_score_fits_the_window`

### Verified by hand

- **The picture.** Spec 0005's reference is re-blessed and "Score: 0" now
  sits in the top left corner with the field clear beside it.
- **A control.** The fixed 32 at (120, 20) put back, and the test says the
  score runs to 440 in a 320 wide window, which is 120 past the edge.

## Out of scope

**The pause and win text.** Both are centred by `layout` already and both
are 24 and 32 points of fixed size. Neither overflowed at any size tried,
and a line that is centred fails differently from one that is not.

**A general rule about readouts.** This game's unit is the cell. Another
game's is not, and spec 0005 of pong says in as many words that its own
layout is a fraction of the window.
