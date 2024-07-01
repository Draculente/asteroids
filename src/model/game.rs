//! The heart of the game is the game struct. It contains all the information about the game state.
//!
//! The central game loop is implemented here using a state machine.

use unique_id::{sequence::SequenceGenerator, Generator};
use wasm_bindgen::prelude::*;

use crate::game_ressources::GameRessource;

use super::{
    enemy::Enemy,
    error_message::ErrorMessage,
    game_settings::GameSettings,
    positioned::{Moves, Positioned},
    projectile::Projectile,
    shop::Shop,
};

/// The model of the game. It contains all the information about the game state.
pub struct Game {
    /// The id used to save the game
    pub id: Option<u32>,
    /// The id generator is used to generate unique ids for game objects
    id_generator: SequenceGenerator,
    /// The shop contains all the items that can be bought
    pub shop: Shop,
    /// The game settings contain all the settings of the game
    pub game_settings: GameSettings,
    /// The score of the game (the number of points on the top right)
    pub score: f64,
    /// The coins of the game (the number of coins on the top left). Coins can be used to buy items in the shop.
    pub coins: i32,
    /// The lives the player has left. If this is 0, the game is over. Starts with 3.
    pub lives: i32,
    /// The time until the next shot can be fired. This time is configured by the [game settings](GameSettings).
    pub time_until_next_shot: i32,
    /// The time until the ship is vulnerable again. This time is configured by the [game settings](GameSettings).
    pub time_until_vulnerable: i32,
    /// The time until the next enemy is spawned. This time decreases with the score.
    pub time_until_enemy_spawn: i32,
    /// The state of the game.
    pub state: GameState,
    /// The enemies in the game
    pub enemies: Vec<Enemy>,
    /// The projectiles in the game
    pub projectiles: Vec<Projectile>,
}

/// The state of the game
#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Running,
    Paused,
    /// The game is not running. This is the initial state and the state when the game is over.
    NotRunning,
}

/// The actions that can be performed on the game
pub enum GameAction {
    /// A tick is a frame of the game. This is used to calculate the next frame.
    Tick,
    /// The ship shoots a projectile
    Shoot,
    /// The ship rotates. The parameters are the x and y coordinates of the mouse.
    RotateShip(f32, f32),
    /// The game is started
    Start,
    /// The game is paused
    Pause,
    /// An item is bought. The parameters are the id of the item and if the coins should be ignored (e.g. for loading an old game state, the coins should not get substracted twice).
    BuyItem(u32, bool),
    /// The game is ended
    End,
}

impl Game {
    /// Creates a new game
    pub fn new() -> Self {
        let game_settings = GameSettings::new();
        Self {
            id: None,
            shop: Shop::new(),
            id_generator: SequenceGenerator::default(),
            lives: game_settings.max_lives.clone(),
            game_settings,
            score: 0.0,
            coins: 0,
            time_until_vulnerable: 0,
            time_until_next_shot: 0,
            time_until_enemy_spawn: 0,
            state: GameState::NotRunning,
            enemies: vec![],
            projectiles: vec![],
        }
    }

    /// Creates a new game from a game ressource (used to load an old game state from the api)
    pub fn load(game_ressource: GameRessource) -> Self {
        let mut game = Self::new();
        game.state = GameState::Paused;
        game.id = game_ressource.id;
        game.score = game_ressource.score;
        game.coins = game_ressource.coins;
        game.lives = game_ressource.lives;
        game.game_settings.enemy_spawn_timeout = game_ressource.enemy_spawn_timeout;
        game_ressource.items.into_iter().for_each(|item_level| {
            for _ in 0..item_level.level {
                game.step(GameAction::BuyItem(item_level.item.id, true))
                    .expect("Error loading item");
            }
        });
        game
    }

