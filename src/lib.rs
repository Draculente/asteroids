//! This is the documentation for the rust part of the [Asteroids](https://asteroids.edu.k8s.th-luebeck.dev/) game.
//! It contains the GameModel and the Game struct.
//!
//! The [GameModel] is the entry point of the game. This model creates the game and manages it by e.g. recreating it when the player wants to start a new one. It also contains the error messages displayed to the user.
//!
//! The [Game](Game) struct is the heart of this model. It contains the logic of the central `tick` method used to calculate the next frame, holds all objects of the game and orchestrates their interaction. It also contains the [GameSettings](GameSettings) which are used by the shop system to apply the item functions.
use wasm_bindgen::prelude::*;

mod model;
// Those imports are used in the doc-block above.
use model::game::Game;
use model::game_settings::GameSettings;
pub use model::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    // console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}
