pub mod util;

use bevy::{prelude::*, math::vec3};
use bevy_ecs_ldtk::prelude::*;
// use bevy_editor_pls::prelude::*;
use bevy_rapier2d::{prelude::{RapierPhysicsPlugin, NoUserData, Collider}, render::RapierDebugRenderPlugin};

const CELL: f32 = 16.0 * SCALE; // size of a cell
const DEFAULT_FAR: f32 = 999.0;

const WIDTH: f32 = 512.0 * SCALE;
const HEIGHT: f32 = 256.0 * SCALE;

const SCALE: f32 = 1.5;

#[derive(Component, Default)]
enum BlockType {
    #[default]
    Solid,
    Semi,
    Target
}

#[derive(Default, Component)]
pub struct PlayerTag;

#[derive(Bundle, LdtkEntity)]
pub struct PlayerEntity {
    // transform: Transform,
    tag: PlayerTag,

    #[sprite_bundle]
    #[bundle]
    sprite: SpriteBundle,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Untitled".to_string(),
                width: WIDTH,
                height: HEIGHT,
                resizable: false,
                ..default()
            },
            ..default()
        }).set(AssetPlugin {
            watch_for_changes: true,            // hot-reload
            ..default()
        }).set(ImagePlugin::default_nearest())) // no antialias 
       
        // .add_plugin(EditorPlugin)
        

        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<PlayerEntity>("Player")

        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(CELL))
        .add_plugin(RapierDebugRenderPlugin::default())

        .add_startup_system(setup_pre)
        .add_system(setup_player.after(setup_pre))
        .add_system(setup_blocks.after(setup_pre))
        .run();
}

fn setup_pre(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
    ) {
   commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(WIDTH/2.0, HEIGHT/2.0, DEFAULT_FAR),
        ..default()
   });

   commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("proto/testmap.ldtk"),
        transform: Transform::from_scale(vec3(SCALE, SCALE, SCALE)),
        ..default()
   });
}

fn setup_player(
    mut commands: Commands,
    player: Query<Entity, Added<PlayerTag>>
    ) {
    if player.is_empty() { return; }
    let player = player.single();

    commands.entity(player)
        .insert(Collider::capsule_y(10.0, 3.0));
}

fn setup_blocks(
    mut commands: Commands
    ) {

}
