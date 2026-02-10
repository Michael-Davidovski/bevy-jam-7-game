use bevy::{platform::collections::HashMap, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::game::interactable::{Interactable, InteractionType};

pub struct ItemPlugin;
impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Startup, 
        ItemSetupSet);
        app.add_systems(Startup, setup_item_catalog.in_set(ItemSetupSet));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemSetupSet;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Item {
    pub name: String,
}

pub struct ItemData {
    pub sprite: Sprite,
    pub collider: Collider
}
impl ItemData{
    pub fn new(sprite: Sprite, collider: Collider) -> Self{
        ItemData {sprite, collider}
    }
    pub fn from_img_asset(img: Handle<Image>) -> Self{
        ItemData {
            sprite: Sprite::from_image(img),
            collider: Collider::cuboid(16.0, 16.0),
        }
    }
}
#[derive(Resource, Default)]
pub struct ItemCatalog(pub HashMap<String, ItemData>);


fn setup_item_catalog(
    mut cmds: Commands,
    asset_server: Res<AssetServer>
){
    let mut items = HashMap::<String, ItemData>::new();
    items.insert("trash".to_string(), ItemData::new(Sprite::from_color(Color::linear_rgb(0.0, 0.0, 0.0), vec2(32.0, 32.0)), Collider::cuboid(16.0, 16.0)));
    items.insert("red".to_string(), ItemData::new(Sprite::from_color(Color::linear_rgb(1.0, 0.0, 0.0), vec2(32.0, 32.0)), Collider::cuboid(16.0, 16.0)));
    items.insert("green".to_string(), ItemData::new(Sprite::from_color(Color::linear_rgb(0.0, 1.0, 1.0), vec2(32.0, 32.0)), Collider::cuboid(16.0, 16.0)));
    items.insert("blue".to_string(), ItemData::new(Sprite::from_color(Color::linear_rgb(0.0, 0.0, 1.0), vec2(32.0, 32.0)), Collider::cuboid(16.0, 16.0)));
    items.insert("yellow".to_string(), ItemData::new(Sprite::from_color(Color::linear_rgb(1.0, 1.0, 0.0), vec2(32.0, 32.0)), Collider::cuboid(16.0, 16.0)));
    items.insert("violet".to_string(), ItemData::new(Sprite::from_color(Color::linear_rgb(1.0, 0.0, 1.0), vec2(32.0, 32.0)), Collider::cuboid(16.0, 16.0)));
    items.insert("turkeu".to_string(), ItemData::new(Sprite::from_color(Color::linear_rgb(0.0, 1.0, 1.0), vec2(32.0, 32.0)), Collider::cuboid(16.0, 16.0)));
    items.insert("white".to_string(), ItemData::new(Sprite::from_color(Color::linear_rgb(1.0, 1.0, 1.0), vec2(32.0, 32.0)), Collider::cuboid(16.0, 16.0)));

    cmds.insert_resource(ItemCatalog(items));
}


pub fn spawn_item(cmds: &mut Commands, item_catalog: &Res<ItemCatalog>, pos: Vec2, name: String) {
    if let Some(item_data) = item_catalog.0.get(&name){
        cmds.spawn((
            Item { name },
            // Interactable { interaction_type: InteractionType::Item },
            item_data.sprite.clone(),
            Transform::from_xyz(pos.x, pos.y, 1.0),
            RigidBody::Dynamic,
            item_data.collider.clone(),
        ));
    }
}




