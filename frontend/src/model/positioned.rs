//! This module contains traits for objects that have a position and dimensions.

use super::game_settings::field::Field;

/// A trait for objects that have a position and dimensions.
///
/// This trait is used to check for collisions between objects and to check if an object is in the field.
pub trait Positioned {
    /// The position of the object returns (x, y).
    fn get_position(&self) -> (f32, f32);
    /// Sets the position of the object.
    fn set_position(&mut self, x: f32, y: f32);
    /// The dimensions of the object returns (width, height).
    fn dimensions(&self) -> (f32, f32);
    /// Whether the object is round or not. This is used to calculate collisions more accurately. If this is true, the width is treated as a radius.
    fn is_round(&self) -> bool;

    /// Returns the width of the object.
    fn get_width(&self) -> f32 {
        let (width, _) = self.dimensions();
        width
    }

    /// Returns the height of the object.
    fn get_height(&self) -> f32 {
        let (_, height) = self.dimensions();
        height
    }

    /// Checks if the object is out of bounds of the field.
    fn is_out_of_bounds(&self, field: &Field) -> bool {
        let (x, y) = self.get_center();
        let (width, height) = self.dimensions();
        x + width < 0.0
            || x - width > field.width as f32
            || y + height < 0.0
            || y - height > field.height as f32
    }

    /// Returns the center of the object.
    fn get_center(&self) -> (f32, f32) {
        let (x, y) = self.get_position();
        (x, y)
    }

    /// Checks if the object collides with another object.
    fn is_collision(&self, other: &dyn Positioned) -> bool {
        let (x, y) = self.get_center();
        let (x2, y2) = other.get_center();
        let (width, height) = self.dimensions();
        let (width2, height2) = other.dimensions();
        let x_distance = (x - x2).abs();
        let y_distance = (y - y2).abs();

        if self.is_round() && other.is_round() {
            let radius = width / 2.0;
            let radius2 = width2 / 2.0;

            let distance = (x_distance * x_distance + y_distance * y_distance).sqrt();
            return distance < (radius + radius2) * 0.8;
        }

        x_distance < (width + width2) / 3.0 && y_distance < (height + height2) / 3.0
    }
}

/// A trait for objects that can move.
///
/// This trait is used to move objects in the game per frame.
pub trait Moves: Positioned {
    /// The direction the object moves in. Returns (dx, dy).
    fn direction(&self) -> (f32, f32);
    /// Moves the object one tick.
    ///
    /// Returns true if the object is still in the field.
    fn move_tick(&mut self, field: &Field) -> bool {
        let (x, y) = self.get_position();
        let (dx, dy) = self.direction();
        self.set_position(x + dx, y + dy);
        !self.is_out_of_bounds(field)
    }
}
