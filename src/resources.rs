// 游戏资源定义

use bevy::prelude::*;

/// 游戏得分资源
#[derive(Resource, Default)]
pub struct GameScore {
    pub score: u32,
}

/// 玩家生命数资源
#[derive(Resource)]
pub struct PlayerLives {
    pub lives: u32,
}

impl Default for PlayerLives {
    fn default() -> Self {
        Self {
            lives: crate::constants::PLAYER_INITIAL_LIVES,
        }
    }
}

/// 当前波次资源
#[derive(Resource, Default)]
pub struct CurrentWave {
    pub wave: u32,
    pub enemies_spawned: usize,
    pub enemies_to_spawn: usize,
}

/// 敌人生成计时器
#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(
                crate::constants::ENEMY_SPAWN_INTERVAL,
                TimerMode::Repeating,
            ),
        }
    }
}

/// 鼠标位置资源
#[derive(Resource, Default)]
pub struct MousePosition {
    pub world_position: Vec2,
}

/// 游戏字体资源
#[derive(Resource)]
pub struct GameFont {
    pub handle: Handle<Font>,
}

/// 坦克贴图资源
#[derive(Resource)]
pub struct TankTextures {
    pub player: Option<Handle<Image>>,
    pub enemy: Option<Handle<Image>>,
}
