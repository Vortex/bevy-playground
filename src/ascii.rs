use bevy::prelude::*;

use crate::TILE_SIZE;

pub struct AsciiPlugin;

pub struct AsciiSheet(pub Handle<TextureAtlas>);

#[derive(Component)]
pub struct AsciiText;

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_ascii);
    }
}

pub fn spawn_ascii_text(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    to_print: &str,
    left_center: Vec3,
) -> Entity {
    let mut character_sprites = Vec::new();
    let color = Color::rgb(0.8, 0.8, 0.8);

    for (i, char) in to_print.chars().enumerate() {
        character_sprites.push(spawn_ascii_sprite(
            commands,
            ascii,
            char as usize,
            color,
            Vec3::new(i as f32 * TILE_SIZE, 0.0, 0.0),
        ))
    }

    commands
        .spawn()
        .insert(Name::new(format!("Text - {}", to_print)))
        .insert(AsciiText)
        .insert(Transform {
            translation: left_center,
            ..Default::default()
        })
        .insert(GlobalTransform::default())
        .push_children(&character_sprites)
        .id()
}

pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    index: usize,
    color: Color,
    translation: Vec3,
) -> Entity {
    assert!(index < 256, "Index out of Ascii Range");

    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = color;
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}

fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("Ascii.png");
    let atlas =
        TextureAtlas::from_grid_with_padding(image, Vec2::splat(9.0), 16, 16, Vec2::splat(2.0));

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(AsciiSheet(atlas_handle));
}
