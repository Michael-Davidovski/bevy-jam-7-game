use bevy::prelude::*;
use crate::core::{message::*, AppState, GamePhase};

pub struct MessageHandlersPlugin;

impl Plugin for MessageHandlersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            handle_game_started,
            handle_game_paused,
            handle_game_resumed,
            handle_player_died,
            handle_level_completed,
        ));
    }
}

fn handle_game_started(
    mut cmds: Commands,
    mut reader: MessageReader<GameStarted>,
) {
    for _msg in reader.read() {
        cmds.set_state(AppState::InGame);
        cmds.set_state(GamePhase::Playing);
    }
}

fn handle_game_paused(
    mut cmds: Commands,
    mut reader: MessageReader<GamePaused>,
) {
    for _msg in reader.read() {
        cmds.set_state(GamePhase::Paused);
    }
}

fn handle_game_resumed(
    mut cmds: Commands,
    mut reader: MessageReader<GameResumed>,
) {
    for _msg in reader.read() {
        cmds.set_state(GamePhase::Playing);
    }
}

fn handle_player_died(
    mut cmds: Commands,
    mut reader: MessageReader<PlayerDied>,
) {
    for msg in reader.read() {
        println!("Player died: {}", msg.cause);
        cmds.set_state(GamePhase::LevelFailed);
    }
}

fn handle_level_completed(
    mut cmds: Commands,
    mut reader: MessageReader<LevelCompleted>,
) {
    for msg in reader.read() {
        println!("Level {} completed! Score: {}", msg.level_id, msg.score);
        cmds.set_state(GamePhase::LevelComplete);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        MessagesPlugin,
        CoreStatesPlugin,
    };

    #[test]
    fn test_game_started_changes_state() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins((CoreStatesPlugin, MessagesPlugin, MessageHandlersPlugin));
        
        // Initial state prüfen
        let state = app.world_mut().resource::<State<AppState>>();
        assert_eq!(**state, AppState::MainMenu);
        
        // Message senden
        fn send_message(mut writer: MessageWriter<GameStarted>) {
            writer.write(GameStarted);
        }
        app.add_systems(Update, send_message);
        app.update();
        
        // State sollte geändert sein
        let state = app.world_mut().resource::<State<AppState>>();
        assert_eq!(**state, AppState::InGame);
    }

    #[test]
    fn test_player_died_changes_to_failed() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins((CoreStatesPlugin, MessagesPlugin, MessageHandlersPlugin));
        
        // Erst ins Spiel wechseln
        app.world_mut().resource_mut::<NextState<AppState>>()
            .set(AppState::InGame);
        app.update();
        
        // Player died senden
        fn send_death(mut writer: MessageWriter<PlayerDied>) {
            writer.write(PlayerDied {
                cause: "Spike trap".to_string(),
            });
        }
        app.add_systems(Update, send_death);
        app.update();
        
        // GamePhase sollte LevelFailed sein
        let phase = app.world_mut().resource::<State<GamePhase>>();
        assert_eq!(**phase, GamePhase::LevelFailed);
    }
}
