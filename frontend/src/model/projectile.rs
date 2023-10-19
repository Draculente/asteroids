//! Projectile module

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use super::{
    game::Game,
    positioned::{Moves, Positioned},
};

/// A projectile is a bullet that is shot by the player.
///
/// It moves in a straight line and can hit enemies.
/// The projectile is destroyed when the hit count reaches 0 or it leaves the field.
#[wasm_bindgen]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Projectile {
    /// The id of the projectile. This is used to identify the projectile.
    pub id: i64,
    /// The x direction of the projectile
    pub dx: f32,
    /// The y direction of the projectile
    pub dy: f32,
    /// The x position of the projectile
    pub x: f32,
    /// The y position of the projectile
    pub y: f32,
    /// the number of hits this projectile can take before it is destroyed
    pub hits: i32,
    /// The damage this projectile deals on hit
    pub damage: i32,
    /// The height of the projectile
    pub radius: f32,
}

impl Projectile {
    pub fn new(game: &Game) -> Self {
        let (dx, dy) = (game.game_settings.ship.dx, game.game_settings.ship.dy);
        let (x, y) = game.game_settings.ship.get_center();
        let length = (dx.powi(2) + dy.powi(2)).sqrt();
        let (norm_dx, norm_dy) = (dx / length, dy / length);
        Self {
            id: game.get_next_id(),
            dx: norm_dx * game.game_settings.projectile_speed as f32,
            dy: -norm_dy * game.game_settings.projectile_speed as f32,
            x: x + (norm_dx * game.game_settings.ship.width / 2.0),
            y: y - (norm_dy * game.game_settings.ship.height / 2.0),
            hits: game.game_settings.projectile_hits,
            damage: 1,
            radius: 20.0,
        }
    }
}

impl Positioned for Projectile {
    fn is_round(&self) -> bool {
        true
    }

    fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn dimensions(&self) -> (f32, f32) {
        (self.radius, self.radius)
    }
}

impl Moves for Projectile {
    fn direction(&self) -> (f32, f32) {
        (self.dx, self.dy)
    }
}
