use bevy::{ app::App,
            core_pipeline::{core_3d::Camera3dBundle,
                            clear_color::ClearColor},
            render::color::Color,
            ecs::system::Commands,
            {*,prelude::*,window::CursorGrabMode},
            render::mesh,
            render::mesh::*};
use {std::f32::consts::PI, rand::Rng};
mod createk;
mod ocean;
mod eye;
mod vehicle;

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_state::<TravelMode>()
        .insert_resource(ClearColor(Color::rgb(0.99, 0.99, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(createk::createk)
        .add_startup_system(add_light)
        .add_startup_system(ocean::ocean::Ocean::create)
        .add_system(mouse_motion)
        .add_system(nomau5)
        .add_system(move_player.in_set( OnUpdate(TravelMode::Walk) ))
        .add_system(move_vhc.in_set( OnUpdate(TravelMode::Vehicle) ))
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
    game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
){
    *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
        translation: game.player.pl.to_vec3(),
        ..default()
    };
    transforms.get_mut(game.eye.unwrap()).unwrap() .look_at( game.player.forward(), Vec3::Y);
    
    *transforms.get_mut(game.vehicle.entity.unwrap()).unwrap() = Transform {
        translation: game.vehicle.pl.to_vec3 (),
        rotation: math::f32::Quat::from_rotation_y( game.vehicle.camera_y ),
        scale: Vec3::new( 10.0, 10.0,10.0 ),
    };
    *transforms.get_mut(game.sand.entity.unwrap()).unwrap() = Transform {
        translation: Vec3::new(
            0.0,
            -10.0,
            0.0,
        ),
        scale: Vec3::new( time.elapsed_seconds().sin(),time.elapsed_seconds().sin(), time.elapsed_seconds().sin() ),
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

fn nomau5(
    mut windows: Query<&mut Window>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}

const MOUSE_SP: f32 = 0.005;
fn mouse_motion(
    mut motion_evr: EventReader<bevy::input::mouse::MouseMotion>,
    mut game: ResMut<Game>,
    mut query: Query<(&mut Transform, &Camera)>,
){
    for ev in motion_evr.iter() {
        game.player.camera_x -= ev.delta.y * MOUSE_SP;
        game.player.camera_y -= ev.delta.x * MOUSE_SP;
        for (mut transform, _) in query.iter_mut() {
            transform.look_at( game.player.forward(), Vec3::Y)
            //transform.scale = Vec3::new(1.0, 1.0, 1.0);
        }
        if let Some( _l ) = game.player.busy{
            let m = game.player.accum_forward();
            game.enemies[0].pl.from_vec3( m);
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<Game>,
){
    game.eye = Some(
        commands.spawn(Camera3dBundle {
            transform: Transform::from_xyz( 0.0, 0.0, 0.0 )
            .looking_at( game.player.forward(), Vec3::Y),
            ..default()
        })
        .id()
    );

    // commands.insert_resource(GoalsReached { 
    //     main_goal0: false,
    //     main_goal1: false,
    //     main_goal2: false,
    //     main_goal3: false,
    //     main_goal4: false,
    //     main_goal5: false,
    //     bonus0: false,
    //     bonus1: false,
    //     bonus2: false,
    //     bonus3: false,
    //     bonus4: false,
    //     bonus5: false,
    //});
    game.sand.entity = Some(
        commands.spawn(SceneBundle {
            transform: Transform {
                translation: Vec3::new( 0.0 as f32, -10.0 as f32, 0.0 as f32 ),
                rotation: Quat::from_rotation_y(-PI / 2.),
                ..default()
            },
            scene: asset_server.load("mesh0 .glb"),
            ..default()
        })
        .id(),
    );
    game.ocean.entity = Some(
        commands.spawn(SceneBundle {
            transform: Transform {
                translation: Vec3::new( 0.0 as f32, -10.0 as f32, 0.0 as f32 ),
                rotation: Quat::from_rotation_y(-PI / 2.),
                ..default()
            },
            scene: asset_server.load("mossy_rusty_sheet.glb"),
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
    
    commands.entity( game.player.entity.unwrap() ) .insert_children( 1, &[game.eye.unwrap()] );
    
    let k = commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight{
                color: render::color::Color::BLUE,
                illuminance: 30000.0,
                //shadows_enabled: true,
                ..Default::default()
            },
        transform: Transform{ 
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: math::f32::Quat::from_rotation_y( 1.5 * PI ),
            scale: Vec3::new(1.0, 1.0, 1.0),
            },
        ..Default::default()
    }).id();

    game.vehicle.entity = Some(
        commands.spawn(SceneBundle {
            transform: Transform {
                translation: Vec3::new( game.vehicle.pl.i, game.vehicle.pl.j, game.vehicle.pl.k ),
                rotation: math::f32::Quat::from_rotation_y( game.vehicle.camera_y ),
                scale: Vec3::new( 10.0, 10.0,10.0 ),
            },
            scene: asset_server.load("small_wooden_boat_new.glb#Scene0"),
            ..default()
        })
        .id(),
    );
    commands.entity( game.vehicle.entity.unwrap() ) .insert_children( 1, &[k] );

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[0., 0., 0.], [10., 0., 10.], [10., 0., 0.], [0.0, 0.0, 10.0]]
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 4]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 4]);

    mesh.set_indices(Some(mesh::Indices::U32(vec![0, 1, 2, 1, 0, 3])));

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0.2, 0.5, 0.3).into()),
        ..default()
    });

}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    //mut commands: Commands,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    mut next_state: ResMut<NextState<TravelMode>>,
    //mut walkingmode: ResMut<TravelMode>,
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

    let q = Island::get_level( game.player.pl.i, game.player.pl.k );
    let j =  0.2*(( game.player.pl.i + game.player.pl.k )*0.2).sin();
    game.player.pl.j = if q < j{ j }else { q };
    
    if keyboard_input.just_pressed(KeyCode::Z) {
        let k = game.vehicle.passenger();
        game.player.pl.from_vec3( k );
        next_state.set(TravelMode::Vehicle);
    }
    //game.player.pl.j = 0.0;

    if keyboard_input.just_pressed(KeyCode::Space) {
        //search closest enemy, make it chil of player or undo
        let k = game.player.accum_forward();
        game.enemies[0].pl.from_vec3( k );
        if let Some( _l ) = game.player.busy{
            //*transforms.get_mut(game.player.busy.unwrap()).unwrap() = Transform::from_translation( game.player.accum_forward()) ;
            //game.player.busy.unwrap().pl.from_vec3( game.player.forward() );
            //commands.add( RemoveParent{ child: game.player.busy.unwrap() } );
            game.player.unbusy();
        }
        else{
            //if let Some( l ) = game.player.entity{
            //let k = game.player.accum_forward();
            //game.enemies[0].pl.from_vec3( k);
            //commands.entity( game.player.entity.unwrap() ) .insert_children( 1, &[game.enemies[0].entity.unwrap()] );
            //}
            game.player.busy = game.enemies[0].entity;
        }
    }

    if moved {
        if let Some( _l ) = game.player.busy{
            let k = game.player.accum_forward();
            game.enemies[0].pl.from_vec3( k );
            }
        *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
            translation: Vec3::new( game.player.pl.i, game.player.pl.j, game.player.pl.k ),
            ..default()
        };   
    }
}
fn move_vhc(
    keyboard_input: Res<Input<KeyCode>>,
    //mut commands: Commands,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    mut next_state: ResMut<NextState<TravelMode>>,
    //mut walkingmode: ResMut<TravelMode>,
){
    let mut moved = false;

    if keyboard_input.pressed(KeyCode::W) {
        let k = game.vehicle.forward();
        game.vehicle.pl.i += 0.4 * k.x;
        game.vehicle.pl.j += 0.4 * k.y;
        game.vehicle.pl.k += 0.4 * k.z;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::S) {
        let k = game.vehicle.forward();
        game.vehicle.pl.i -= 0.4 * k.x;
        game.vehicle.pl.j -= 0.4 * k.y;
        game.vehicle.pl.k -= 0.4 * k.z;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::A) {
        game.player.camera_y += 0.02;
        game.vehicle.camera_y += 0.02;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::D) {
        game.player.camera_y -= 0.02;
        game.vehicle.camera_y -= 0.02;
        moved = true;
    }
    
    if keyboard_input.just_pressed(KeyCode::Z) {
        next_state.set(TravelMode::Walk);
    }
    //game.player.pl.j = 0.0;

    if moved {
        *transforms.get_mut(game.vehicle.entity.unwrap()).unwrap() = Transform {
            translation: Vec3::new( game.vehicle.pl.i, game.vehicle.pl.j, game.vehicle.pl.k ),
            rotation: math::f32::Quat::from_rotation_y( game.vehicle.camera_y ),
            ..default()
        };
        let k = game.vehicle.passenger();
        game.player.pl.from_vec3( k );
    }
}

