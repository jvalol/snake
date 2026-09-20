# 0001 Playfield

**Status:** implemented
**Date:** 2026-09-20

## Goal

A grid of square cells that fills the window, so the snake reads as moving on
graph paper rather than through a stretched space.

## Behavior

`Grid` divides the window into square cells. The cell size is the window height
divided by 32 rows, rounded down, and the playfield then takes as many whole
columns and rows as fit and is centered in the window. Leftover pixels at the
edges are outside the playfield.

Positions are pixels, with the origin at the window's top-left and y down. The
game's rules work in cells: `cell_at` gives the cell holding a position, and
`center_of` gives a cell's center in pixels. A cell outside the playfield is
negative or past the column and row counts, which is how a crash is detected.

The border is drawn as four quads on the playfield edge. They are decoration, not
collision.

`layout` rebuilds the grid whenever the window changes size. Mid-game the snake
and the pellet keep their cells, clamped to the new bounds, so a resize doesn't
end a run.

## Acceptance criteria

- Cells are square and sized from the window height. — `coords::tests::cells_are_square`
- The playfield is centered, with leftover pixels split between the edges. — `coords::tests::playfield_is_centered`
- A position maps to the cell holding it. — `coords::tests::cell_at_finds_the_cell`
- A cell maps back to its center in pixels. — `coords::tests::center_of_is_the_middle_of_the_cell`
- Cells outside the playfield are not contained. — `coords::tests::contains_rejects_cells_outside`
- The border is four quads. — `state::tests::layout_makes_four_walls`
- A resize keeps the snake and pellet in their cells. — `state::tests::resize_keeps_cells`

### Verified by hand

- Cells look square in a wide or tall window. — run snake and resize it.

## Out of scope

Choosing the grid size at runtime, non-square cells, and scrolling playfields.
