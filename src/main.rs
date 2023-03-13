use bevy::app::App;
use bevy::core_pipeline::clear_color::ClearColor;
use bevy::render::color::Color;
use bevy::ecs::system::Commands;
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::{*,prelude::*};
use {std::f32::consts::PI, rand::Rng};
mod createk;

fn main() {
    App::new()
        .init_resource::<Game>()
        .insert_resource(ClearColor(Color::rgb(0.99, 0.99, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(createk::createk::createk)
        .add_startup_system(add_light)
        .add_system(updateframe)
        .add_system(mouse_motion)
        .add_system(move_player)
        .add_system(enemiesthink)
        .run();
}

fn add_light(
    mut commands: Commands,
    mut ambient_light: ResMut<AmbientLight>,
    ) {
    commands.spawn(PointLightBundle {
        point_light: PointLight{
                intensity: 30000.0,
                ..Default::default()
            },
        transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)),
        ..Default::default()
    });
    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 0.3;
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
            game.player.pl.k,
            game.player.pl.j,
        ),
        //rotation: Quat::from_rotation_y(time.elapsed_seconds()*0.3),
        ..default()
    };
//    *transforms.get_mut(game.enemy.entity.unwrap()).unwrap() = Transform {
//        translation: Vec3::new(
//            game.enemy.pl.i,
//            0.0,
//            game.enemy.pl.j,
//        ),
//        //rotation: Quat::from_rotation_y(time.elapsed_seconds()*5.0),
//        ..default()
//    };
    *transforms.get_mut(game.vehicle.entity.unwrap()).unwrap() = Transform {
        translation: Vec3::new(
            game.vehicle.pl.i,
            game.vehicle.pl.j,
            game.vehicle.pl.k,
        ),
        scale: Vec3::new( 10.0, 10.0,10.0 ),
        ..default()
    };
    *transforms.get_mut(game.sand.entity.unwrap()).unwrap() = Transform {
        translation: Vec3::new(
            0.0,
            -10.0,
            0.0,
        ),
        ..default()
    };
    for i in &game.enemies{
        *transforms.get_mut(i.entity.unwrap()).unwrap() = Transform {
            translation: Vec3::new(
                i.pl.i,
                i.pl.j,
                i.pl.k,
            ),
            ..default()
        };
    }
}

fn mouse_motion(
    mut motion_evr: EventReader<bevy::input::mouse::MouseMotion>,
    mut game: ResMut<Game>,
    mut query: Query<(&mut Transform, &Camera)>,
) {
    for ev in motion_evr.iter() {
        //println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
        game.camera_x -= ev.delta.y * 0.005;
        game.camera_y += ev.delta.x * 0.005;
        //game.camera.looking_at( Vec3::new(game.camera_y.cos(), game.camera_x.sin(),game.camera_y.sin()), Vec3::Y);
        for (mut transform, _) in query.iter_mut() {
        // Set the camera's position
        //transform.translation = camera_pos;

        // Calculate the direction vector to the target position
        //let direction = target_pos - camera_pos;
        //let distance = direction.length();

        // Calculate the rotation of the camera
        transform.look_at(Vec3::new(game.camera_y.cos(), game.camera_x.sin(),game.camera_y.sin()), Vec3::Y);

        // Set the camera's scale
        //transform.scale = Vec3::new(1.0, 1.0, 1.0);
    }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
) {
    let k = commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(
            0.0,
            0.0,
            0.0,
        )
        .looking_at( Vec3::new(game.camera_y.cos()*game.camera_x.cos(), game.camera_x,game.camera_y.sin()*game.camera_x.cos()), Vec3::Y),
        ..default()
    }).id();
    //game.camera = k.clone();
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
    game.sand.entity = Some(
        commands
            .spawn(SceneBundle {
                transform: Transform {
                    translation: Vec3::new(
                        0.0 as f32,
                        -10.0 as f32,
                        0.0 as f32,
                    ),
                    rotation: Quat::from_rotation_y(-PI / 2.),
                    ..default()
                },
                scene: asset_server.load("m.glb"),
                ..default()
            })
            .id(),
    );
    game.player.entity = Some(
        commands
            .spawn(SceneBundle {
                transform: Transform {
                    translation: Vec3::new(
                        game.player.pl.i as f32,
                        game.player.pl.k,
                        game.player.pl.j as f32,
                    ),
                    rotation: Quat::from_rotation_y(-PI / 2.),
                    ..default()
                },
                //scene: asset_server.load("coconut_palm.glb#Scene0"),
                //scene: asset_server.load("viking_boat.glb#Scene0"),
                ..default()
            })
            .id(),
    );
    if let Some( l ) = game.player.entity{
        //commands.entity(k).insert_children( 1,&[l] );
        commands.entity( l ) .insert_children( 1, &[k] );
    }
