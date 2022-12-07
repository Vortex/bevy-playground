use bevy::prelude::*;

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    GameState,
};

pub struct CombatPlugin;

#[derive(Component)]
pub struct Enemy;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Combat).with_system(test_exit_combat))
            .add_system_set(SystemSet::on_enter(GameState::Combat).with_system(spawn_enemy))
            .add_system_set(SystemSet::on_exit(GameState::Combat).with_system(despawn_enemy));
    }
}

fn spawn_enemy(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let sprite = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        'b' as usize,
        Color::rgb(0.8, 0.8, 0.8),
        Vec3::new(0.0, 0.5, 100.0),
    );

    commands
        .entity(sprite)
        .insert(Enemy)
        .insert(Name::new("Bat"));
}

fn despawn_enemy(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn test_exit_combat(mut keyboard: ResMut<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        println!("Changing to overworld");
        state.set(GameState::Overworld).unwrap();
        keyboard.clear();
    }
}
