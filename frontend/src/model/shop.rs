//! The shop module contains the shop struct and the item struct.
//!
//! The shop is used to buy items. The items change the game settings.

use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::error;

use super::{error_message::ErrorMessage, game_settings::GameSettings};

const SHOP_PRICE_MULTIPLIER: f32 = 2.1;

/// The item struct represents an item in the shop
///
/// The effect of items is simply to change values of the [game settings](GameSettings).
#[wasm_bindgen]
#[derive(Serialize)]
pub struct Item {
    /// The id of the item
    pub id: u32,
    /// The name of the item
    #[wasm_bindgen(readonly)]
    pub title: &'static str,
    /// The description of the item. This is currently shown on hover.
    #[wasm_bindgen(readonly)]
    pub description: &'static str,
    /// The internal price is the starting price of the item.
    pub internal_price: i32,
    /// The maximum level of the item
    pub max_level: u8,
    /// The current level of the item. It start with 0 which means the item is not bought.
    pub level: u8,
    /// The function which is called when the item is bought. It takes a mutable reference to the [game settings](GameSettings) and the current level of the item.
    #[wasm_bindgen(skip)]
    #[serde(skip)]
    pub apply_item: fn(&mut GameSettings, level: u8),
    #[wasm_bindgen(readonly)]
    /// DO NOT USE! This is only for the javascript part.
    pub price: i32,
}

impl Item {
    /// The price of the item at a specific level is calculated by the formula:
    /// ```text
    /// internal_price * level * SHOP_PRICE_MULTIPLIER
    /// ```
    fn get_price(&self, level: u8) -> i32 {
        ((self.internal_price * (level as i32)) as f32 * SHOP_PRICE_MULTIPLIER).round() as i32
    }

    /// Get the current price of the item. This is used to substract the coins from the player.
    pub fn get_current_price(&self) -> i32 {
        self.get_price(self.level)
    }

    /// Get the price of the next level of the item. This is used to display the price in the shop.
    pub fn get_next_level_price(&self) -> i32 {
        self.get_price((self.level + 1).min(self.max_level))
    }

    /// As we can't call functions on this item from javascript (wasm-bindgen can't handle vecs of structs), we need to convert the price to a js value.
    pub fn to_js_value(&mut self) -> &Self {
        self.price = self.get_next_level_price();
        self
    }
}

/// The shop struct represents the shop in the game
///
/// It contains a list of [items](Item).
#[derive(Serialize)]
pub struct Shop {
    /// The list of items in the shop
    pub items: Vec<Item>,
}

impl Shop {
    /// Create a new shop
    pub fn new() -> Self {
        let price = 0;
        Self {
            items: vec![
                Item {
                    id: 0,
                    title: "Energy Refresher",
                    description: "Increases the fire rate",
                    internal_price: 100,
                    price,
                    max_level: 6,
                    level: 0,
                    apply_item: |settings, _| {
                        settings.shoot_timeout -= 10;
                    },
                },
                Item {
                    id: 1,
                    title: "Warp Shot Amplifier",
                    description: "Increases the speed of the projectiles",
                    internal_price: 120,
                    price,
                    max_level: 5,
                    level: 0,
                    apply_item: |settings, _| {
                        settings.projectile_speed += 2.5;
                    },
                },
                Item {
                    id: 2,
                    title: "Optic Obfuscator",
                    description: "Decreases the chance of alien vessels to spot you",
                    internal_price: 200,
                    price,
                    max_level: 4,
                    level: 0,
                    apply_item: |settings, _| {
                        settings.ship_visibility -= 1.0;
                    },
                },
                Item {
                    id: 3,
                    title: "Treasure Enrichment Device",
                    description: "Increases the coin multiplier",
                    internal_price: 130,
                    price,
                    max_level: 5,
                    level: 0,
                    apply_item: |settings, _| {
                        settings.coin_multiplier += 1;
                    },
                },
                Item {
                    id: 4,
                    title: "Piercing Projectile Mod",
                    description: "Increases the amount of aliens you can hit with one shot",
                    internal_price: 300,
                    price,
                    max_level: 2,
                    level: 0,
                    apply_item: |settings, _| {
                        settings.projectile_hits += 1;
                    },
                },
                Item {
                    id: 5,
                    title: "Unbreakable Fortitude",
                    description: "Increases the time you stay invulnerable after getting hit",
                    internal_price: 150,
                    price,
                    max_level: 6,
                    level: 0,
                    apply_item: |settings, _| {
                        settings.invulnerability_time += 50;
                    },
                },
            ],
        }
    }

    /// Buy an item from the shop
    ///
    /// The level of the item will be increased by 1.
    /// The method will return an error if the item is already at max level or if the item is not found.
    /// Otherwise it will return the item.
    pub fn buy_item(
        &mut self,
        id: u32,
        coins: i32,
        ignore_coins: bool,
    ) -> Result<&Item, ErrorMessage> {
        let item = self.items.iter_mut().find(|item| item.id == id);
        match item {
            Some(item) => {
                if item.get_next_level_price() > coins && !ignore_coins {
                    return Err(error!("Not enough coins"));
                }
                if item.level < item.max_level {
                    item.level += 1;
                    Ok(item)
                } else {
                    Err(error!("Item already at max level"))
                }
            }
            None => Err(error!("Item not found")),
        }
    }
}
