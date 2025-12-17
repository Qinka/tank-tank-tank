// 玩家控制系统

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::*;
use crate::constants::*;
use crate::resources::*;

/// 更新鼠标位置
pub fn update_mouse_position(
    mut mouse_pos: ResMut<MousePosition>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let window = q_window.single();
    let (camera, camera_transform) = q_camera.single();
    
    if let Some(cursor_pos) = window.cursor_position() {
        if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            mouse_pos.world_position = world_pos;
        }
    }
}

/// 玩家移动系统
pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        let mut direction = Vec2::ZERO;
        
        if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        
        velocity.x = direction.x * PLAYER_SPEED;
        velocity.y = direction.y * PLAYER_SPEED;
        
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
        
        // 限制玩家在地图边界内
        transform.translation.x = transform.translation.x.clamp(-MAP_HALF_WIDTH, MAP_HALF_WIDTH);
        transform.translation.y = transform.translation.y.clamp(-MAP_HALF_HEIGHT, MAP_HALF_HEIGHT);
    }
}

/// 玩家旋转（跟随鼠标）
pub fn player_rotation(
    mouse_pos: Res<MousePosition>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in query.iter_mut() {
        let player_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let direction = mouse_pos.world_position - player_pos;
        
        if direction.length() > 0.01 {
            let angle = direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2;
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

/// 玩家射击系统
pub fn player_shooting(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut query: Query<(&Transform, &mut ShootCooldown), With<Player>>,
) {
    for (transform, mut cooldown) in query.iter_mut() {
        cooldown.timer.tick(time.delta());
        
        let should_shoot = keyboard.pressed(KeyCode::Space) || mouse.pressed(MouseButton::Left);
        
        if should_shoot && cooldown.can_shoot() {
            cooldown.reset();
            
            // 计算子弹方向（坦克当前朝向）
            let rotation = transform.rotation;
            let direction = rotation * Vec3::Y; // 坦克面向上方
            
            // 在坦克前方生成子弹
            let spawn_offset = direction * (PLAYER_SIZE / 2.0 + BULLET_SIZE);
            let bullet_pos = transform.translation + spawn_offset;
            
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: BULLET_COLOR,
                        custom_size: Some(Vec2::new(BULLET_SIZE, BULLET_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_translation(bullet_pos),
                    ..default()
                },
                Bullet {
                    owner: BulletOwner::Player,
                },
                Velocity::new(direction.x * BULLET_SPEED, direction.y * BULLET_SPEED),
                Collider::new(BULLET_SIZE),
            ));
        }
    }
}
