//! The ship is the player's avatar in the game.
use wasm_bindgen::prelude::wasm_bindgen;

use crate::model::{
    game_settings::field::Field,
    positioned::{Moves, Positioned},
};

/// The ship is the player's avatar in the game.
///
/// It stays in the middle of the screen and can only be rotated by the player.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Ship {
    /// The x position of the ship
    pub x: f32,
    /// The y position of the ship
    pub y: f32,
    /// The y direction the this ship is looking
    pub dy: f32,
    /// The x direction the this ship is looking
    pub dx: f32,
    /// The height of the ship
    pub height: f32,
    /// The width of the ship
    pub width: f32,
}

#[wasm_bindgen]
impl Ship {
    /// Creates a new ship in the middle of the field
    pub fn new(field: &Field) -> Self {
        Ship {
            x: field.width as f32 / 2.0,
            y: field.height as f32 / 2.0,
            dx: 0.0,
            dy: 0.0,
            width: 80.0,
            height: 80.0,
        }
    }
    /// Calculates the angle the ship is facing in radians
    pub fn get_angle(&self) -> f32 {
        let (x, y) = (self.dx, self.dy);
        y.atan2(x)
    }
}

impl Positioned for Ship {
    fn is_round(&self) -> bool {
        true
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn dimensions(&self) -> (f32, f32) {
        (self.width, self.height)
    }
}
