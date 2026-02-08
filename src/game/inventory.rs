use bevy::prelude::*;

use crate::game::items::Item;

#[derive(Resource)]
pub struct Inventory{
    pub items: Vec<Item>
}
