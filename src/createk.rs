pub mod createk{

    use bevy::core_pipeline::clear_color::ClearColor;
    use bevy::ecs::system::Commands;
    use bevy::{prelude::*};
    use {std::f32::consts::PI, rand::Rng};
    
    pub fn createk(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut game: ResMut<crate::Game>,
    ){
        let enemy3d = asset_server.load("grand_piano_and_stool.glb#Scene0");
        let enemy3d0 = asset_server.load("low-poly_fruit_box_assets.glb#Scene0");
        let la_night_city = asset_server.load("la_night_city.glb#Scene0");
        //let mossy_rusty_sheet = asset_server.load("mossy_rusty_sheet.glb#Scene0");
        //let concrete_barrier = asset_server.load("concrete_barrier.glb#Scene0");
        //let fantasy_town = asset_server.load("fantasy_town.glb#Scene0");
        //let simple_wood_planks_debris_pack = asset_server.load("simple_wood_planks_debris_pack.glb#Scene0");
        //let q63dae3c872f63b7131e032b6 = asset_server.load(r"girl-woman-glb-animated\source\63dae3c872f63b7131e032b6.glb#Scene0");

        struct what_to_create{
            what: String,
            wher: crate::Place,
        }

        let elems: Vec<what_to_create> = vec![
            what_to_create{ what: "enemy".to_string(), wher: crate::Place{ i:3.0, j:0.0, k:2.0 } },
            what_to_create{ what: "enemy".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            what_to_create{ what: "city".to_string(), wher: crate::Place::rand() },
        ];

        for i in elems{
            match i.what.as_str(){
                "enemy" => { game.enemies.push( 
                                            crate::Enemy{ 
                                                entity: Some(
                                                    commands.spawn(
                                                        SceneBundle {
                                                            transform: Transform {
                                                                translation: Vec3::new(
                                                                    i.wher.i,
                                                                    i.wher.j,
                                                                    i.wher.k,
                                                                ),
                                                                ..default()
                                                            },
                                                            scene: enemy3d.clone(),
                                                            ..default()
                                                        }
                                                    ).id(),
                                                ), 
                                                pl: crate::Place{
                                                    i: i.wher.i,
                                                    j: i.wher.j,
                                                    k: i.wher.k,
                                                }
                                            } 
                                        );}
                "enemy0" => { game.enemies.push( 
                                            crate::Enemy{ 
                                                entity: Some(
                                                    commands.spawn(
                                                        SceneBundle {
                                                            transform: Transform {
                                                                translation: Vec3::new(
                                                                    i.wher.i,
                                                                    i.wher.j,
                                                                    i.wher.k,
                                                                ),
                                                                ..default()
                                                            },
                                                            scene: enemy3d0.clone(),
                                                            ..default()
                                                        }
                                                    ).id(),
                                                ), 
                                                pl: crate::Place{
                                                    i: i.wher.i,
                                                    j: i.wher.j,
                                                    k: i.wher.k,
                                                }
                                            } 
                                        );}
                "city" => { game.enemies.push( 
                                            crate::Enemy{ 
                                                entity: Some(
                                                    commands.spawn(
                                                        SceneBundle {
                                                            transform: Transform {
                                                                translation: Vec3::new(
                                                                    i.wher.i,
                                                                    i.wher.j,
                                                                    i.wher.k,
                                                                ),
                                                                scale: Vec3::new(
                                                                    30.0,
                                                                    30.0,
                                                                    30.0,
                                                                ),
                                                                ..default()
                                                            },
                                                            scene: la_night_city.clone(),
                                                            ..default()
                                                        }
                                                    ).id(),
                                                ), 
                                                pl: crate::Place{
                                                    i: i.wher.i,
                                                    j: i.wher.j,
                                                    k: i.wher.k,
                                                }
                                            } 
                                        );}
                _ => {}
            }
        }
    }
}