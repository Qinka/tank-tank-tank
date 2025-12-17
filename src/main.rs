// 坦克大战 - 主程序

use bevy::prelude::*;
use bevy::state::state::NextState;
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
        .add_systems(Startup, (setup::setup_camera, load_font))
        // 加载状态系统 - 等待资源加载完成
        .add_systems(OnEnter(GameState::Loading), setup_loading_screen)
        .add_systems(Update, check_font_loaded.run_if(in_state(GameState::Loading)))
        .add_systems(OnExit(GameState::Loading), cleanup_loading_screen)
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

/// 加载字体和贴图资源
fn load_font(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // 加载思源黑体字体
    let font_handle = asset_server.load("fonts/SourceHanSansCN-Regular.otf");
    commands.insert_resource(GameFont {
        handle: font_handle,
    });
    
    // 加载坦克贴图（如果文件不存在，将回退到纯色显示）
    let player_texture = Some(asset_server.load("textures/player_tank.png"));
    let enemy_texture = Some(asset_server.load("textures/enemy_tank.png"));
    
    commands.insert_resource(TankTextures {
        player: player_texture,
        enemy: enemy_texture,
    });
}

/// 设置加载界面
fn setup_loading_screen(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::srgb(0.1, 0.1, 0.15).into(),
            ..default()
        },
        LoadingScreenUI,
    )).with_children(|parent| {
        // 加载提示（使用简单的方块代替文字，因为字体还未加载）
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                ..default()
            },
            background_color: Color::srgb(0.3, 0.3, 0.3).into(),
            ..default()
        });
    });
}

/// 检查字体是否加载完成
fn check_font_loaded(
    mut next_state: ResMut<NextState<GameState>>,
    font: Option<Res<GameFont>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(font) = font {
        // 检查字体资源是否已加载
        if asset_server.is_loaded_with_dependencies(&font.handle) {
            next_state.set(GameState::MainMenu);
        }
    }
}

/// 清理加载界面
fn cleanup_loading_screen(
    mut commands: Commands,
    query: Query<Entity, With<LoadingScreenUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
