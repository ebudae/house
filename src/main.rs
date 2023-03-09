use bevy::app::App;
use bevy::core_pipeline::clear_color::ClearColor;
use bevy::render::color::Color;
use bevy::ecs::system::Commands;
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::{*,prelude::*};
use std::f32::consts::PI;

fn main() {
    App::new()
        .init_resource::<Game>()
        .insert_resource(ClearColor(Color::rgb(0.99, 0.99, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(queryplayer)
        .run();
}

fn setup(
    mut commands: Commands,
    game: Res<Game>,
) {
    commands.spawn(Camera3dBundle::default());
    commands.spawn(K::new());
    commands.spawn(Vehicle{});
    commands.insert_resource(Ocean{});
    commands.insert_resource(GoalsReached { 
        main_goal0: false,
        main_goal1: false,
        main_goal2: false,
        main_goal3: false,
        main_goal4: false,
        main_goal5: false,
        bonus0: false,
        bonus1: false,
        bonus2: false,
        bonus3: false,
        bonus4: false,
        bonus5: false,
    });
    game.player.entity = Some(
        commands
            .spawn(SceneBundle {
                transform: Transform {
                    translation: Vec3::new(
                        game.player.pl.i as f32,
                        0.0 as f32,
                        game.player.pl.j as f32,
                    ),
                    rotation: Quat::from_rotation_y(-PI / 2.),
                    ..default()
                },
                scene: asset_server.load("models/AlienCake/alien.glb#Scene0"),
                ..default()
            })
            .id(),
    );
}

fn queryplayer(mut q: Query<&K>) {
    let k = q.single_mut();
}

#[derive(Component)]
struct Health (f32);

#[derive(Default)]
#[derive(Component)]
struct Place{
    i: f32,
    j: f32,
}
impl Place{
    fn new()
    -> Place{
        Place{
            i: 0.0,
            j: 0.0,
        } 
    }
}

#[derive(Component)]
struct Price (f32);

#[derive(Component)]
struct Nme (String);

#[derive(Component)]
struct Enemy{
    pl: Place,
    entity: Option<Entity>,
}

#[derive(Component, Default)]
struct K{
    pl:     Place,
    entity: Option<Entity>,
}
impl K{
    fn new()
    -> K{
        K{
            pl: Place::new(),
            entity: None,
        }
    }
}

#[derive(Component)]
struct Friendly;

#[derive(Component)]
struct Vehicle;

#[derive(Resource)]
struct GoalsReached {
    main_goal0: bool,
    main_goal1: bool,
    main_goal2: bool,
    main_goal3: bool,
    main_goal4: bool,
    main_goal5: bool,
    bonus0: bool,
    bonus1: bool,
    bonus2: bool,
    bonus3: bool,
    bonus4: bool,
    bonus5: bool,
}

#[derive(Resource, Default, Debug)]
struct Ocean();

#[derive(Bundle)]
struct PlayerBundle {
    name: Nme,
    pl: Place,
    health: Health,
    price: Price,
    _p: K,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Resource, Default)]
struct Game {
    player: K,
    score: i32,
    camera_focus: Vec3,
}