use std::collections::HashMap;

use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::game::{interactable::{Interactable, InteractionType}, item::{Item, ItemCatalog, ItemSetupSet, spawn_item}, machine::spawn_machine, spawner::spawn_item_spawner};

pub struct RoomPlugin;
impl Plugin for RoomPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            setup_rooms.after(ItemSetupSet),
            place_ground_walls_ceiling.after(setup_rooms)
        ));
        app.add_systems(Update, move_between_rooms);
    }
}

pub const ROOM_SIZE: Vec2 = vec2(1280.0, 720.0);
pub const FLOOR_WIDTH: f32 =  128.0;
pub const FLOOR_HEIGHT: f32 = 32.0;
pub const WALL_WIDTH: f32 =  128.0;
pub const CEILING_WIDTH: f32 =  128.0;


#[derive(Resource)]
pub struct RoomManager{
    pub current_room_pos: IVec2,
    pub rooms: HashMap<IVec2, Entity>
}

impl RoomManager{
    fn try_moving(&mut self, dir: IVec2){
        if self.rooms.contains_key(&(self.current_room_pos +dir)){
            self.current_room_pos += dir;
            println!("moved to {:?}", self.current_room_pos)
        }
    }
}

#[derive(Component)]
struct Room{
    locked: bool,
    colliders: Vec<RoomCollider>
}

enum RoomCollider{
    Ground,
    Ceiling,
    WallLeft,
    WallRight
}


fn setup_rooms(
    mut cmds: Commands,
    item_catalog: Res<ItemCatalog>
){
    let mut rooms = HashMap::<IVec2, Entity>::new();

    let kitchen_room = cmds.spawn((
        Room {locked: false, colliders: vec![RoomCollider::Ground, RoomCollider::Ceiling]},
        Transform::from_xyz(0.0, 0.0, -10.0),
        Sprite::from_color(Color::linear_rgb(0.3, 0.0, 0.0), ROOM_SIZE),
    )).id();
    rooms.insert(ivec2(0, 0), kitchen_room);
    
    let counter_room = cmds.spawn((
        Room {locked: false, colliders: vec![RoomCollider::Ground, RoomCollider::WallLeft, RoomCollider::Ceiling]},
        Transform::from_xyz( -ROOM_SIZE.x, 0.0, -10.0),
        Sprite::from_color(Color::linear_rgb(0.0, 0.3, 0.0), ROOM_SIZE),
    )).id();
    rooms.insert(ivec2(-1, 0), counter_room);

    let dispencer_room = cmds.spawn((
        Room {locked: false, colliders: vec![RoomCollider::Ground, RoomCollider::WallRight, RoomCollider::Ceiling]},
        Transform::from_xyz( ROOM_SIZE.x, 0.0, -10.0),
        Sprite::from_color(Color::linear_rgb(0.0, 0.3, 0.0), ROOM_SIZE),
    )).id();
    rooms.insert(ivec2(1, 0), dispencer_room);

    let item_spawner = spawn_item_spawner(
        &mut cmds, 
        vec2(ROOM_SIZE.x, ROOM_SIZE.y/3.0), 
        vec!["red".to_string(),"green".to_string(),"blue".to_string()], 
        vec2(0.0, -128.0)
    );

    spawn_machine(&mut cmds, vec2(ROOM_SIZE.x/3.0, 16.0));
    // spawn_item(&mut cmds, &item_catalog, vec2(ROOM_SIZE.x, 0.0), "yellow".to_string());

    cmds.insert_resource(RoomManager{
        current_room_pos: ivec2(0, 0),
        rooms: rooms
    });
}

fn place_ground_walls_ceiling(
    mut cmds: Commands,
    room_query: Query<(&Room, &Transform)>
){
    for (room, room_transform) in room_query.iter(){
        for room_collider_type in room.colliders.iter(){
            let (collider, offset) = match room_collider_type {
                RoomCollider::Ground => {
                    (
                        Collider::cuboid(ROOM_SIZE.x/2.0, FLOOR_WIDTH/2.0),
                        vec2(0.0, -ROOM_SIZE.y/2.0 +FLOOR_HEIGHT)
                    )
                }
                RoomCollider::Ceiling => {
                    (
                        Collider::cuboid(ROOM_SIZE.x/2.0, CEILING_WIDTH/2.0),
                        vec2(0.0, ROOM_SIZE.y/2.0 +CEILING_WIDTH/2.0)
                    )
                }
                RoomCollider::WallLeft => {
                    (
                        Collider::cuboid(WALL_WIDTH/2.0, ROOM_SIZE.y/2.0),
                        vec2(-ROOM_SIZE.x/2.0 -WALL_WIDTH/2.0, 0.0)
                    )
                }
                RoomCollider::WallRight => {
                    (
                        Collider::cuboid(WALL_WIDTH/2.0, ROOM_SIZE.y/2.0),
                        vec2(ROOM_SIZE.x/2.0 +WALL_WIDTH/2.0, 0.0)
                    )
                }
            };
            let pos = room_transform.translation.truncate() +offset;
            cmds.spawn((
                collider,
                Transform::from_xyz(pos.x, pos.y, 0.0)
            ));
        }
    }
}

fn move_between_rooms(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut room_manager: ResMut<RoomManager>
){
    if keyboard.just_pressed(KeyCode::KeyA){
        room_manager.try_moving(ivec2(-1, 0));
    }
    if keyboard.just_pressed(KeyCode::KeyD){
        room_manager.try_moving(ivec2(1, 0));
    }
    if keyboard.just_pressed(KeyCode::KeyW){
        room_manager.try_moving(ivec2(0, 1));
    }
    if keyboard.just_pressed(KeyCode::KeyS){
        room_manager.try_moving(ivec2(0, -1));
    }
}
