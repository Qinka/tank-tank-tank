// 战斗系统

use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::resources::*;

/// 子弹移动系统
pub fn move_bullets(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Bullet>>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

/// 清理出界子弹
pub fn cleanup_bullets(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Bullet>>,
) {
    for (entity, transform) in query.iter() {
        let pos = transform.translation;
        if pos.x.abs() > WINDOW_WIDTH / 2.0 + 100.0 
            || pos.y.abs() > WINDOW_HEIGHT / 2.0 + 100.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// 子弹碰撞检测和伤害系统
pub fn bullet_collision(
    mut commands: Commands,
    mut score: ResMut<GameScore>,
    bullets: Query<(Entity, &Transform, &Collider, &Bullet)>,
    mut targets: Query<
        (Entity, &Transform, &Collider, &mut Health, Option<&Enemy>, Option<&Player>, Option<&DestructibleWall>),
        Without<Bullet>
    >,
) {
    for (bullet_entity, bullet_transform, bullet_collider, bullet) in bullets.iter() {
        let bullet_pos = Vec2::new(bullet_transform.translation.x, bullet_transform.translation.y);
        
        for (target_entity, target_transform, target_collider, mut health, enemy, player, wall) in targets.iter_mut() {
            let target_pos = Vec2::new(target_transform.translation.x, target_transform.translation.y);
            let distance = bullet_pos.distance(target_pos);
            let collision_distance = (bullet_collider.size + target_collider.size) / 2.0;
            
            if distance < collision_distance {
                // 检查是否应该造成伤害
                let should_damage = match bullet.owner {
                    BulletOwner::Player => enemy.is_some() || wall.is_some(),
                    BulletOwner::Enemy => player.is_some(),
                };
                
                if should_damage {
                    health.take_damage(BULLET_DAMAGE);
                    
                    // 删除子弹
                    commands.entity(bullet_entity).despawn();
                    
                    // 如果目标死亡，生成爆炸效果
                    if !health.is_alive() {
                        spawn_explosion(&mut commands, target_pos);
                        commands.entity(target_entity).despawn();
                        
                        // 如果击杀敌人，增加分数
                        if enemy.is_some() {
                            score.score += SCORE_PER_ENEMY;
                        }
                    }
                    
                    break;
                }
            }
        }
    }
}

/// 生成爆炸效果
fn spawn_explosion(commands: &mut Commands, position: Vec2) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.5, 0.0),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(position.x, position.y, Z_PARTICLES),
            ..default()
        },
        Explosion::new(0.5),
    ));
}

/// 爆炸动画系统
pub fn explosion_animation(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Explosion, &mut Sprite)>,
) {
    for (entity, mut explosion, mut sprite) in query.iter_mut() {
        explosion.timer.tick(time.delta());
        
        // 淡出效果
        let progress = explosion.timer.elapsed_secs() / explosion.timer.duration().as_secs_f32();
        let alpha = 1.0 - progress;
        sprite.color.set_alpha(alpha);
        
        if explosion.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// 检查玩家死亡
pub fn check_player_death(
    mut commands: Commands,
    mut lives: ResMut<PlayerLives>,
    query: Query<(Entity, &Health), With<Player>>,
) {
    for (entity, health) in query.iter() {
        if !health.is_alive() {
            commands.entity(entity).despawn();
            
            // 减少生命数
            lives.lives = lives.lives.saturating_sub(1);
            
            // 如果还有生命，重新生成玩家
            if lives.lives > 0 {
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
        }
    }
}
