// UI 系统

use bevy::prelude::*;
use bevy::state::state::{State, NextState};
use crate::components::*;
use crate::resources::*;
use crate::states::GameState;

/// 设置游戏 HUD
pub fn setup_hud(mut commands: Commands) {
    // 背景容器
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        },
    )).with_children(|parent| {
        // 顶部信息栏
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.5).into(),
            ..default()
        }).with_children(|parent| {
            // 得分
            parent.spawn((
                TextBundle::from_section(
                    "得分: 0",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                UIScoreText,
            ));
            
            // 生命值
            parent.spawn((
                TextBundle::from_section(
                    "生命: 3",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                UILivesText,
            ));
            
            // 波次
            parent.spawn((
                TextBundle::from_section(
                    "波次: 1",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                UIWaveText,
            ));
        });
    });
}

/// 更新 HUD 信息
pub fn update_hud(
    score: Res<GameScore>,
    lives: Res<PlayerLives>,
    wave: Res<CurrentWave>,
    _player_query: Query<&Health, With<Player>>,
    mut score_text: Query<&mut Text, (With<UIScoreText>, Without<UILivesText>, Without<UIWaveText>)>,
    mut lives_text: Query<&mut Text, (With<UILivesText>, Without<UIScoreText>, Without<UIWaveText>)>,
    mut wave_text: Query<&mut Text, (With<UIWaveText>, Without<UIScoreText>, Without<UILivesText>)>,
) {
    // 更新得分
    for mut text in score_text.iter_mut() {
        text.sections[0].value = format!("得分: {}", score.score);
    }
    
    // 更新生命
    for mut text in lives_text.iter_mut() {
        text.sections[0].value = format!("生命: {}", lives.lives);
    }
    
    // 更新波次
    for mut text in wave_text.iter_mut() {
        text.sections[0].value = format!("波次: {}", wave.wave);
    }
}

/// 设置主菜单
pub fn setup_main_menu(mut commands: Commands) {
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
        MainMenuUI,
    )).with_children(|parent| {
        // 标题
        parent.spawn(TextBundle::from_section(
            "坦克大战",
            TextStyle {
                font_size: 60.0,
                color: Color::WHITE,
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::all(Val::Px(30.0)),
            ..default()
        }));
        
        // 开始按钮
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(60.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::srgb(0.2, 0.5, 0.2).into(),
                ..default()
            },
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "开始游戏",
                TextStyle {
                    font_size: 32.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
        
        // 说明文字
        parent.spawn(TextBundle::from_section(
            "WASD/方向键移动 | 鼠标控制方向 | 空格/左键射击\nESC 暂停",
            TextStyle {
                font_size: 20.0,
                color: Color::srgb(0.7, 0.7, 0.7),
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::top(Val::Px(50.0)),
            ..default()
        }));
    });
}

/// 主菜单按钮交互
pub fn main_menu_interaction(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.1, 0.3, 0.1).into();
                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.25, 0.6, 0.25).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.2, 0.5, 0.2).into();
            }
        }
    }
}

/// 清理主菜单
pub fn cleanup_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// 设置游戏结束界面
pub fn setup_game_over(mut commands: Commands, score: Res<GameScore>) {
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
            background_color: Color::srgba(0.1, 0.0, 0.0, 0.9).into(),
            ..default()
        },
        GameOverUI,
    )).with_children(|parent| {
        // 游戏结束标题
        parent.spawn(TextBundle::from_section(
            "游戏结束",
            TextStyle {
                font_size: 60.0,
                color: Color::srgb(1.0, 0.3, 0.3),
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::all(Val::Px(30.0)),
            ..default()
        }));
        
        // 最终得分
        parent.spawn(TextBundle::from_section(
            format!("最终得分: {}", score.score),
            TextStyle {
                font_size: 40.0,
                color: Color::WHITE,
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::all(Val::Px(20.0)),
            ..default()
        }));
        
        // 重新开始按钮
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(60.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::srgb(0.2, 0.5, 0.2).into(),
                ..default()
            },
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "重新开始",
                TextStyle {
                    font_size: 32.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
    });
}

/// 游戏结束按钮交互
pub fn game_over_interaction(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.1, 0.3, 0.1).into();
                next_state.set(GameState::MainMenu);
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.25, 0.6, 0.25).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.2, 0.5, 0.2).into();
            }
        }
    }
}

/// 清理游戏结束界面
pub fn cleanup_game_over(
    mut commands: Commands,
    query: Query<Entity, With<GameOverUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// 检查游戏结束条件
pub fn check_game_over(
    mut next_state: ResMut<NextState<GameState>>,
    lives: Res<PlayerLives>,
    player_query: Query<&Player>,
) {
    // 如果生命为0且没有玩家实体，游戏结束
    if lives.lives == 0 && player_query.is_empty() {
        next_state.set(GameState::GameOver);
    }
}

/// 暂停切换
pub fn toggle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
            _ => {}
        }
    }
}
