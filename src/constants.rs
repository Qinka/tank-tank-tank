// 游戏常量定义

use bevy::prelude::*;

// 窗口设置
pub const WINDOW_WIDTH: f32 = 1024.0;
pub const WINDOW_HEIGHT: f32 = 768.0;
pub const WINDOW_TITLE: &str = "坦克大战 - Tank Tank Tank";

// 玩家坦克属性
pub const PLAYER_SPEED: f32 = 200.0;
pub const PLAYER_ROTATION_SPEED: f32 = 3.0;
pub const PLAYER_SIZE: f32 = 40.0;
pub const PLAYER_MAX_HEALTH: f32 = 100.0;
pub const PLAYER_INITIAL_LIVES: u32 = 3;
pub const PLAYER_COLOR: Color = Color::srgb(0.2, 0.7, 0.2);

// 敌人坦克属性
pub const ENEMY_SPEED: f32 = 100.0;
pub const ENEMY_SIZE: f32 = 35.0;
pub const ENEMY_MAX_HEALTH: f32 = 50.0;
pub const ENEMY_COLOR: Color = Color::srgb(0.8, 0.2, 0.2);
pub const ENEMY_SPAWN_INTERVAL: f32 = 3.0;
pub const MAX_ENEMIES: usize = 10;

// 子弹属性
pub const BULLET_SPEED: f32 = 400.0;
pub const BULLET_SIZE: f32 = 8.0;
pub const BULLET_DAMAGE: f32 = 25.0;
pub const BULLET_COLOR: Color = Color::srgb(1.0, 1.0, 0.0);
pub const SHOOT_COOLDOWN: f32 = 0.5;

// 墙壁和障碍物
pub const WALL_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);
pub const DESTRUCTIBLE_WALL_COLOR: Color = Color::srgb(0.6, 0.4, 0.2);
pub const WALL_SIZE: f32 = 40.0;
pub const DESTRUCTIBLE_WALL_HEALTH: f32 = 50.0;

// 地图边界
pub const MAP_HALF_WIDTH: f32 = WINDOW_WIDTH / 2.0 - 20.0;
pub const MAP_HALF_HEIGHT: f32 = WINDOW_HEIGHT / 2.0 - 20.0;

// 游戏机制
pub const SCORE_PER_ENEMY: u32 = 100;
pub const WAVE_ENEMY_INCREMENT: usize = 2;

// AI 行为
pub const AI_DETECTION_RANGE: f32 = 300.0;
pub const AI_SHOOT_RANGE: f32 = 250.0;
pub const AI_SHOOT_COOLDOWN: f32 = 1.5;

// Z 层级
pub const Z_BACKGROUND: f32 = 0.0;
pub const Z_WALLS: f32 = 1.0;
pub const Z_TANKS: f32 = 2.0;
pub const Z_BULLETS: f32 = 3.0;
pub const Z_PARTICLES: f32 = 4.0;
pub const Z_UI: f32 = 10.0;
