pub mod ocean{

    use bevy::{ render::color::Color,
            ecs::system::Commands,
            prelude::*,
            render::mesh,
            render::mesh::*};
    
    type float = f32;

    pub struct Oceanpart{
        //pub entity: Option<Entity>,
        mesh: Mesh,
//        vert: Vec<[f32; 3]>,
//        indices: Vec<u32>,
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
            //self.mesh = Mesh::new(PrimitiveTopology::TriangleList);
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

    struct OceanWave{
        speed: float,
        ampl: float,
        x: float,
        z: float,
    }
    impl OceanWave{
        fn get( self,x: float,z: float )
        -> float
        {
            self.ampl * ( self.speed * (self.x * x + self.z * z) ).sin()
        }
    }

    #[derive(Component, Default)]
    pub struct Nmesh;
    #[derive(Component, Default)]
    pub struct Ocean{
        pub entity: Option<Entity>,
        pub mesh_c: Option<Oceanpart>,
        pub mesh: Option<Mesh>,
        time: f32,
        oceanwaves: Vec<OceanWave>,
    }
    impl Ocean{
        //pub fn create( mut self ){
        //    self.mesh_c = Some( Oceanpart::new() );
        //    self.mesh_c.unwrap().create( 10, 10, 1, 0.0, 0.0, );
        //}

        pub fn at(){}
        pub fn update(
            time: Res<Time>,
            mut query: Query<(&Transform, &Handle<Mesh>, With<Nmesh>)>,
            mut assets: ResMut<Assets<Mesh>>
        ){
            //self.time = time.elapsed_seconds();
            for (transform, handle, _) in query.iter(){
                if let Some( mut mesh ) = assets.get_mut(&handle){
                    let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
                    if let VertexAttributeValues::Float32x3(thing) = positions {
                        let mut temporary = Vec::new();
                        for i in thing {
                            let temp = Vec3::new( i[0], ( time.elapsed_seconds()+i[0]+i[2] ).sin(), i[2]);
                            temporary.push(temp);
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
            //let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
            //let mut vert = vec![];
            //let mut indices = Vec::new();
            //
            //for i in 0..=20 {
            //    for j in 0..=20 {
            //        let x = (i as f32 - 10.0) * 1.0;
            //        let z = (j as f32 - 10.0) * 1.0;
            //        vert.push([x,0.2*((x+z)*0.5).sin(),z]);
            //    }
            //}
            //for i in 0..=19 {
            //    for j in 0..=19 {
            //        let v0 = i * 21 + j;
            //        let v1 = i * 21 + j + 1;
            //        let v2 = (i + 1) * 21 + j;
            //        let v3 = (i + 1) * 21 + j + 1;
            //        indices.push(v0);
            //        indices.push(v1);
            //        indices.push(v2);
            //        indices.push(v2);
            //        indices.push(v1);
            //        indices.push(v3);
            //    }
            //}
            //mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION,vert);
            //mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 21*21]);
            //mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 21*21]);
            //mesh.set_indices(Some(mesh::Indices::U32(indices)));
//
            //commands.spawn(PbrBundle {
            //    mesh: meshes.add(mesh),
            //    material: materials.add(Color::rgb(0.0, 0.1, 0.3).into()),
            //    ..default()
            //});

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