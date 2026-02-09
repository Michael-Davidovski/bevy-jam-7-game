
use bevy::prelude::*;

struct StatesPlugin;
impl Plugin for StatesPlugin{
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.init_state::<GamePhase>();
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    Splash,
    MainMenu,
    #[default]
    InGame,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GamePhase {
    #[default]
    Playing,
    Paused,
}

