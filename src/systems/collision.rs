// 碰撞检测系统

use bevy::prelude::*;
use crate::components::*;

/// 坦克与墙壁碰撞检测
pub fn tank_wall_collision(
    mut tanks: Query<(&mut Transform, &Collider), Or<(With<Player>, With<Enemy>)>>,
    walls: Query<(&Transform, &Collider), (With<Wall>, Without<Player>, Without<Enemy>)>,
) {
    for (mut tank_transform, tank_collider) in tanks.iter_mut() {
        let tank_pos = Vec2::new(tank_transform.translation.x, tank_transform.translation.y);
        
        for (wall_transform, wall_collider) in walls.iter() {
            let wall_pos = Vec2::new(wall_transform.translation.x, wall_transform.translation.y);
            let distance = tank_pos.distance(wall_pos);
            let collision_distance = (tank_collider.size + wall_collider.size) / 2.0;
            
            if distance < collision_distance {
                // 计算分离向量
                let direction = (tank_pos - wall_pos).normalize_or_zero();
                let separation = collision_distance - distance;
                
                // 推开坦克
                tank_transform.translation.x += direction.x * separation;
                tank_transform.translation.y += direction.y * separation;
            }
        }
    }
}

/// 坦克与坦克碰撞检测
pub fn tank_tank_collision(
    mut tanks: Query<(Entity, &mut Transform, &Collider), Or<(With<Player>, With<Enemy>)>>,
) {
    let mut combinations = tanks.iter_combinations_mut();
    
    while let Some([(_entity1, mut transform1, collider1), (_entity2, mut transform2, collider2)]) = combinations.fetch_next() {
        let pos1 = Vec2::new(transform1.translation.x, transform1.translation.y);
        let pos2 = Vec2::new(transform2.translation.x, transform2.translation.y);
        let distance = pos1.distance(pos2);
        let collision_distance = (collider1.size + collider2.size) / 2.0;
        
        if distance < collision_distance && distance > 0.0 {
            // 计算分离向量
            let direction = (pos1 - pos2).normalize();
            let separation = (collision_distance - distance) / 2.0;
            
            // 推开两个坦克
            transform1.translation.x += direction.x * separation;
            transform1.translation.y += direction.y * separation;
            transform2.translation.x -= direction.x * separation;
            transform2.translation.y -= direction.y * separation;
        }
    }
}

/// 坦克与可破坏墙壁碰撞
pub fn tank_destructible_wall_collision(
    mut tanks: Query<(&mut Transform, &Collider), Or<(With<Player>, With<Enemy>)>>,
    walls: Query<(&Transform, &Collider), (With<DestructibleWall>, Without<Player>, Without<Enemy>)>,
) {
    for (mut tank_transform, tank_collider) in tanks.iter_mut() {
        let tank_pos = Vec2::new(tank_transform.translation.x, tank_transform.translation.y);
        
        for (wall_transform, wall_collider) in walls.iter() {
            let wall_pos = Vec2::new(wall_transform.translation.x, wall_transform.translation.y);
            let distance = tank_pos.distance(wall_pos);
            let collision_distance = (tank_collider.size + wall_collider.size) / 2.0;
            
            if distance < collision_distance {
                // 计算分离向量
                let direction = (tank_pos - wall_pos).normalize_or_zero();
                let separation = collision_distance - distance;
                
                // 推开坦克
                tank_transform.translation.x += direction.x * separation;
                tank_transform.translation.y += direction.y * separation;
            }
        }
    }
}
