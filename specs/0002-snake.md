# 0002 Snake

**Status:** implemented
**Date:** 2026-09-20

## Goal

Movement that feels continuous but plays on the grid.

## Behavior

The snake starts in the middle of the playfield, one segment long, and still. It
moves once a direction is chosen, and it cannot be stopped again.

The head's position moves smoothly, at the snake's speed in cells per second times
the frame's delta time. The body only changes when the head crosses into a new
cell: a new head segment appears in that cell and the last segment is dropped, so
the snake keeps its length.

Arrows or WASD set the direction. A direction that reverses the snake onto itself
is ignored, so a snake moving left cannot be sent right.

Eating grows the snake by one segment, placed in the cell behind the tail, and
speeds it up by 0.5 cells per second. It starts at 8.

A crash is the head leaving the playfield or entering a cell one of the other
segments occupies. Either ends the game.

## Acceptance criteria

- The snake starts still, one segment, in the middle. — `snake::tests::starts_in_the_middle`
- A still snake does not move. — `snake::tests::does_not_move_without_a_direction`
- The head moves at its speed in cells per second. — `snake::tests::advances_at_its_speed`
- The body only follows when a cell boundary is crossed. — `snake::tests::body_follows_on_cell_change`
- Growing adds a segment behind the tail. — `snake::tests::growing_adds_a_segment_behind`
- The head sharing a cell with its body is a bite. — `snake::tests::bites_itself_when_the_head_meets_the_body`
- Regridding keeps every segment's cell. — `snake::tests::regrid_keeps_cells`
- Leaving the playfield ends the game. — `system::tests::leaving_the_playfield_crashes`
- Biting itself ends the game. — `system::tests::biting_itself_crashes`
- A reversing direction is ignored. — `system::tests::cannot_reverse_onto_itself`
- A turn to a new direction is taken. — `system::tests::turns_to_a_new_direction`

### Verified by hand

- The snake moves at a playable speed. — run snake and play a round.

## Out of scope

Queued turns within one cell, wrapping around the edges, and difficulty levels.
