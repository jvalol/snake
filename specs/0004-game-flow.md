# 0004 Game flow

**Status:** implemented
**Date:** 2026-09-20

## Goal

Getting in and out of a run is obvious, and a new run always starts clean.

## Behavior

**Menu.** SNAKE with Play and Quit. Up and Down move between them, Enter chooses,
and Escape quits.

**Playing.** The run, per specs 0002 and 0003. Escape returns to the menu.

**Paused.** Losing window focus during a run pauses it and shows Paused with
Resume. Enter resumes. Losing focus anywhere else, such as on the menu or the
game over screen, changes nothing. Returning to the menu puts its own wording
back.

**Game over.** "Game Over" is shown for five seconds, then the game returns to the
menu. Escape quits from here.

Starting a run resets the snake to one segment in the middle, the score to zero,
and the speed to its starting value, and places a fresh pellet.

Escape is acted on once per press. Holding it does not carry from a run into the
menu and quit the game.

## Acceptance criteria

- Escape during a run returns to the menu. — `system::tests::escape_returns_to_the_menu`
- Escape from the menu quits. — `system::tests::escape_quits_from_the_menu`
- Starting a run resets the snake, score, and speed. — `system::tests::starting_a_run_resets_the_snake`
- Escape is ignored on key repeat. — `input::tests::escape_ignores_key_repeat`
- Escape is ignored on release. — `input::tests::escape_ignores_release`
- Losing focus during a run pauses it. — `snake_game::tests::losing_focus_while_playing_pauses`
- Losing focus on the menu changes nothing. — `snake_game::tests::losing_focus_on_the_menu_does_nothing`
- Enter resumes a paused run. — `system::tests::resuming_returns_to_playing`
- The menu's wording comes back when a run ends. — `system::tests::returning_to_the_menu_restores_its_text`

### Verified by hand

- The five second game over wait feels right. — run snake and crash.

## Out of scope

A pause key, a high score table, and remembering anything between runs.
