//! This module contains the game settings.
//!
//! The game settings are used to configure the game.
//!
//! This allows for the shop-items to just change those values.

use self::{field::Field, ship::Ship};

pub mod field;
pub mod ship;

/// The settings of the game
///
/// This struct contains all the settings of the game. It is used to configure the game.
///
/// This allows for the shop-items to just change those values.
#[derive(Clone)]
pub struct GameSettings {
    /// The speed of the projectiles
    pub projectile_speed: f32,
    /// The probability that the enemy will be an asteroid. The higher the number the higher the chance that the enemy will be an asteroid
    pub asteroid_probability: f32,
    /// The multiplier for the coins gained by destroying an enemy
    pub coin_multiplier: i32,
    /// The visibility of the ship. The lower the number the lower the chance the enemy will fly directly towards the ship. Number between 0 and 10.
    pub ship_visibility: f32,
    /// The multiplier for the speed of the enemies
    pub enemy_speed_multiplier: f32,
    /// The timeout between shots. The lower the number the higher the shooting rate
    pub shoot_timeout: i32,
    /// The increase in score per frame
    pub score_increase: f64,
    /// The maximum number of lives
    pub max_lives: i32,
    /// The minimum speed of the enemies
    pub enemy_min_speed: f32,
    /// The field
    pub field: Field,
    /// The ship
    pub ship: Ship,
    /// The time the ship is invulnerable after being hit
    pub invulnerability_time: i32,
    /// The timeout between enemy spawns. The lower the number the higher the spawn rate
    pub enemy_spawn_timeout: f32,
    /// The number of enemies a projectile can hit before being destroyed
    pub projectile_hits: i32,
}

impl GameSettings {
    /// Creates a new game settings struct
    pub fn new() -> Self {
        let width = 1100;
        let height = 700;
        let field: Field = Field::new(width, height);
        Self {
            projectile_speed: 5.0,
            asteroid_probability: 0.6,
            enemy_speed_multiplier: 2.0,
            coin_multiplier: 10,
            score_increase: 0.03,
            ship_visibility: 5.0,
            enemy_min_speed: 0.5,
            shoot_timeout: 70,
            max_lives: 3,
            field,
            ship: Ship::new(&field),
            invulnerability_time: 100,
            enemy_spawn_timeout: 80.0,
            projectile_hits: 1,
        }
    }
}
