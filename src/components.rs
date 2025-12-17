// 游戏组件定义

use bevy::prelude::*;

/// 玩家坦克标记
#[derive(Component)]
pub struct Player;

/// 敌方坦克标记
#[derive(Component)]
pub struct Enemy;

/// 子弹组件
#[derive(Component)]
pub struct Bullet {
    pub owner: BulletOwner,
}

/// 子弹所有者
#[derive(Clone, Copy, PartialEq)]
pub enum BulletOwner {
    Player,
    Enemy,
}

/// 速度组件
#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// 生命值组件
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
        }
    }

    pub fn take_damage(&mut self, damage: f32) {
        self.current = (self.current - damage).max(0.0);
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }
}

/// 碰撞盒组件
#[derive(Component)]
pub struct Collider {
    pub size: f32,
}

impl Collider {
    pub fn new(size: f32) -> Self {
        Self { size }
    }
}

/// 墙壁标记
#[derive(Component)]
pub struct Wall;

/// 边界墙壁标记（不参与碰撞检测，仅用于视觉显示）
#[derive(Component)]
pub struct BoundaryWall;

/// 可破坏墙壁标记
#[derive(Component)]
pub struct DestructibleWall;

/// 射击冷却组件
#[derive(Component)]
pub struct ShootCooldown {
    pub timer: Timer,
}

impl ShootCooldown {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }

    pub fn can_shoot(&self) -> bool {
        self.timer.finished()
    }

    pub fn reset(&mut self) {
        self.timer.reset();
    }
}

/// AI 状态组件
#[derive(Component)]
pub struct AIState {
    pub mode: AIMode,
    pub last_shot: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum AIMode {
    Patrol,
    Chase,
    Attack,
}

impl AIState {
    pub fn new() -> Self {
        Self {
            mode: AIMode::Patrol,
            last_shot: 0.0,
        }
    }
}

/// 巡逻目标组件
#[derive(Component)]
pub struct PatrolTarget {
    pub target: Vec2,
}

/// 爆炸效果标记
#[derive(Component)]
pub struct Explosion {
    pub timer: Timer,
}

impl Explosion {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
}

/// UI 组件标记
#[derive(Component)]
pub struct UIHealthBar;

#[derive(Component)]
pub struct UIScoreText;

#[derive(Component)]
pub struct UILivesText;

#[derive(Component)]
pub struct UIWaveText;

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct LoadingScreenUI;
