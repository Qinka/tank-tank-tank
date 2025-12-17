// 敌人 AI 系统

use crate::components::*;
use crate::constants::*;
use crate::resources::*;
use bevy::prelude::*;
use rand::Rng;

/// 敌人生成系统
pub fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    mut wave: ResMut<CurrentWave>,
    enemies: Query<&Enemy>,
    textures: Res<TankTextures>,
) {
    timer.timer.tick(time.delta());

    let current_enemy_count = enemies.iter().count();

    if timer.timer.just_finished()
        && current_enemy_count < MAX_ENEMIES
        && wave.enemies_spawned < wave.enemies_to_spawn
    {
        spawn_enemy(&mut commands, &textures);
        wave.enemies_spawned += 1;
    }
}

/// 生成单个敌人
fn spawn_enemy(commands: &mut Commands, textures: &TankTextures) {
    let mut rng = rand::thread_rng();

    // 在地图边缘随机位置生成
    let spawn_positions = vec![
        Vec2::new(
            rng.gen_range(-MAP_HALF_WIDTH..MAP_HALF_WIDTH),
            MAP_HALF_HEIGHT - 50.0,
        ),
        Vec2::new(
            rng.gen_range(-MAP_HALF_WIDTH..MAP_HALF_WIDTH),
            -MAP_HALF_HEIGHT + 50.0,
        ),
        Vec2::new(
            MAP_HALF_WIDTH - 50.0,
            rng.gen_range(-MAP_HALF_HEIGHT..MAP_HALF_HEIGHT),
        ),
        Vec2::new(
            -MAP_HALF_WIDTH + 50.0,
            rng.gen_range(-MAP_HALF_HEIGHT..MAP_HALF_HEIGHT),
        ),
    ];

    let spawn_pos = spawn_positions[rng.gen_range(0..spawn_positions.len())];

    let mut sprite_bundle = SpriteBundle {
        sprite: Sprite {
            color: ENEMY_COLOR,
            custom_size: Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
            ..default()
        },
        transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, Z_TANKS),
        ..default()
    };
    
    // 如果有贴图，使用贴图；否则使用纯色
    if let Some(ref texture) = textures.enemy {
        sprite_bundle.texture = texture.clone();
        // 使用白色以显示原始贴图颜色
        sprite_bundle.sprite.color = Color::WHITE;
    }

    commands.spawn((
        sprite_bundle,
        Enemy,
        Health::new(ENEMY_MAX_HEALTH),
        Velocity::new(0.0, 0.0),
        Collider::new(ENEMY_SIZE),
        AIState::new(),
        PatrolTarget {
            target: Vec2::new(0.0, 0.0),
        },
    ));
}

