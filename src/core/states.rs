use bevy::prelude::*;

pub struct CoreStatesPlugin;

impl Plugin for CoreStatesPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_state::<AppState>()
        .init_state::<GamePhase>()
        ;
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    Loading,
    Splash,
    MainMenu,
    InGame,
    #[default]
    LevelEditor,
}

#[derive(SubStates, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[source(AppState = AppState::InGame)]
pub enum GamePhase {
    #[default]
    Playing,
    Paused,
    LevelComplete,
    LevelFailed,
}

// Am Ende von states.rs

#[cfg(test)]
mod tests {
    use bevy::state::app::StatesPlugin;

    use super::*;

    #[test]
    fn test_app_state_default() {
        let state = AppState::default();
        assert_eq!(state, AppState::Loading);
    }

    #[test]
    fn test_game_phase_default() {
        let phase = GamePhase::default();
        assert_eq!(phase, GamePhase::Playing);
    }

    #[test]
    fn test_core_states_plugin_initializes_states() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin)
           .add_plugins(CoreStatesPlugin);
        
        // Prüfen ob States initialisiert wurden
        let app_state = app.world().get_resource::<State<AppState>>();
        assert!(app_state.is_some());
        assert_eq!(*app_state.unwrap(), AppState::MainMenu);
    }

    #[test]
    fn test_state_transitions() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin)
           .add_plugins(CoreStatesPlugin);
        
        // State wechseln
        app.world_mut().resource_mut::<NextState<AppState>>()
            .set(AppState::InGame);
        app.update();
        
        let current_state = app.world().get_resource::<State<AppState>>().unwrap();
        assert_eq!(**current_state, AppState::InGame);
    }

    #[test]
    fn test_all_app_states_are_different() {
        // Stellt sicher, dass alle States unterschiedlich sind
        assert_ne!(AppState::Loading, AppState::MainMenu);
        assert_ne!(AppState::MainMenu, AppState::InGame);
        assert_ne!(AppState::InGame, AppState::Splash);
    }
}
