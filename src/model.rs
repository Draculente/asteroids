pub mod enemy;
pub mod error_message;
pub mod game;
pub mod game_ressources;
pub mod game_settings;
pub mod positioned;
pub mod projectile;
pub mod shop;

use shop::Item;
use wasm_bindgen::prelude::*;

use game::Game;
use game::GameAction;

use crate::game_ressources::GameRessource;
use crate::model::game_settings::field::Field;
use crate::model::game_settings::ship::Ship;

use crate::model::game::GameState;

use self::error_message::ErrorMessage;

/// The model of the game. It contains all the information about the game state.
#[wasm_bindgen]
pub struct GameModel {
    /// The game
    game: Game,
    /// The error message
    error_message: ErrorMessage,
}

#[wasm_bindgen]
impl GameModel {
    /// Creates a new game model
    pub fn new() -> Self {
        Self {
            game: Game::new(),
            error_message: ErrorMessage::new_empty(),
        }
    }

    /// Calculates the next frame of the game.
    pub fn tick(&mut self) {
        self.error_message.tick();
        self.take_game_action(GameAction::Tick)
    }

    /// Starts a new game.
    pub fn start_game(&mut self) {
        self.game = Game::new();
        self.resume_game();
    }

    /// Ends the game.
    pub fn end_game(&mut self) {
        self.take_game_action(GameAction::End);
    }

    /// Shoots a projectile.
    pub fn shoot(&mut self) {
        self.take_game_action(GameAction::Shoot);
    }

    /// Moves the ship in a direction.
    pub fn rotate_ship(&mut self, dx: f32, dy: f32) {
        self.take_game_action(GameAction::RotateShip(dx, dy))
    }

    /// Pauses the game.
    pub fn pause_game(&mut self) {
        self.take_game_action(GameAction::Pause)
    }

    /// Resumes the game.
    pub fn resume_game(&mut self) {
        self.take_game_action(GameAction::Start)
    }

    /// Buy an item.
    pub fn buy_item(&mut self, item_id: u32) {
        self.take_game_action(GameAction::BuyItem(item_id, false));
    }

    /// Internal function to handle potential errors returned by the game.
    fn take_game_action(&mut self, game_action: GameAction) {
        if let Err(e) = self.game.step(game_action) {
            self.error_message = e;
        }
    }

    /// Dismiss the error message.
    pub fn dismiss_error(&mut self) {
        self.error_message.dismiss();
    }

    /// Loads a game from a game ressource.
    pub fn load_game(&mut self, game_ressource: JsValue) {
        let game_ressource: GameRessource = serde_wasm_bindgen::from_value(game_ressource).unwrap();
        self.game = Game::load(game_ressource);
    }

    /// The id is optional. The id is determined by the api. This function is used to set the id after the game is saved to the api.
    pub fn set_id(&mut self, id: u32) {
        self.game.id = Some(id);
    }

    // Getter

    /// Returns the id of the game.
    pub fn get_id(&self) -> Option<u32> {
        self.game.id
    }

    /// Return the enemy spawn timeout. This is used to save the game.
    pub fn get_enemy_spawn_timeout(&self) -> f32 {
        self.game.game_settings.enemy_spawn_timeout
    }

    /// Return the current error message if there is one.
    pub fn get_error_message(&self) -> Option<String> {
        self.error_message.get_message()
    }

    /// Returns the current GameState
    pub fn get_game_state(&self) -> GameState {
        self.game.state
    }

    /// Returns the current score
    pub fn get_score(&self) -> f64 {
        self.game.score
    }

    /// Returns the ship
    pub fn get_ship(&self) -> Ship {
        self.game.game_settings.ship
    }

    /// Returns the enemies. This method is special, because `wasm_bindgen` can't handle vecs of structs. Therefore we need to convert the enemies to a js value. In the typescript we can simply cast the value to an array of enemies.
    pub fn get_enemies(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.game.enemies).unwrap()
    }

    /// Returns the projectiles. See `get_enemies` for more information.
    pub fn get_projectiles(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.game.projectiles).unwrap()
    }

    /// Returns the field.
    pub fn get_field(&self) -> Field {
        self.game.game_settings.field
    }

    /// Returns the coins.
    pub fn get_coins(&self) -> i32 {
        self.game.coins
    }

    /// Returns the number of lives left.
    pub fn get_lives(&self) -> i32 {
        self.game.lives
    }

    /// Returns the maximum number of lives.
    pub fn get_max_lives(&self) -> i32 {
        self.game.game_settings.max_lives
    }

    /// Return the percentage to which the time until the next shot has run out.
    pub fn get_shoot_refill_percentage(&self) -> i32 {
        100 - ((self.game.time_until_next_shot as f32
            / self.game.game_settings.shoot_timeout as f32) as f32
            * 100.0) as i32
    }

    /// Returns if the ship is currently invulnerable.
    pub fn is_invulnerable(&self) -> bool {
        self.game.is_invulnerable()
    }

    /// Returns the shop items. See `get_enemies` for more information.
    pub fn get_shop_items(&mut self) -> JsValue {
        serde_wasm_bindgen::to_value(
            &self
                .game
                .shop
                .items
                .iter_mut()
                .map(|e| e.to_js_value())
                .collect::<Vec<&Item>>(),
        )
        .unwrap()
    }
}
