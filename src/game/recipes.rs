use std::{arch::x86_64::_CMP_ORD_S, collections::{BTreeSet, btree_set}};

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_rapier2d::prelude::Collider;

use crate::game::item::{ItemData, ItemSetupSet};

pub struct RecepiePlugin;
impl Plugin for RecepiePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ingriedients_results_recepies.in_set(ItemSetupSet));
    }
} 



// #[derive(Resource, Default)]
// pub struct Ingriedients(pub HashMap<String, ItemData>);

// #[derive(Resource, Default)]
// pub struct Results(pub Vec<>);


#[derive(Resource, Default)]
pub struct Recipes(pub HashMap<BTreeSet<String>, String>);// ingriedients, result

impl Recipes {
    pub fn check_machine(&self, machine_ingredients: Vec<String>) -> Option<&String> {
        // Konvertierung in ein Set sortiert automatisch und entfernt Duplikate
        let set: BTreeSet<String> = machine_ingredients.into_iter().collect();
        self.0.get(&set)
    }
}

fn setup_ingriedients_results_recepies(
    mut cmds: Commands,
    asset_server: Res<AssetServer>
){
    // let mut ingriedients = HashMap::<String, ItemData>::new();
    // ingriedients.insert("red".to_string(), ItemData::new(Sprite::from_color(Color::linear_rgb(1.0, 0.0, 0.0), vec2(32.0, 32.0)), Collider::cuboid(16.0, 16.0)));
    // ingriedients.insert("green".to_string(), ItemData::new(Sprite::from_color(Color::linear_rgb(0.0, 1.0, 1.0), vec2(32.0, 32.0)), Collider::cuboid(16.0, 16.0)));
    // ingriedients.insert("blue".to_string(), ItemData::new(Sprite::from_color(Color::linear_rgb(0.0, 0.0, 1.0), vec2(32.0, 32.0)), Collider::cuboid(16.0, 16.0)));
    
    // let mut results = HashMap::<String, ItemData>::new();
    // results.insert("yellow".to_string(), ItemData::new(Sprite::from_color(Color::linear_rgb(1.0, 1.0, 0.0), vec2(32.0, 32.0)), Collider::cuboid(16.0, 16.0)));
    // results.insert("violet".to_string(), ItemData::new(Sprite::from_color(Color::linear_rgb(1.0, 0.0, 1.0), vec2(32.0, 32.0)), Collider::cuboid(16.0, 16.0)));
    // results.insert("turkeu".to_string(), ItemData::new(Sprite::from_color(Color::linear_rgb(0.0, 1.0, 1.0), vec2(32.0, 32.0)), Collider::cuboid(16.0, 16.0)));
    // results.insert("white".to_string(), ItemData::new(Sprite::from_color(Color::linear_rgb(1.0, 1.0, 1.0), vec2(32.0, 32.0)), Collider::cuboid(16.0, 16.0)));

    let mut recipes = HashMap::<BTreeSet<String>, String>::new();
    recipes.insert(vec!["red".to_string(), "green".to_string()].into_iter().collect(), "yellow".to_string());
    recipes.insert(vec!["red".to_string(), "blue".to_string()].into_iter().collect(), "violet".to_string());
    recipes.insert(vec!["green".to_string(), "blue".to_string()].into_iter().collect(), "turkeu".to_string());
    recipes.insert(vec!["red".to_string(), "green".to_string(), "blue".to_string()].into_iter().collect(), "white".to_string());

    // cmds.insert_resource(Ingriedients(ingriedients));
    // cmds.insert_resource(Results(results));
    cmds.insert_resource(Recipes(recipes));
}
