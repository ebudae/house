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
        .add_system(mouse_motion)
        .add_system(move_player)
        .add_system(enemiesthink)
        .add_system(updateframe)
        .run();
}

fn add_light(
    mut commands: Commands,
    mut ambient_light: ResMut<AmbientLight>,
){
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
    mut transforms: Query<&mut Transform>,
){
    *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
        translation: game.player.pl.to_vec3(),
        ..default()
    };
    *transforms.get_mut(game.vehicle.entity.unwrap()).unwrap() = Transform {
        translation: game.vehicle.pl.to_vec3 (),
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

const mouse_sp: f32 = 0.005;
fn mouse_motion(
    mut motion_evr: EventReader<bevy::input::mouse::MouseMotion>,
    mut game: ResMut<Game>,
    mut query: Query<(&mut Transform, &Camera)>,
){
    for ev in motion_evr.iter() {
        game.player.camera_x -= ev.delta.y * mouse_sp;
        game.player.camera_y += ev.delta.x * mouse_sp;
        for (mut transform, _) in query.iter_mut() {
            transform.look_at( game.player.forward(), Vec3::Y)
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
        transform: Transform::from_xyz( 0.0, 0.0, 0.0 )
        .looking_at( game.player.forward(), Vec3::Y),
        ..default()
    })
    .id();

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
        commands.spawn(SceneBundle {
            transform: Transform {
                translation: Vec3::new( 0.0 as f32, -10.0 as f32, 0.0 as f32 ),
                rotation: Quat::from_rotation_y(-PI / 2.),
                ..default()
            },
            scene: asset_server.load("tropical_island.glb"),
            ..default()
        })
        .id(),
    );
    game.player.entity = Some(
        commands.spawn(SceneBundle {
            transform: Transform {
                translation: Vec3::new( game.player.pl.i, game.player.pl.j, game.player.pl.k ),
                rotation: Quat::from_rotation_y(-PI / 2.),
                ..default()
            },
            //scene: asset_server.load("coconut_palm.glb#Scene0"),
            //scene: asset_server.load("viking_boat.glb#Scene0"),
            ..default()
        })
        .id(),
    );
    
    commands.entity( game.player.entity.unwrap() ) .insert_children( 1, &[k] );
    
    game.vehicle.entity = Some(
        commands.spawn(SceneBundle {
            transform: Transform {
                translation: Vec3::new( game.vehicle.pl.i, game.vehicle.pl.j, game.vehicle.pl.k ),
                rotation: Quat::from_rotation_y(-PI / 2.),
                scale: Vec3::new( 10.0, 10.0,10.0 ),
            },
            scene: asset_server.load("small_wooden_boat.glb#Scene0"),
            ..default()
        })
        .id(),
    );
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
){
    let mut moved = false;

    if keyboard_input.pressed(KeyCode::W) {
        let k = game.player.forward();
        game.player.pl.i += 0.1 * k.x;
        game.player.pl.j += 0.1 * k.y;
        game.player.pl.k += 0.1 * k.z;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::S) {
        let k = game.player.forward();
        game.player.pl.i -= 0.1 * k.x;
        game.player.pl.j -= 0.1 * k.y;
        game.player.pl.k -= 0.1 * k.z;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::Q) {
        game.player.pl.j += 0.1;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::E) {
        game.player.pl.j -= 0.1;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::D) {
        let k = game.player.right();
        game.player.pl.i += 0.1 * k.x;
        game.player.pl.j += 0.1 * k.y;
        game.player.pl.k += 0.1 * k.z;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::A) {
        let k = game.player.right();
        game.player.pl.i -= 0.1 * k.x;
        game.player.pl.j -= 0.1 * k.y;
        game.player.pl.k -= 0.1 * k.z;
        moved = true;
    }
    if keyboard_input.just_pressed(KeyCode::Space) {
        //search closest enemy, make it chil of player or undo
        if let Some( l ) = game.player.busy{
            *transforms.get_mut(game.player.busy.unwrap()).unwrap() = Transform::from_translation( game.player.accum_forward()) ;
            //game.player.busy.unwrap().pl.from_vec3( game.player.forward() );
            let k = game.player.accum_forward();
            game.enemies[0].pl.from_vec3( k );
            commands.add( RemoveParent{ child: game.player.busy.unwrap() } );
            game.player.busy = None;
        }
        else{
            //if let Some( l ) = game.player.entity{
            let k = game.player.accum_forward();
            game.enemies[0].pl.from_vec3( k);
            commands.entity( game.player.entity.unwrap() ) .insert_children( 1, &[game.enemies[0].entity.unwrap()] );
            //}
            game.player.busy = game.enemies[0].entity;
        }
    }

    if moved {
        *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
            translation: Vec3::new( game.player.pl.i, game.player.pl.j, game.player.pl.k ),
            ..default()
        };   
    }
}

fn enemiesthink(
    mut game: ResMut<Game>,
){
    for enemy in &mut game.enemies{
        enemy.pl.i += rand::thread_rng().gen_range(-0.1..0.1) *0.005 ;
    }
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
    -> Self{
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
    fn from_vec3( &mut self, k:Vec3 ){
        self.i = k.x;
        self.j = k.y;
        self.k = k.z;
    }
    fn to_vec3( &self )
    -> Vec3{
        Vec3::new( self.i, self.j, self.k )
    }
}

#[derive(Component, Default)]
struct Price (f32);

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
    camera_y: f32,
    camera_x: f32,
}
impl K{
    fn new()
    -> Self{
        K{
            pl: Place::new(),
            entity: None,
            busy : None,
            camera_y: 0.0,
            camera_x: 0.0,
        }
    }
    fn forward( &self )
    -> Vec3{
        Vec3::new( self.camera_y.cos() * self.camera_x.cos(), self.camera_x.sin(), self.camera_y.sin() * self.camera_x.cos())
    }
    fn accum_forward( &self )
    -> Vec3{
        Vec3::new( self.pl.i + self.camera_y.cos() * self.camera_x.cos(), self.pl.j + self.camera_x.sin(),self.pl. k + self.camera_y.sin() * self.camera_x.cos())
    }
    fn right( &self )
    -> Vec3{
        Vec3::new( self.camera_y.sin()* - self.camera_x.cos() , 0.0, self.camera_y.cos() * self.camera_x.cos())
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
    -> Self{
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
    -> Self{
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
    vehicle: Vehicle,
    enemies: Vec<Enemy>,
    sand: Ocean,
}
