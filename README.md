# antz

A small game inspired by `AntMe!` where you control an ant colony through scripting in order to get more points.

The game is written in Rust and uses `ggez` as game engine.

## Run the game

To run the game you need [rust](https://rustup.rs/) and [just](https://just.systems/) installed.

Once both are installed, clone the repository and run `just run` inside.

```sh
git clone https://github.com/hardliner66/antz
cd antz
just run
```

# WIP

Currently the project is work in progress and not in a usable state.

Things that already work:
- rendering
- ant spawns
- apple spawns
- sugar spawns
- using wasm plugins
- wasm events
  - on_idle
  - on_near_sugar
  - on_near_apple
- wasm api
  - basic movement
    - turn
    - turn_to
    - move_forward
  - random functions for wasm
    - rand
    - rand_range
    - rand_range_int
    - rand_range_uint
  - other
    - clear

Things that don't work, but will work in the future:
- scoring
- more functions to control ants
- stable API
- simulate multiple ant colonies at once
- UI (maybe)