/// 敌人 AI 行为系统
pub fn enemy_ai(
    time: Res<Time>,
    mut commands: Commands,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<
        (
            Entity,
            &mut Transform,
            &mut Velocity,
            &mut AIState,
            &mut PatrolTarget,
        ),
        (With<Enemy>, Without<Player>),
    >,
) {
    let player_result = player_query.get_single();

    for (_entity, mut transform, mut velocity, mut ai_state, mut patrol) in enemy_query.iter_mut() {
        let enemy_pos = Vec2::new(transform.translation.x, transform.translation.y);

        // 如果玩家存在，检测玩家
        if let Ok(player_transform) = player_result {
            let player_pos = Vec2::new(
                player_transform.translation.x,
                player_transform.translation.y,
            );
            let distance_to_player = enemy_pos.distance(player_pos);

            // 检测到玩家
            if distance_to_player < AI_DETECTION_RANGE {
                if distance_to_player < AI_SHOOT_RANGE {
                    ai_state.mode = AIMode::Attack;
                } else {
                    ai_state.mode = AIMode::Chase;
                }

                // 追击或攻击模式：移动向玩家
                let direction = (player_pos - enemy_pos).normalize_or_zero();
                velocity.x = direction.x * ENEMY_SPEED;
                velocity.y = direction.y * ENEMY_SPEED;

                // 旋转面向玩家
                let angle = direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2;
                transform.rotation = Quat::from_rotation_z(angle);

                // 攻击模式：射击
                if ai_state.mode == AIMode::Attack {
                    ai_state.last_shot += time.delta_seconds();
                    if ai_state.last_shot >= AI_SHOOT_COOLDOWN {
                        ai_state.last_shot = 0.0;
                        shoot_at_player(&mut commands, &transform, player_pos, enemy_pos);
                    }
                }
            } else {
                // 巡逻模式
                ai_state.mode = AIMode::Patrol;
                patrol_behavior(&mut transform, &mut velocity, &mut patrol, enemy_pos);
            }
        } else {
            // 没有玩家时巡逻
            ai_state.mode = AIMode::Patrol;
            patrol_behavior(&mut transform, &mut velocity, &mut patrol, enemy_pos);
        }

        // 应用移动
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();

        // 限制在地图边界内
        transform.translation.x = transform
            .translation
            .x
            .clamp(-MAP_HALF_WIDTH, MAP_HALF_WIDTH);
        transform.translation.y = transform
            .translation
            .y
            .clamp(-MAP_HALF_HEIGHT, MAP_HALF_HEIGHT);
    }
}

/// 巡逻行为
fn patrol_behavior(
    transform: &mut Transform,
    velocity: &mut Velocity,
    patrol: &mut PatrolTarget,
    enemy_pos: Vec2,
) {
    let distance_to_target = enemy_pos.distance(patrol.target);

    // 到达巡逻点，选择新目标
    if distance_to_target < 50.0 {
        let mut rng = rand::thread_rng();
        patrol.target = Vec2::new(
            rng.gen_range(-MAP_HALF_WIDTH + 100.0..MAP_HALF_WIDTH - 100.0),
            rng.gen_range(-MAP_HALF_HEIGHT + 100.0..MAP_HALF_HEIGHT - 100.0),
        );
    }

    let direction = (patrol.target - enemy_pos).normalize_or_zero();
    velocity.x = direction.x * ENEMY_SPEED * 0.5; // 巡逻时速度较慢
    velocity.y = direction.y * ENEMY_SPEED * 0.5;

    // 旋转面向巡逻目标
    if direction.length() > 0.01 {
        let angle = direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2;
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

/// 向玩家射击
fn shoot_at_player(
    commands: &mut Commands,
    transform: &Transform,
    player_pos: Vec2,
    enemy_pos: Vec2,
) {
    let direction = (player_pos - enemy_pos).normalize();

    // 在敌人前方生成子弹
    let rotation = transform.rotation;
    let forward = rotation * Vec3::Y;
    let spawn_offset = forward * (ENEMY_SIZE / 2.0 + BULLET_SIZE);
    let bullet_pos = transform.translation + spawn_offset;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.3, 0.0),
                custom_size: Some(Vec2::new(BULLET_SIZE, BULLET_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(bullet_pos),
            ..default()
        },
        Bullet {
            owner: BulletOwner::Enemy,
        },
        Velocity::new(
            direction.x * BULLET_SPEED * 0.8,
            direction.y * BULLET_SPEED * 0.8,
        ),
        Collider::new(BULLET_SIZE),
    ));
}

/// 检查波次完成
pub fn check_wave_completion(mut wave: ResMut<CurrentWave>, enemies: Query<&Enemy>) {
    let enemy_count = enemies.iter().count();

    // 如果所有敌人都生成且都被消灭
    if wave.enemies_spawned >= wave.enemies_to_spawn && enemy_count == 0 {
        // 开始新一波
        wave.wave += 1;
        wave.enemies_spawned = 0;
        wave.enemies_to_spawn += WAVE_ENEMY_INCREMENT;
    }
}
