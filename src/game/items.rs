use bevy::prelude::*;

use crate::game::ice_cream::IceCream;

pub struct Item{
    name: String,
    description: String,
    item_type: ItemType,
}

pub enum ItemType{
    IceCream(IceCream),
    Hand,

}

