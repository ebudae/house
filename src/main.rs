use bevy::app::App;
use bevy::core_pipeline::clear_color::ClearColor;
use bevy::render::color::Color;
use bevy::ecs::system::Commands;
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::{*,prelude::*};
use {std::f32::consts::PI, rand::Rng};

fn main() {
    App::new()
        .init_resource::<Game>()
        .insert_resource(ClearColor(Color::rgb(0.99, 0.99, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(add_light)
        .add_system(updateframe)
        .add_system(move_player)
        .add_system(enemiesthink)
        .run();
}

fn add_light(mut commands: Commands) {
    let p:PointLight = PointLight{
        intensity: 30000.0,
        ..Default::default()
    };
    commands.spawn(PointLightBundle {
        point_light: p,
        transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)),
        ..Default::default()
    });
}

fn updateframe(
    mut game: ResMut<Game>,
    time: Res<Time>,
    mut transforms: Query<&mut Transform>,
){
    //game.player.pl.i = time.elapsed_seconds().sin() * 10.0 ;
    *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
        translation: Vec3::new(
            game.player.pl.i,
            0.0,
            game.player.pl.j,
        ),
        //rotation: Quat::from_rotation_y(time.elapsed_seconds()*0.3),
        ..default()
    };
    *transforms.get_mut(game.enemy.entity.unwrap()).unwrap() = Transform {
        translation: Vec3::new(
            game.enemy.pl.i,
            0.0,
            game.enemy.pl.j,
        ),
        //rotation: Quat::from_rotation_y(time.elapsed_seconds()*5.0),
        ..default()
    };
    *transforms.get_mut(game.vehicle.entity.unwrap()).unwrap() = Transform {
        translation: Vec3::new(
            game.vehicle.pl.i,
            0.0,
            game.vehicle.pl.j,
        ),
        scale: Vec3::new( 10.0, 10.0,10.0 ),
        ..default()
    };
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
) {
    let k = commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(
            -(10 as f32),
            4.0,
            5.0,
        )
        .looking_at( Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    }).id();
//    commands.spawn(K::new());
//    commands.spawn(Vehicle{});
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
                scene: asset_server.load("coconut_palm.glb#Scene0"),
                ..default()
            })
            .id(),
    );
    if let Some( l ) = game.player.entity{
        //commands.entity(k).insert_children( 1,&[l] );
        commands.entity( l ) .insert_children( 1, &[k] );
    }
    game.enemy.entity = Some(
        commands
            .spawn(SceneBundle {
                transform: Transform {
                    translation: Vec3::new(
                        game.enemy.pl.i as f32,
                        0.0 as f32,
                        game.enemy.pl.j as f32,
                    ),
                    rotation: Quat::from_rotation_y(-PI / 2.),
                    ..default()
                },
                scene: asset_server.load("low-poly_fruit_box_assets.glb#Scene0"),
                ..default()
            })
            .id(),
    );
    game.vehicle.entity = Some(
        commands
            .spawn(SceneBundle {
                transform: Transform {
                    translation: Vec3::new(
                        game.vehicle.pl.i as f32,
                        0.0 as f32,
                        game.vehicle.pl.j as f32,
                    ),
                    rotation: Quat::from_rotation_y(-PI / 2.),
                    scale: Vec3::new( 10.0, 10.0,10.0 ),
                },
                scene: asset_server.load("small_wooden_boat.glb#Scene0"),
                ..default()
            })
            .id(),
    );
//    for i in 0..10{
//        game.enemies.resize( 10,
//            commands.spawn(SceneBundle {
//                    transform: Transform {
//                        translation: Vec3::new(
//                            game.enemies[i].pl.i as f32,
//                            0.0 as f32,
//                            game.enemies[i].pl.j as f32,
//                        ),
//                        rotation: Quat::from_rotation_y(-PI / 2.),
//                        ..default()
//                    },
//                    scene: asset_server.load("low-poly_fruit_box_assets.glb#Scene0"),
//                    ..default()
//                })
//                .id(),
//        );
//    }
}
fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
) 
{
    let mut moved = false;

    if keyboard_input.pressed(KeyCode::Up) {
        game.player.pl.i += 1.0;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        game.player.pl.i -= 1.0;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        game.player.pl.j += 1.0;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        game.player.pl.j -= 1.0;
        moved = true;
    }

    if moved {
        *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
            translation: Vec3::new(
                game.player.pl.i,
                0.0,
                game.player.pl.j,
            ),
            ..default()
        };   

    }
}

fn enemiesthink(
    mut game: ResMut<Game>,
){
    game.enemy.pl.i += rand::thread_rng().gen_range(-0.1..0.1) *0.0;
}

fn queryplayer(mut q: Query<&K>) {
    let k = q.single_mut();
}

#[derive(Component)]
struct Health (f32);

#[derive(Component, Default, Clone)]
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

#[derive(Component, Default)]
struct Price (f32);

#[derive(Component)]
struct Nme (String);

#[derive(Component, Default, Clone)]
struct Enemy{
    pl: Place,
    entity: Option<Entity>,
}
impl Enemy{
    fn new()
    -> Enemy{
        Enemy{
            pl: Place::new(),
            entity: None,
        }
    }
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

#[derive(Component, Default)]
struct Vehicle{
    pl:     Place,
    entity: Option<Entity>,
}
impl Vehicle{
    fn new()
    -> Vehicle{
        Vehicle{
            pl: Place::new(),
            entity: None,
        }
    }
}

#[derive(Component, Default)]
struct Nmo{
    pl:     Place,
    entity: Option<Entity>,
}
impl Nmo{
    fn new()
    -> Nmo{
        Nmo{
            pl: Place::new(),
            entity: None,
        }
    }
}

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
    enemy: Enemy,
    vehicle: Vehicle,
    enemies: Vec<Enemy>,
    score: i32,
    camera_focus: Vec3,
}
