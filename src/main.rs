use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_loopless::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

use std::time::Duration;

fn main() {
    App::new()
        .add_loopless_state(GameStates::AssetsLoading)
        .add_loading_state(
            LoadingState::new(GameStates::AssetsLoading)
                .continue_to_state(GameStates::MainMenu)
                .with_collection::<GameAssets>(),
        )
        .insert_resource(Msaa { samples: 1 })
        .add_fixed_timestep(
            Duration::from_millis(125),
            "fixed_update",
        )
        .add_plugins(DefaultPlugins)
        .add_enter_system(GameStates::MainMenu, spawn_player)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameStates::MainMenu)
                .with_system(move_player)
                .into(),
        )
        .add_fixed_timestep_system(
            "fixed_update", 
            0, 
            animate_player.run_in_state(GameStates::MainMenu)
        )
        .add_system(debug_current_state)
        .add_startup_system(setup_camera)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameStates {
    AssetsLoading,
    MainMenu,
    InGame,
}

#[derive(AssetCollection, Resource)]
struct GameAssets {
    #[asset(texture_atlas(tile_size_x = 96., tile_size_y = 99., columns = 8, rows = 1))]
    #[asset(path = "images/female_adventurer_sheet.png")]
    female_adventurer: Handle<TextureAtlas>,
}

#[derive(Component)]
struct Player;

fn spawn_player(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    commands
        .spawn((SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(0., 150., 0.),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: assets.female_adventurer.clone(),
            ..Default::default()
        }, Player));
}

fn animate_player(
    mut player: Query<&mut TextureAtlasSprite, With<Player>>,
) {
    let mut sprite = player.single_mut();
    sprite.index = (sprite.index + 1) % 8;
}

fn move_player(
    input: Res<Input<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let mut movement = Vec3::new(0., 0., 0.);
    if input.pressed(KeyCode::W) {
        movement.y += 1.;
    }
    if input.pressed(KeyCode::S) {
        movement.y -= 1.;
    }
    if input.pressed(KeyCode::A) {
        movement.x -= 1.;
    }
    if input.pressed(KeyCode::D) {
        movement.x += 1.;
    }
    if movement == Vec3::ZERO {
        return;
    }
    movement = movement.normalize() * 5.;

    let mut transform = player.single_mut();
    transform.translation += movement;
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn debug_current_state(state: Res<CurrentState<GameStates>>) {
    if state.is_changed() {
        println!("Detected state change to {:?}!", state);
    }
}
