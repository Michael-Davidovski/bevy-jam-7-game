use bevy::prelude::*;
use std::collections::HashMap;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameProgress>()
            .init_resource::<GameSettings>();
    }
}


#[derive(Resource, Default)]
pub struct GameProgress {
    pub current_level: usize,
    pub unlocked_levels: Vec<usize>,
    pub total_score: u32,
    pub total_deaths: usize,
    pub total_playtime: f32,
}

#[derive(Resource)]
pub struct GameSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub fullscreen: bool,
    pub vsync: bool,
    pub show_fps: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 0.7,
            sfx_volume: 0.8,
            fullscreen: false,
            vsync: true,
            show_fps: false,
        }
    }
}

#[derive(Resource)]
pub struct AssetCache {
    pub fonts: HashMap<String, Handle<Font>>,
    pub sounds: HashMap<String, Handle<AudioSource>>,
    pub textures: HashMap<String, Handle<Image>>,
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_progress_default() {
        let progress = GameProgress::default();
        assert_eq!(progress.current_level, 0);
        assert_eq!(progress.total_score, 0);
        assert_eq!(progress.total_deaths, 0);
        assert!(progress.unlocked_levels.is_empty());
    }

    #[test]
    fn test_game_settings_default_volumes() {
        let settings = GameSettings::default();
        assert_eq!(settings.master_volume, 1.0);
        assert_eq!(settings.music_volume, 0.7);
        assert_eq!(settings.sfx_volume, 0.8);
        assert!(!settings.fullscreen);
        assert!(settings.vsync);
    }

    #[test]
    fn test_resources_plugin_registers_resources() {
        let mut app = App::new();
        app.add_plugins(ResourcesPlugin);
        
        // Prüfen ob Resources registriert wurden
        assert!(app.world().get_resource::<GameProgress>().is_some());
        assert!(app.world().get_resource::<GameSettings>().is_some());
    }

    #[test]
    fn test_game_progress_mutation() {
        let mut progress = GameProgress::default();
        progress.current_level = 5;
        progress.total_score = 1000;
        progress.unlocked_levels.push(1);
        progress.unlocked_levels.push(2);
        
        assert_eq!(progress.current_level, 5);
        assert_eq!(progress.total_score, 1000);
        assert_eq!(progress.unlocked_levels.len(), 2);
    }
}
