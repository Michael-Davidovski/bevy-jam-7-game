use bevy::prelude::*;

use crate::core::{AppState, GamePhase};


pub struct MessagesPlugin;

impl Plugin for MessagesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<GameStarted>()
            .add_message::<GamePaused>()
            .add_message::<GameResumed>()
            .add_message::<LevelCompleted>()
            .add_message::<LevelRestart>()
            .add_message::<PlayerDied>()
            .add_message::<ScoreChanged>()
            .add_message::<PlaySound>();
    }
}



// Game-wide events
#[derive(Message)]
pub struct GameStarted;

#[derive(Message)]
pub struct GamePaused;

#[derive(Message)]
pub struct GameResumed;

#[derive(Message)]
pub struct LevelCompleted {
    pub level_id: usize,
    pub time: f32,
    pub score: u32,
}

#[derive(Message)]
pub struct LevelRestart;

#[derive(Message)]
pub struct PlayerDied {
    pub cause: String,
}

#[derive(Message)]
pub struct ScoreChanged(pub i32);

#[derive(Message)]
pub struct PlaySound {
    pub sound: SoundEffect,
    pub volume: f32,
}

pub enum SoundEffect {
    Jump,
    Collect,
    Die,
    Victory,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_plugin_registers_all_messages() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins(MessagesPlugin);
        
        app.update();
        // Plugin läuft ohne Fehler = alle Messages registriert
    }

    #[test]
    fn test_game_started_message() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins(MessagesPlugin);
        
        // System das Message schreibt
        fn write_system(mut writer: MessageWriter<GameStarted>) {
            writer.write(GameStarted);
        }
        
        // System das liest - muss als separate Variable sein
        let mut has_message = false;
        
        app.add_systems(Update, write_system);
        app.update();
        
        // Direkt auf Messages Resource zugreifen
        let messages = app.world_mut().resource::<Messages<GameStarted>>();
        let mut cursor = messages.get_cursor();
        has_message = cursor.read(messages).count() > 0;
        
        assert!(has_message);
    }

    #[test]
    fn test_level_completed_with_data() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins(MessagesPlugin);
        
        fn write_system(mut writer: MessageWriter<LevelCompleted>) {
            writer.write(LevelCompleted {
                level_id: 3,
                time: 45.5,
                score: 1000,
            });
        }
        
        app.add_systems(Update, write_system);
        app.update();
        
        let messages = app.world_mut().resource::<Messages<LevelCompleted>>();
        let mut cursor = messages.get_cursor();
        let collected: Vec<_> = cursor.read(messages).collect();
        
        assert_eq!(collected.len(), 1);
        assert_eq!(collected[0].level_id, 3);
        assert_eq!(collected[0].time, 45.5);
        assert_eq!(collected[0].score, 1000);
    }

    #[test]
    fn test_player_died_with_cause() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins(MessagesPlugin);
        
        fn write_system(mut writer: MessageWriter<PlayerDied>) {
            writer.write(PlayerDied {
                cause: "Fell into pit".to_string(),
            });
        }
        
        app.add_systems(Update, write_system);
        app.update();
        
        let messages = app.world_mut().resource::<Messages<PlayerDied>>();
        let mut cursor = messages.get_cursor();
        let collected: Vec<_> = cursor.read(messages).collect();
        
        assert_eq!(collected.len(), 1);
        assert_eq!(collected[0].cause, "Fell into pit");
    }

    #[test]
    fn test_score_changed_multiple() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins(MessagesPlugin);
        
        fn write_system(mut writer: MessageWriter<ScoreChanged>) {
            writer.write(ScoreChanged(100));
            writer.write(ScoreChanged(-50));
            writer.write(ScoreChanged(25));
        }
        
        app.add_systems(Update, write_system);
        app.update();
        
        let messages = app.world_mut().resource::<Messages<ScoreChanged>>();
        let mut cursor = messages.get_cursor();
        let collected: Vec<_> = cursor.read(messages).collect();
        
        assert_eq!(collected.len(), 3);
        assert_eq!(collected[0].0, 100);
        assert_eq!(collected[1].0, -50);
        assert_eq!(collected[2].0, 25);
    }

    #[test]
    fn test_play_sound_message() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins(MessagesPlugin);
        
        fn write_system(mut writer: MessageWriter<PlaySound>) {
            writer.write(PlaySound {
                sound: SoundEffect::Jump,
                volume: 0.8,
            });
        }
        
        app.add_systems(Update, write_system);
        app.update();
        
        let messages = app.world_mut().resource::<Messages<PlaySound>>();
        let mut cursor = messages.get_cursor();
        let collected: Vec<_> = cursor.read(messages).collect();
        
        assert_eq!(collected.len(), 1);
        assert_eq!(collected[0].volume, 0.8);
    }

    #[test]
    fn test_mixed_messages_in_one_frame() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins(MessagesPlugin);
        
        fn write_system(
            mut started: MessageWriter<GameStarted>,
            mut score: MessageWriter<ScoreChanged>,
            mut sound: MessageWriter<PlaySound>,
        ) {
            started.write(GameStarted);
            score.write(ScoreChanged(50));
            sound.write(PlaySound { 
                sound: SoundEffect::Victory, 
                volume: 1.0 
            });
        }
        
        app.add_systems(Update, write_system);
        app.update();
        
        // GameStarted prüfen
        let messages = app.world_mut().resource::<Messages<GameStarted>>();
        let mut cursor = messages.get_cursor();
        assert_eq!(cursor.read(messages).count(), 1);
        
        // ScoreChanged prüfen
        let messages = app.world_mut().resource::<Messages<ScoreChanged>>();
        let mut cursor = messages.get_cursor();
        assert_eq!(cursor.read(messages).count(), 1);
        
        // PlaySound prüfen
        let messages = app.world_mut().resource::<Messages<PlaySound>>();
        let mut cursor = messages.get_cursor();
        assert_eq!(cursor.read(messages).count(), 1);
    }

    #[test]
    fn test_all_game_state_messages() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins(MessagesPlugin);
        
        fn write_all(
            mut started: MessageWriter<GameStarted>,
            mut paused: MessageWriter<GamePaused>,
            mut resumed: MessageWriter<GameResumed>,
            mut restart: MessageWriter<LevelRestart>,
        ) {
            started.write(GameStarted);
            paused.write(GamePaused);
            resumed.write(GameResumed);
            restart.write(LevelRestart);
        }
        
        app.add_systems(Update, write_all);
        app.update();
        
        // Alle Messages prüfen
        let world = app.world_mut();
        
        let messages = world.resource::<Messages<GameStarted>>();
        assert_eq!(messages.get_cursor().read(messages).count(), 1);
        
        let messages = world.resource::<Messages<GamePaused>>();
        assert_eq!(messages.get_cursor().read(messages).count(), 1);
        
        let messages = world.resource::<Messages<GameResumed>>();
        assert_eq!(messages.get_cursor().read(messages).count(), 1);
        
        let messages = world.resource::<Messages<LevelRestart>>();
        assert_eq!(messages.get_cursor().read(messages).count(), 1);
    }
}
