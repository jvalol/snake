# 0003 Pellets

**Status:** implemented
**Date:** 2026-09-20

## Goal

Something to chase that never appears somewhere unreachable or unfair.

## Behavior

One pellet is on the playfield at a time, filling a single cell. Eating it means
the head entering its cell.

Eating scores a point, grows the snake, speeds it up, and places a new pellet on a
random cell that no part of the snake occupies. When the playfield is essentially
full, the placement gives up after a hundred tries and uses the middle cell.

The score reads "Score: N" and updates while playing.

## Acceptance criteria

- Eating scores a point and grows the snake. — `system::tests::eating_scores_and_grows`
- Eating speeds the snake up. — `system::tests::eating_speeds_the_snake_up`
- A new pellet never lands on the snake. — `system::tests::new_pellet_avoids_the_snake`
- A pellet fills exactly one cell. — `system::tests::pellet_fills_one_cell`

### Verified by hand

- Eating plays a sound. — run snake and eat a pellet.

## Out of scope

Several pellets at once, pellets worth different amounts, and pellets that expire.
