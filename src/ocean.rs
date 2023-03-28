pub mod ocean{

    use bevy::{ render::color::Color,
            ecs::system::Commands,
            prelude::*,
            render::mesh,
            render::mesh::*};
    
    type Float = f32;

    pub struct Oceanpart{
        mesh: Mesh,
        x: u32,
        z: u32,
    }
    impl Oceanpart{
        pub fn new()
        -> Self
        {
            Oceanpart{
                mesh: Mesh::new(PrimitiveTopology::TriangleList) ,
                x: 0,
                z: 0,
            }
        }
        pub fn create( &mut self, x: u32, z: u32, k: u32, x_offset: f32, z_offset: f32 )
        {
            let mut vert = vec![];
            let mut indices = Vec::new();
            
            self.z  = z;
            self.x  = x;
            
            for i in 0..= ( 2*x ) {
                for j in 0..= ( 2*z ) {
                    let x = (i as f32 - (k*x) as f32) * k as f32;
                    let z = (j as f32 - (k*z) as f32) * k as f32;
                    vert.push([x+x_offset,0.2*((x+x_offset+z+z_offset)*0.5).sin(),z+z_offset]);
                }
            }
            for i in 0.. ( 2*x ) {
                for j in 0.. ( 2*z ) {
                    let v0 = i * ( 2*z+1 ) + j;
                    let v1 = i * ( 2*z+1 ) + j + 1;
                    let v2 = (i + 1) * ( 2*z+1 ) + j;
                    let v3 = (i + 1) * ( 2*z+1 ) + j + 1;
                    indices.push(v0);
                    indices.push(v1);
                    indices.push(v2);
                    indices.push(v2);
                    indices.push(v1);
                    indices.push(v3);
                }
            }

            self.mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION,vert);
            self.mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; (( 2*x+1 )*( 2*z+1 )).try_into().unwrap()]);
            self.mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; (( 2*x+1 )*( 2*z+1 )).try_into().unwrap()]);
            self.mesh.set_indices(Some(mesh::Indices::U32(indices)));

        }
    }

    #[derive(Clone,Copy)]
    pub struct OceanWave{
        pub speed: Float,
        pub ampl: Float,
        pub x: Float,
        pub z: Float,
    }
    impl OceanWave{
        pub fn new()
        -> Self
        {
            OceanWave{
                speed: 0.5,
                ampl: 0.1,
                x: 1.0,
                z: 1.0,
            }
        }
        pub fn get( self,x: Float,z: Float,t:Float )
        -> Float
        {
            self.ampl * ( self.speed * (self.x*x+ self.z*z+t) ).sin()
        }
    }

    #[derive(Component, Default)]
    pub struct Nmesh;
    #[derive(Component, Default)]
    pub struct Ocean{
        pub entity: Option<Entity>,
        pub mesh_c: Option<Oceanpart>,
        pub mesh: Option<Mesh>,
    }
    impl Ocean{

        //pub fn at(){}
        pub fn update(
            time: Res<Time>,
            game: Res<crate::Game>,
            mut assets: ResMut<Assets<Mesh>>,
            query: Query<(&Handle<Mesh>, With<Nmesh>)>,
        ){
            for (handle, _) in query.iter(){
                if let Some( mesh ) = assets.get_mut(&handle){
                    let p = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
                    if let VertexAttributeValues::Float32x3(thing) = p {
                        let mut temporary = Vec::new();
                        for i in thing {
                            //let mut h = 0.0;
                            //for e in &game.oceanwaves{
                            //    h =h + e .get( i[0], i[2],time.elapsed_seconds() );
                            //}
                            temporary.push(Vec3::new( i[0],game.get_ocean_waves_level(i[0],i[2],time.elapsed_seconds()), i[2]));
                        }
                        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, temporary);
                    }
                }
            }
        }
        pub fn register(
            mut commands: Commands,
            mut meshes: ResMut<Assets<Mesh>>,
            mut materials: ResMut<Assets<StandardMaterial>>,
        ){
            let mut mesh_c = Oceanpart::new();
            mesh_c.create( 10, 10, 1, 0.0, 0.0, );
            commands.spawn((PbrBundle {
                mesh: meshes.add(mesh_c.mesh),
                material: materials.add(Color::rgb(0.0, 0.1, 0.3).into()),
                ..default()
            },Nmesh));
            let mut mesh_c = Oceanpart::new();
            mesh_c.create( 50, 10, 1, 60.0, 0.0, );
            commands.spawn((PbrBundle {
                mesh: meshes.add(mesh_c.mesh),
                material: materials.add(Color::rgb(0.0, 0.8, 0.5).into()),
                ..default()
            },Nmesh));
            let mut mesh_c = Oceanpart::new();
            mesh_c.create( 50, 10, 1, -60.0, 0.0, );
            commands.spawn((PbrBundle {
                mesh: meshes.add(mesh_c.mesh),
                material: materials.add(Color::rgb(0.5, 0.3, 0.5).into()),
                ..default()
            },Nmesh));
            let mut mesh_c = Oceanpart::new();
            mesh_c.create( 10, 50, 1, 0.0, 60.0, );
            commands.spawn((PbrBundle {
                mesh: meshes.add(mesh_c.mesh),
                material: materials.add(Color::rgb(0.9, 0.1, 0.5).into()),
                ..default()
            },Nmesh));
            let mut mesh_c = Oceanpart::new();
            mesh_c.create( 10, 50, 1, 0.0, -60.0, );
            commands.spawn((PbrBundle {
                mesh: meshes.add(mesh_c.mesh),
                material: materials.add(Color::rgb(0.0, 0.0, 0.5).into()),
                ..default()
            },Nmesh));

            let mut mesh_c = Oceanpart::new();
            mesh_c.create( 50, 50, 1, 60.0, 60.0, );
            commands.spawn((PbrBundle {
                mesh: meshes.add(mesh_c.mesh),
                material: materials.add(Color::rgb(0.0, 0.3, 0.5).into()),
                ..default()
            },Nmesh));
            let mut mesh_c = Oceanpart::new();
            mesh_c.create( 50, 50, 1, -60.0, 60.0, );
            commands.spawn((PbrBundle {
                mesh: meshes.add(mesh_c.mesh),
                material: materials.add(Color::rgb(0.0, 0.3, 0.5).into()),
                ..default()
            },Nmesh));
            let mut mesh_c = Oceanpart::new();
            mesh_c.create( 50, 50, 1, -60.0, -60.0, );
            commands.spawn((PbrBundle {
                mesh: meshes.add(mesh_c.mesh),
                material: materials.add(Color::rgb(0.0, 0.3, 0.5).into()),
                ..default()
            },Nmesh));
            let mut mesh_c = Oceanpart::new();
            mesh_c.create( 50, 50, 1, 60.0, -60.0, );
            commands.spawn((PbrBundle {
                mesh: meshes.add(mesh_c.mesh),
                material: materials.add(Color::rgb(0.0, 0.3, 0.5).into()),
                ..default()
            },Nmesh));
        }
    }
}