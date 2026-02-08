use bevy::prelude::*;

pub struct CameraZoomPlugin;

impl Plugin for CameraZoomPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, handle_camera_zoom);
    }
}

#[derive(Component)]
pub struct CameraZoom {
    pub target_zoom: f32,
    pub current_zoom: f32,
    pub zoom_speed: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
}

impl Default for CameraZoom {
    fn default() -> Self {
        Self {
            target_zoom: 1.0,
            current_zoom: 1.0,
            zoom_speed: 5.0,
            min_zoom: 0.1,
            max_zoom: 10.0,
        }
    }
}

impl CameraZoom {
    pub fn new(initial_zoom: f32) -> Self {
        Self {
            target_zoom: initial_zoom,
            current_zoom: initial_zoom,
            ..default()
        }
    }

    pub fn zoom_in(&mut self, amount: f32) {
        self.target_zoom = (self.target_zoom - amount).max(self.min_zoom);
    }

    pub fn zoom_out(&mut self, amount: f32) {
        self.target_zoom = (self.target_zoom + amount).min(self.max_zoom);
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.target_zoom = zoom.clamp(self.min_zoom, self.max_zoom);
    }
}


fn handle_camera_zoom(
    mut camera_query: Query<(&mut Projection, &mut CameraZoom), 
    Or<(With<Camera2d>, With<Camera3d>)>>,
    time: Res<Time>,
) {

    for (mut projection, mut camera_zoom) in camera_query.iter_mut(){

        camera_zoom.current_zoom = camera_zoom.current_zoom.lerp(
            camera_zoom.target_zoom,
            (camera_zoom.zoom_speed * time.delta_secs()).min(1.0),
        );
        
        match *projection.into_inner() {
            Projection::Orthographic(ref mut orthographic) => {
                orthographic.scale = camera_zoom.current_zoom;
            }
            Projection::Perspective(ref mut persp) => {
                // For perspective, we adjust FOV (smaller FOV = more zoom)
                // Base FOV is typically around 45-60 degrees
                let base_fov = 50.0_f32.to_radians();
                persp.fov = base_fov / camera_zoom.current_zoom;
            }
            _ => (),
        }
    }

}