//    let enemy3d = asset_server.load("low-poly_fruit_box_assets.glb#Scene0");
//    game.enemy.entity = Some(
//        commands
//            .spawn(SceneBundle {
//                transform: Transform {
//                    translation: Vec3::new(
//                        game.enemy.pl.i as f32,
//                        0.0 as f32,
//                        game.enemy.pl.j as f32,
//                    ),
//                    rotation: Quat::from_rotation_y(-PI / 2.),
//                    ..default()
//                },
//                scene: enemy3d.clone(),
//                ..default()
//            })
//            .id(),
//    );
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
}

//fn createk(
//    mut commands: Commands,
//    asset_server: Res<AssetServer>,
//    mut game: ResMut<Game>,
//){
//    let enemy3d = asset_server.load("grand_piano_and_stool.glb#Scene0");
//    let enemy3d0 = asset_server.load("low-poly_fruit_box_assets.glb#Scene0");
//
//    struct what_to_create{
//        what: String,
//        wher: Place,
//    }
//
//    let elems: Vec<what_to_create> = vec![
//        what_to_create{ what: "enemy".to_string(), wher: Place{ i:3.0, j:0.0, k:2.0 } },
//        what_to_create{ what: "enemy".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy0".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy0".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy0".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy0".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy0".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy0".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy0".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy0".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy0".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy0".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy0".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy0".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy0".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy0".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy0".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy0".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy0".to_string(), wher: Place::rand() },
//        what_to_create{ what: "enemy0".to_string(), wher: Place::rand() },
//    ];
//
//    for i in elems{
//        match i.what.as_str(){
//            "enemy" => { game.enemies.push( 
//                                        Enemy{ 
//                                            entity: Some(
//                                                commands.spawn(
//                                                    SceneBundle {
//                                                        transform: Transform {
//                                                            translation: Vec3::new(
//                                                                i.wher.i,
//                                                                i.wher.j,
//                                                                i.wher.k,
//                                                            ),
//                                                            ..default()
//                                                        },
//                                                        scene: enemy3d.clone(),
//                                                        ..default()
//                                                    }
//                                                ).id(),
//                                            ), 
//                                            pl: Place{
//                                                i: i.wher.i,
//                                                j: i.wher.j,
//                                                k: i.wher.k,
//                                            }
//                                        } 
//                                    );}
//            "enemy0" => { game.enemies.push( 
//                                        Enemy{ 
//                                            entity: Some(
//                                                commands.spawn(
//                                                    SceneBundle {
//                                                        transform: Transform {
//                                                            translation: Vec3::new(
//                                                                i.wher.i,
//                                                                i.wher.j,
//                                                                i.wher.k,
//                                                            ),
//                                                            ..default()
//                                                        },
//                                                        scene: enemy3d0.clone(),
//                                                        ..default()
//                                                    }
//                                                ).id(),
//                                            ), 
//                                            pl: Place{
//                                                i: i.wher.i,
//                                                j: i.wher.j,
//                                                k: i.wher.k,
//                                            }
//                                        } 
//                                    );}
//            _ => {}
//        }
//    }
//}
fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
) 
{
    let mut moved = false;

    if keyboard_input.pressed(KeyCode::Up) {
        game.player.pl.i += 0.2;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        game.player.pl.i -= 0.2;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        game.player.pl.j += 0.2;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        game.player.pl.j -= 0.2;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::Q) {
        game.player.pl.k += 0.2;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::E) {
        game.player.pl.k -= 0.2;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::Space) {
        //search closest enemy, make it chil of player or undo
        if let Some( l ) = game.player.busy{
            //game.player.entity.unwrap().remove_children( l );
            game.player.busy = None;
        }
        else{
            if let Some( l ) = game.player.entity{
                commands.entity( l ) .insert_children( 1, &[game.enemies[0].entity.unwrap()] );
            }
            game.player.busy =game.enemies[0].entity;
        }
    }

    if moved {
        *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
            translation: Vec3::new(
                game.player.pl.i,
                game.player.pl.k,
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
    k: f32,
}
impl Place{
    fn new()
    -> Place{
        Place{
            i: 0.0,
            j: 0.0,
            k: 0.0,
        } 
    }
    fn rand()
    -> Place{
        Place{
            i: rand::thread_rng().gen_range(-40.0..40.0),
            j: 0.0,
            k: rand::thread_rng().gen_range(-40.0..40.0),
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
    busy:   Option<Entity>,
}
impl K{
    fn new()
    -> K{
        K{
            pl: Place::new(),
            entity: None,
            busy : None,
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

#[derive(Component, Default)]
struct Ocean{
    entity: Option<Entity>,
}

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
pub struct Game {
    player: K,
    enemy: Enemy,
    vehicle: Vehicle,
    enemies: Vec<Enemy>,
    score: i32,
    camera : Option<Entity>,
    camera_y: f32,
    camera_x: f32,
    sand: Ocean,
}
