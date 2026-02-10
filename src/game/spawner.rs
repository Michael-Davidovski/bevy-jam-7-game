use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;
use rand::seq::IndexedRandom;

use crate::game::item::{self, ItemCatalog, spawn_item};

pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin{
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnRandomItem>();
        app.add_systems(Update, handle_spawning);
    }
}

#[derive(Component)]
struct ItemSpawner{
    item_list: Vec<String>,
    offset: Vec2,
}

#[derive(Message)]
struct SpawnRandomItem(Entity);


pub fn spawn_item_spawner(
    cmds: &mut Commands,
    pos: Vec2,
    item_list: Vec<String>,
    offset: Vec2
) -> Entity{
    cmds.spawn((
        ItemSpawner {
            item_list,
            offset
        },
        Sprite::from_color(Color::linear_rgb(0.5, 0.5, 0.0), vec2(64.0, 256.0)),
        Transform::from_xyz(pos.x, pos.y, 0.0),
        Collider::cuboid(32.0, 128.0)
    )).id()
}


fn handle_spawning(
    mut cmds: Commands,
    item_spawner_q: Query<(&ItemSpawner, &Transform)>,
    mut spawn_random_item_msgs: MessageReader<SpawnRandomItem>,
    item_catalog: Res<ItemCatalog>,
){
    for spawn_random_item_msg in spawn_random_item_msgs.read().into_iter(){
        if let Ok((item_spawner, item_spawner_transform)) = item_spawner_q.get(spawn_random_item_msg.0){
            let mut rng = rand::rng();

            let Some(random_item) = item_spawner.item_list.choose(&mut rng) else {continue;};
            let pos = item_spawner_transform.translation.truncate() +item_spawner.offset;
            spawn_item(&mut cmds, &item_catalog, pos, random_item.clone());
        }
    }
}
