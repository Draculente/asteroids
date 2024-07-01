//! This module contains the representation of the game used by the api. This is used to save and load the game.

use serde::Deserialize;
use ts_rs::TS;

/// The representation of the game used by the api. This is used to save and load the game.
#[derive(TS, Deserialize)]
#[ts(export)]
pub struct GameRessource {
    pub id: Option<u32>,
    pub ended: bool,
    pub score: f64,
    pub coins: i32,
    pub lives: i32,
    pub enemy_spawn_timeout: f32,
    pub items: Vec<ItemLevelRessource>,
}

/// The representation of an item used by the api. This is used to save and load the game.
#[derive(TS, Deserialize)]
#[ts(export)]
pub struct ItemLevelRessource {
    pub level: u32,
    pub item: ItemRessource,
}

/// Part of the representation of an item used by the api. This is used to save and load the game.
#[derive(TS, Deserialize)]
#[ts(export)]
pub struct ItemRessource {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub price: u32,
}