    /// Calculates a new frame of the game
    ///
    /// Implemented using a state machine.
    pub fn step(&mut self, action: GameAction) -> Result<(), ErrorMessage> {
        match (&self.state, action) {
            (GameState::Running, GameAction::Tick) => {
                // increase score
                self.score += self.game_settings.score_increase;

                // decrease shoot timeout if necessary
                if self.time_until_next_shot > 0 {
                    self.time_until_next_shot -= 1;
                }

                // decrease invulnerability time if necessary
                if self.time_until_vulnerable > 0 {
                    self.time_until_vulnerable -= 1;
                }

                // check for collisions

                // check for collisions between projectiles and enemies
                let new_enemies = self.collisions_enemy_projectile();
                self.enemies = new_enemies;

                // check for collisions between enemies and ship
                if let Some(e) = self.collision_enemy_ship() {
                    if !self.is_invulnerable() {
                        self.time_until_vulnerable = self.game_settings.invulnerability_time;
                        self.lives -= 1;
                        if self.lives == 0 {
                            self.state = GameState::NotRunning;
                        }
                    }

                    // remove enemy
                    self.enemies
                        .retain(|enemy| enemy.get_position() != e.get_position());
                }

                // increase the enemy spawn rate every 100 points
                if self.score.round() as i32 % 100 == 0
                    && self.game_settings.enemy_spawn_timeout > 1.0
                {
                    self.game_settings.enemy_spawn_timeout -=
                        self.game_settings.score_increase as f32;
                }

                // spawn new enemies
                if self.time_until_enemy_spawn <= 0 {
                    self.time_until_enemy_spawn = self.game_settings.enemy_spawn_timeout as i32;
                    self.enemies.append(&mut self.spawn_enemies());
                } else {
                    self.time_until_enemy_spawn -= 1;
                }

                let field = self.game_settings.field.clone();

                // Move projectiles and remove out of bounds projectiles and projectiles that have no hits left
                self.projectiles
                    .retain_mut(|p| p.move_tick(&field) && p.hits > 0);

                // Move enemies and remove out of bounds enemies
                self.enemies.retain_mut(|e| e.move_tick(&field));
            }
            (GameState::Running, GameAction::Shoot) => {
                if self.time_until_next_shot <= 0 {
                    self.time_until_next_shot = self.game_settings.shoot_timeout;
                    self.projectiles.push(Projectile::new(self));
                }
            }
            (GameState::Running, GameAction::RotateShip(dx, dy)) => {
                self.game_settings.ship.dx = dx;
                self.game_settings.ship.dy = dy;
            }
            (GameState::Paused, GameAction::Start) => {
                self.state = GameState::Running;
            }
            (GameState::Running, GameAction::Pause) => {
                self.state = GameState::Paused;
            }
            (GameState::NotRunning, GameAction::Start) => {
                self.state = GameState::Running;
            }
            (
                GameState::Running | GameState::Paused,
                GameAction::BuyItem(item_id, ignore_coins),
            ) => {
                let item = self.shop.buy_item(item_id, self.coins, ignore_coins);

                match item {
                    Ok(item) => {
                        if !ignore_coins {
                            self.coins -= item.get_current_price();
                        }
                        (item.apply_item)(&mut self.game_settings, item.level);
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            (GameState::Running | GameState::Paused, GameAction::End) => {
                self.lives = 0;
                self.state = GameState::NotRunning;
            }

            (GameState::Paused, GameAction::RotateShip(_, _)) => (),
            (GameState::Paused, GameAction::Shoot) => (),
            (GameState::Paused, GameAction::Tick) => (),
            (GameState::NotRunning, GameAction::Tick) => (),
            (GameState::NotRunning, GameAction::Shoot) => (),
            (GameState::NotRunning, GameAction::RotateShip(_, _)) => (),
            (GameState::Running, GameAction::Start) => (),
            (GameState::Paused, GameAction::Pause) => (),
            (GameState::NotRunning, GameAction::Pause) => (),
            (GameState::NotRunning, GameAction::BuyItem(_, _)) => (),
            (GameState::NotRunning, GameAction::End) => (),
        };
        Ok(())
    }

    /// Returns the vec of enemies to be spawned in the next frame.
    ///
    /// The higher the score, the more enemies are spawned.
    fn spawn_enemies(&self) -> Vec<Enemy> {
        let mut new_enemies = vec![];

        let enemy = Enemy::new(&self.game_settings, self.score, self.get_next_id());
        new_enemies.push(enemy);

        new_enemies
    }

    /// Checks for collisions between enemies and projectiles.  
    ///
    /// Returns the enemies that are still alive.
    ///
    /// TODO: This is a very naive implementation. It checks every projectile against every enemy.
    /// This is very inefficient.
    fn collisions_enemy_projectile(&mut self) -> Vec<Enemy> {
        let mut new_enemies: Vec<Enemy> = vec![];
        let mut hits = 0;

        if self.projectiles.is_empty() {
            return self.enemies.clone();
        }

        for enemy in &self.enemies {
            let projectile = self.projectiles.iter_mut().find(|p| enemy.is_collision(*p));

            if let Some(p) = projectile {
                new_enemies.extend(enemy.take_damage(
                    &self.game_settings,
                    p.damage,
                    self.id_generator.clone(),
                ));
                p.hits -= 1;
                hits += enemy.get_coins();
            } else {
                new_enemies.push(enemy.clone());
            }
        }

        // increase coins
        self.coins += hits * self.game_settings.coin_multiplier;

        new_enemies
    }

    /// Checks for collisions between enemies and the ship.
    fn collision_enemy_ship(&self) -> Option<Enemy> {
        self.enemies
            .iter()
            .find(|e| self.game_settings.ship.is_collision(*e))
            .cloned()
    }

    /// Returns the next id for a game object.
    pub fn get_next_id(&self) -> i64 {
        self.id_generator.next_id()
    }

    /// Returns true if the ship is invulnerable (e.g. after getting hit by an enemy).
    pub fn is_invulnerable(&self) -> bool {
        !(self.time_until_vulnerable <= 0)
    }
}
