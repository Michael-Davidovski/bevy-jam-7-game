use bevy::{camera, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::game::rooms::{ROOM_SIZE, RoomManager};

pub struct CameraPlugin;
impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, move_camera);
    }
}   


fn setup_camera(
    mut cmds: Commands,
){
    cmds.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn move_camera(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    room_manager: Res<RoomManager>,
    time: Res<Time>,
){
    let Ok( mut camera_transform) = camera_query.single_mut() else {return};

    let target_x = room_manager.current_room_pos.x as f32 * ROOM_SIZE.x;
    let target_y = room_manager.current_room_pos.y as f32 * ROOM_SIZE.y;
    let target_pos = Vec3::new(target_x, target_y, camera_transform.translation.z);

    camera_transform.translation = camera_transform.translation.lerp(target_pos, time.delta_secs() * 10.0);

}