fn enemiesthink(
    //mut game: ResMut<Game>,
){
    //for enemy in &mut game.enemies{
    //    enemy.pl.i += rand::thread_rng().gen_range(-0.1..0.1) *0.005 ;
    //}
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
//#[derive(Resource, Default)]
enum TravelMode{
    #[default]
    Walk,
    Vehicle,
}

#[derive(Component)]
struct Health (f32);

#[derive(Component, Default, Clone)]
pub struct Place{
    i: f32,
    j: f32,
    k: f32,
}
impl Place{
//    fn new()
//    -> Self{
//        Place{
//            i: 0.0,
//            j: 0.0,
//            k: 0.0,
//        } 
//    }
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

#[derive(Component)]
struct Friendly;

//#[derive(Component, Default)]
//struct Nmo{
//    pl:     Place,
//    entity: Option<Entity>,
//}
//impl Nmo{
//    fn new()
//    -> Self{
//        Nmo{
//            pl: Place::new(),
//            entity: None,
//        }
//    }
//}

//#[derive(Resource)]
//struct GoalsReached {
//    main_goal0: bool,
//    main_goal1: bool,
//    main_goal2: bool,
//    main_goal3: bool,
//    main_goal4: bool,
//    main_goal5: bool,
//    bonus0: bool,
//    bonus1: bool,
//    bonus2: bool,
//    bonus3: bool,
//    bonus4: bool,
//    bonus5: bool,
//}

#[derive(Component, Default)]
struct Island{
    entity: Option<Entity>,
    //waves: Vec<Wave>,
}
impl Island{
    fn get_level( x:f32, y:f32 )
    -> f32{
        ( x*x+y*y ).sqrt().sin() / ( x*x+y*y ).sqrt()
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    pl: Place,
    health: Health,
    price: Price,
    _p:  eye::K,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Resource, Default)]
pub struct Game {
    eye: Option<Entity>,
    player: eye::K,
    vehicle: vehicle::Vehicle,
    enemies: Vec<Enemy>,
    sand: Island,
    ocean: ocean::ocean::Ocean,
}
