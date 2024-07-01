//! This module contains the enemy model. It contains the enemy type and the enemy struct.

use std::hash::{Hash, Hasher};

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use unique_id::{sequence::SequenceGenerator, Generator};
use wasm_bindgen::prelude::wasm_bindgen;

use super::{
    game_settings::GameSettings,
    positioned::{Moves, Positioned},
};

/// The type of the enemy. This determines the enemy's appearance and behavior.
#[wasm_bindgen]
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum EnemyType {
    Asteroid,
    Ship,
    Debris,
}

impl EnemyType {
    /// Creates a new enemy type.
    ///
    /// The chance that the enemy will be a ship will increase over time and with the score
    fn new(game_settings: &GameSettings, score: f64) -> Self {
        let mut rng = thread_rng();
        // The chance that the enemy will be a ship will increase over time and with the score
        let asteroid_probability =
            1.0 - (score / 1000.0).min(game_settings.asteroid_probability as f64);
        if rng.gen_bool(asteroid_probability) {
            EnemyType::Asteroid
        } else {
            EnemyType::Ship
        }
    }
}

/// The model of an enemy. It contains all the information about the enemy state.
#[wasm_bindgen]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Enemy {
    dy: f32,
    dx: f32,
    /// The id of the enemy. This is used to identify the enemy.
    pub id: i64,
    /// The x position of the enemy
    pub x: f32,
    /// The y position of the enemy
    pub y: f32,
    /// The radius of the enemy
    pub radius: f32,
    /// The radius limit below that the enemy will not split into smaller enemies on hit
    reproduce_limit: f32,
    /// The type of the enemy
    pub enemy_type: EnemyType,
    /// The health of the enemy
    pub health: i32,
}

impl Enemy {
    /// Creates a new enemy with a random position and speed
    pub fn new(game_settings: &GameSettings, score: f64, id: i64) -> Self {
        let mut rng = thread_rng();
        let radius: f32 = rng.gen_range(20.0..40.0);
        let reproduce_limit = rng.gen_range(5.0..10.0);

        let enemy_type = EnemyType::new(&game_settings, score);

        let health = rng.gen_range(1..2);

        let (x, y) = if rng.gen_bool(0.5) {
            (
                rng.gen_range(0.0..(game_settings.field.width + radius.ceil() as i32) as f32),
                if rng.gen_bool(0.5) {
                    0.0 - radius.ceil()
                } else {
                    game_settings.field.height as f32 + radius.ceil()
                },
            )
        } else {
            (
                if rng.gen_bool(0.5) {
                    0.0 - radius.ceil()
                } else {
                    game_settings.field.width as f32 + radius.ceil()
                },
                rng.gen_range(0.0..(game_settings.field.height + radius.ceil() as i32) as f32),
            )
        };
        let (dx, dy) = Enemy::calculate_speed(&game_settings, (x, y), &enemy_type);
        // Spawn the enemy at the edge of the field

        Self {
            id,
            dy,
            dx,
            x,
            y,
            radius,
            reproduce_limit,
            enemy_type,
            health,
        }
    }

    /// Calculates the speed of the enemy based on the game settings and the enemy type.
    ///
    /// If the enemy is a ship it has a chance that it will fly directly towards the ship.
    fn calculate_speed(
        game_settings: &GameSettings,
        pos: (f32, f32),
        enemy_type: &EnemyType,
    ) -> (f32, f32) {
        let mut rng = thread_rng();
        let speed_multi = game_settings.enemy_speed_multiplier;
        loop {
            let (mut dx, mut dy) = (
                rng.gen_range(-speed_multi..speed_multi),
                rng.gen_range(-speed_multi..speed_multi),
            );

            let probability = rng.gen_range(0.0..10.0);

            // If the enemy is a ship and the probability is less than the ship visibility:
            // The enemy will move directly towards the ship
            if probability < game_settings.ship_visibility && *enemy_type == EnemyType::Ship {
                let (ship_x, ship_y) = game_settings.ship.get_position();

                let (diff_x, diff_y) = (ship_x - pos.0, ship_y - pos.1);

                let length = (diff_x.powi(2) + diff_y.powi(2)).sqrt();

                let (norm_dx, norm_dy) = (diff_x / length, diff_y / length);

                dx = norm_dx * game_settings.enemy_speed_multiplier;
                dy = norm_dy * game_settings.enemy_speed_multiplier;
            }

            if dx.abs() >= game_settings.enemy_min_speed
                && dy.abs() >= game_settings.enemy_min_speed
            {
                return (dx, dy);
            }
        }
    }

    /// Applies damage to the enemy.
    ///
    /// The enemy will split into smaller enemies if it is bigger than the reproduce limit.
    /// If the enemy is dead the method will return an empty vec.
    pub fn take_damage(
        &self,
        game_settings: &GameSettings,
        damage: i32,
        generator: SequenceGenerator,
    ) -> Vec<Self> {
        if self.health <= 0 {
            return vec![];
        }
        if self.radius <= self.reproduce_limit {
            let mut new_enemy = self.clone();
            new_enemy.health -= damage;
            return vec![new_enemy];
        }
        let mut rng = thread_rng();
        let mut children = vec![];
        for _ in 0..rng.gen_range(2..4) {
            let mut child = self.clone();
            child.id = generator.next_id();
            child.radius /= 2.0;
            child.health /= 2;
            child.reproduce_limit /= 2.0;
            // Make sure the child spawns in the radius of the parent
            child.x += rng.gen_range(-self.radius / 2.0..self.radius / 2.0);
            child.y += rng.gen_range(-self.radius / 2.0..self.radius / 2.0);
            if self.enemy_type == EnemyType::Ship {
                child.enemy_type = EnemyType::Debris;
            }

            let (dx, dy) =
                Enemy::calculate_speed(&game_settings, (child.x, child.y), &child.enemy_type);
            child.dx = dx;
            child.dy = dy;

            children.push(child);
        }
        children
    }

    /// Returns the amount of coins the enemy drops when it dies.
    ///
    /// The amount of coins depends on the enemy type.
    /// The number gets multiplied by the coin multiplier in the game settings.
    pub fn get_coins(&self) -> i32 {
        match self.enemy_type {
            EnemyType::Asteroid => 1,
            EnemyType::Ship => 2,
            // Get rewards for cleaning up the space
            EnemyType::Debris => 4,
        }
    }
}

impl PartialEq for Enemy {
    fn eq(&self, other: &Self) -> bool {
        self.get_position().0 == other.get_position().0
            && self.get_position().1 == other.get_position().1
    }
}

impl Hash for Enemy {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let string = format!("{}{}{}{}", self.x, self.y, self.dx, self.dy);
        string.hash(state);
    }
}

impl Eq for Enemy {}

impl Positioned for Enemy {
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
        (self.radius * 2.0, self.radius * 2.0)
    }
}

impl Moves for Enemy {
    fn direction(&self) -> (f32, f32) {
        (self.dx as f32, self.dy as f32)
    }
}
