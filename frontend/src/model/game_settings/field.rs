//! The field module contains the field struct.

use wasm_bindgen::prelude::wasm_bindgen;

/// A field is the area in which the game takes place.
///
/// It is defined by its width and height.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Field {
    /// The width of the field
    pub width: i32,
    /// The height of the field
    pub height: i32,
}

impl Field {
    /// Creates a new field with the given width and height
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
}
