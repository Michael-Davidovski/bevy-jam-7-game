use bevy::{platform::collections::HashMap, prelude::*};

use crate::game::{Interactable, Item};

struct ScenePlugin;
impl Plugin for ScenePlugin{
    fn build(&self, app: &mut App) {
        app.insert_resource(SceneResource {
            current_scene: "test_scene".to_string(),
            scenes: HashMap::default()
        });
    }
}

#[derive(Resource)]
pub struct SceneResource{
    pub current_scene: String,
    pub scenes: HashMap<String, SceneData>
}

pub struct SceneData{
    pub bg_image: Image,
    pub interactable: Vec<Interactable>,
}

