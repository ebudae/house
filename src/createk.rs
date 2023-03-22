pub mod createk{
    use bevy::ecs::system::Commands;
    use bevy::{prelude::*};

    pub fn createk(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut game: ResMut<crate::Game>,
    ){
        let enemy3d = asset_server.load("grand_piano_and_stool.glb#Scene0");
        //let enemy3d = asset_server.load("toon_cat_free.glb#Scene0");
        let enemy3d0 = asset_server.load("low-poly_fruit_box_assets.glb#Scene0");
        let la_night_city = asset_server.load("la_night_city.glb#Scene0");
        let tropical_island = asset_server.load("tropical_island.glb#Scene0");
        let question_mark = asset_server.load("question_mark .glb#Scene0");
        //let mossy_rusty_sheet = asset_server.load("mossy_rusty_sheet.glb#Scene0");
        let concrete_barrier = asset_server.load("concrete_barrier.glb#Scene0");
        //let fantasy_town = asset_server.load("fantasy_town.glb#Scene0");
        //let simple_wood_planks_debris_pack = asset_server.load("simple_wood_planks_debris_pack.glb#Scene0");
        let q63dae3c872f63b7131e032b6 = asset_server.load(r"girl-woman-glb-animated\source\63dae3c872f63b7131e032b6.glb#Scene0");
        let nokia = asset_server.load("nokia.glb#Scene0");
        struct WhatToCreate{
            what: String,
            wher: crate::Place,
        }

        let elems: Vec<WhatToCreate> = vec![
            WhatToCreate{ what:  "q63dae3c872f63b7131e032b6".to_string(), wher: crate::Place{ i:3.0, j:0.0, k:2.0 } },
            WhatToCreate{ what: "enemy".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "enemy0".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "question_mark".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "question_mark".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "question_mark".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "question_mark".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "question_mark".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "question_mark".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "question_mark".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "question_mark".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "question_mark".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "question_mark".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "question_mark".to_string(), wher: crate::Place::rand() },
            WhatToCreate{ what: "nokia".to_string(), wher: crate::Place{ i:3.0, j:0.0, k:2.0 } },
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
                                                                translation: Vec3::new(i.wher.i,i.wher.j,i.wher.k),
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
                                                                translation: Vec3::new(i.wher.i,i.wher.j,i.wher.k),
                                                                scale: Vec3::new(30.0,30.0,30.0,),
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
                "tropical_island" => { game.enemies.push( 
                                            crate::Enemy{ 
                                                entity: Some(
                                                    commands.spawn(
                                                        SceneBundle {
                                                            transform: Transform {
                                                                translation: Vec3::new(i.wher.i,i.wher.j,i.wher.k),
                                                                ..default()
                                                            },
                                                            scene: tropical_island.clone(),
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
                "question_mark" => { game.enemies.push( 
                                            crate::Enemy{ 
                                                entity: Some(
                                                    commands.spawn(
                                                        SceneBundle {
                                                            transform: Transform {
                                                                translation: Vec3::new(i.wher.i,i.wher.j,i.wher.k),
                                                                ..default()
                                                            },
                                                            scene: question_mark.clone(),
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
                "concrete_barrier" => { game.enemies.push( 
                                            crate::Enemy{ 
                                                entity: Some(
                                                    commands.spawn(
                                                        SceneBundle {
                                                            transform: Transform {
                                                                translation: Vec3::new(i.wher.i,i.wher.j,i.wher.k),
                                                                ..default()
                                                            },
                                                            scene: concrete_barrier.clone(),
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
                "nokia" => { game.enemies.push( 
                                            crate::Enemy{ 
                                                entity: Some(
                                                    commands.spawn(
                                                        SceneBundle {
                                                            transform: Transform {
                                                                translation: Vec3::new(i.wher.i,i.wher.j,i.wher.k),
                                                                ..default()
                                                            },
                                                            scene: nokia.clone(),
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
                "q63dae3c872f63b7131e032b6" => { game.enemies.push( 
                                            crate::Enemy{ 
                                                entity: Some(
                                                    commands.spawn(
                                                        SceneBundle {
                                                            transform: Transform {
                                                                translation: Vec3::new(i.wher.i,i.wher.j,i.wher.k),
                                                                ..default()
                                                            },
                                                            scene: q63dae3c872f63b7131e032b6.clone(),
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
