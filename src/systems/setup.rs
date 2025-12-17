// 游戏设置和初始化系统

use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::resources::*;

/// 初始化摄像机
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// 生成玩家坦克
pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: PLAYER_COLOR,
                custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, Z_TANKS),
            ..default()
        },
        Player,
        Health::new(PLAYER_MAX_HEALTH),
        Velocity::new(0.0, 0.0),
        Collider::new(PLAYER_SIZE),
        ShootCooldown::new(SHOOT_COOLDOWN),
    ));
}

/// 创建地图边界
pub fn setup_boundaries(mut commands: Commands) {
    let wall_thickness = 20.0;
    
    // 上边界
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(Vec2::new(WINDOW_WIDTH, wall_thickness)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - wall_thickness / 2.0, Z_WALLS),
            ..default()
        },
        Wall,
        Collider::new(WINDOW_WIDTH.max(wall_thickness)),
    ));
    
    // 下边界
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(Vec2::new(WINDOW_WIDTH, wall_thickness)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 + wall_thickness / 2.0, Z_WALLS),
            ..default()
        },
        Wall,
        Collider::new(WINDOW_WIDTH.max(wall_thickness)),
    ));
    
    // 左边界
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(Vec2::new(wall_thickness, WINDOW_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(-WINDOW_WIDTH / 2.0 + wall_thickness / 2.0, 0.0, Z_WALLS),
            ..default()
        },
        Wall,
        Collider::new(WINDOW_HEIGHT.max(wall_thickness)),
    ));
    
    // 右边界
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(Vec2::new(wall_thickness, WINDOW_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(WINDOW_WIDTH / 2.0 - wall_thickness / 2.0, 0.0, Z_WALLS),
            ..default()
        },
        Wall,
        Collider::new(WINDOW_HEIGHT.max(wall_thickness)),
    ));
}

/// 创建一些障碍物
pub fn setup_obstacles(mut commands: Commands) {
    // 中央掩体
    let obstacle_positions = vec![
        Vec2::new(200.0, 150.0),
        Vec2::new(-200.0, 150.0),
        Vec2::new(200.0, -150.0),
        Vec2::new(-200.0, -150.0),
        Vec2::new(0.0, 200.0),
        Vec2::new(0.0, -200.0),
    ];
    
    for pos in obstacle_positions {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(Vec2::new(WALL_SIZE, WALL_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, Z_WALLS),
                ..default()
            },
            Wall,
            Collider::new(WALL_SIZE),
        ));
    }
    
    // 一些可破坏的墙
    let destructible_positions = vec![
        Vec2::new(100.0, 0.0),
        Vec2::new(-100.0, 0.0),
        Vec2::new(0.0, 100.0),
        Vec2::new(0.0, -100.0),
    ];
    
    for pos in destructible_positions {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: DESTRUCTIBLE_WALL_COLOR,
                    custom_size: Some(Vec2::new(WALL_SIZE, WALL_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, Z_WALLS),
                ..default()
            },
            DestructibleWall,
            Health::new(DESTRUCTIBLE_WALL_HEALTH),
            Collider::new(WALL_SIZE),
        ));
    }
}

/// 初始化波次系统
pub fn init_wave_system(mut wave: ResMut<CurrentWave>) {
    wave.wave = 1;
    wave.enemies_spawned = 0;
    wave.enemies_to_spawn = 5; // 第一波5个敌人
}
