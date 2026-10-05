# 0004 Game flow

**Status:** implemented
**Date:** 2026-09-20

## Goal

Getting in and out of a run is obvious, and a new run always starts clean.

## Behavior

**No menu.** It opens on a run. There used to be a title with Play and Quit in
front of it, which made sense when a game was the only thing you could have
launched and makes none now: the arcade is the menu, and a splash screen in
front of a game you walked up to and started is a second front door.

**Playing.** The run, per specs 0002 and 0003. Escape quits.

**Paused.** Losing window focus during a run pauses it and shows one line on a
panel. Enter carries on, and Escape quits the way it does during a run, so a
pause is not the one state the key stops working in. Losing focus anywhere else,
such as on the game over screen, changes nothing.

**Game over.** "Game Over" sits on a panel until Enter starts a fresh run. It
used to count five seconds down to the menu; with no menu to return to, it waits
to be asked, the way cascada and carom already do. Escape quits from here.

Starting a run resets the snake to one segment in the middle, the score to zero,
and the speed to its starting value, and places a fresh pellet.

Escape is acted on once per press, so holding it cannot quit twice.

## Acceptance criteria

- Escape during a run quits. — `system::tests::escape_quits_a_game_in_play`
- Starting a run resets the snake, score, and speed. — `system::tests::starting_a_run_resets_the_snake`
- Escape is ignored on key repeat. — `input::tests::escape_ignores_key_repeat`
- Escape is ignored on release. — `input::tests::escape_ignores_release`
- Losing focus during a run pauses it. — `snake_game::tests::losing_focus_while_playing_pauses`
- Losing focus while already over changes nothing. — `snake_game::tests::losing_focus_while_already_over_does_nothing`
- Enter resumes a paused run. — `system::tests::resuming_returns_to_playing`
- An ended run waits to be asked. — `system::tests::an_ended_game_waits_to_be_asked`
- Escape leaves a paused run. — `system::tests::escape_leaves_a_paused_game`
- And quits rather than backing out. — `snake_game::tests::escaping_out_of_a_pause_quits`

### Verified by hand

- The five second game over wait feels right. — run snake and crash.

## Out of scope

A pause key, a high score table, and remembering anything between runs.
