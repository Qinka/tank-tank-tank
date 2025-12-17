// 坦克大战 - 主程序

use bevy::prelude::*;
use tank_tank_tank::components::*;
use tank_tank_tank::constants::*;
use tank_tank_tank::resources::*;
use tank_tank_tank::states::GameState;
use tank_tank_tank::systems::*;

fn main() {
    App::new()
        // 窗口和默认插件
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: WINDOW_TITLE.to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        // 游戏状态
        .init_state::<GameState>()
        // 资源
        .init_resource::<GameScore>()
        .init_resource::<PlayerLives>()
        .init_resource::<CurrentWave>()
        .init_resource::<EnemySpawnTimer>()
        .init_resource::<MousePosition>()
        // 启动系统
        .add_systems(Startup, setup::setup_camera)
        // 主菜单状态系统
        .add_systems(OnEnter(GameState::MainMenu), (
            ui::setup_main_menu,
            cleanup_game_entities,
            reset_game_resources,
        ))
        .add_systems(Update, ui::main_menu_interaction.run_if(in_state(GameState::MainMenu)))
        .add_systems(OnExit(GameState::MainMenu), ui::cleanup_main_menu)
        // 游戏进行中状态系统
        .add_systems(OnEnter(GameState::Playing), (
            setup::spawn_player,
            setup::setup_boundaries,
            setup::setup_obstacles,
            setup::init_wave_system,
            ui::setup_hud,
        ))
        // 游戏进行中的系统
        .add_systems(Update, (
            // 输入处理
            player::update_mouse_position,
            // 移动和 AI（需要串行以避免 Transform 冲突）
            (
                player::player_movement,
                player::player_rotation,
            ).chain(),
            enemy::enemy_ai,
            // 碰撞检测（需要在移动后执行）
            (
                collision::tank_wall_collision,
                collision::tank_tank_collision,
                collision::tank_destructible_wall_collision,
            ).chain(),
            // 战斗系统
            player::player_shooting,
            enemy::spawn_enemies,
            (
                combat::move_bullets,
                combat::cleanup_bullets,
                combat::bullet_collision,
            ).chain(),
            combat::explosion_animation,
            combat::check_player_death,
            enemy::check_wave_completion,
            // UI 更新
            ui::update_hud,
            ui::check_game_over,
            ui::toggle_pause,
        ).run_if(in_state(GameState::Playing)))
        .add_systems(OnExit(GameState::Playing), cleanup_game_entities)
        // 暂停状态
        .add_systems(Update, ui::toggle_pause.run_if(in_state(GameState::Paused)))
        // 游戏结束状态系统
        .add_systems(OnEnter(GameState::GameOver), (
            ui::setup_game_over,
            cleanup_game_entities,
        ))
        .add_systems(Update, ui::game_over_interaction.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), ui::cleanup_game_over)
        .run();
}

/// 清理游戏实体
fn cleanup_game_entities(
    mut commands: Commands,
    entities: Query<Entity, Or<(
        With<Player>,
        With<Enemy>,
        With<Bullet>,
        With<Wall>,
        With<BoundaryWall>,
        With<DestructibleWall>,
        With<Explosion>,
        With<UIHealthBar>,
        With<UIScoreText>,
        With<UILivesText>,
        With<UIWaveText>,
    )>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// 重置游戏资源
fn reset_game_resources(
    mut score: ResMut<GameScore>,
    mut lives: ResMut<PlayerLives>,
    mut wave: ResMut<CurrentWave>,
    mut timer: ResMut<EnemySpawnTimer>,
) {
    score.score = 0;
    lives.lives = PLAYER_INITIAL_LIVES;
    wave.wave = 0;
    wave.enemies_spawned = 0;
    wave.enemies_to_spawn = 0;
    timer.timer.reset();
}
