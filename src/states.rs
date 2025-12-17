// 游戏状态定义

use bevy::state::state::States;

/// 游戏主状态
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    GameOver,
}
