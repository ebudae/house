pub mod createsand{

    use bevy::{ render::color::Color,
            ecs::system::Commands,
            prelude::*,
            render::mesh,
            render::mesh::*};
    
    #[derive(Component, Default)]
    pub struct Qmesh;
    pub fn createsand(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut game: ResMut<crate::Game>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ){
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let mut vert = vec![];
        let mut indices = Vec::new();
        
        for i in 0..=20 {
            for j in 0..=20 {
                let x = (i as f32 - 10.0) * 1.0;
                let z = (j as f32 - 10.0) * 1.0;
                if x == 0.0 && z == 0.0 {
                    vert.push([x,3.0,z]);
                }else{
                    vert.push([x*6.0,
                    ((x/2.0).powi(2) + (z/2.0).powi(2)).sqrt().sin() / 
                    ((x/2.0).powi(2) + (z/2.0).powi(2)).sqrt() * 3.0 +
                    1.0 / ((x).powi(2) + (z).powi(2)).sqrt() + 0.2
                        ,z*6.0]);
                }
            }
        }
        for i in 0..=19 {
            for j in 0..=19 {
                let v0 = i * 21 + j;
                let v1 = i * 21 + j + 1;
                let v2 = (i + 1) * 21 + j;
                let v3 = (i + 1) * 21 + j + 1;
                indices.push(v0);
                indices.push(v1);
                indices.push(v2);
                indices.push(v2);
                indices.push(v1);
                indices.push(v3);
            }
        }
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION,vert);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 21*21]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 21*21]);
        mesh.set_indices(Some(mesh::Indices::U32(indices)));

        commands.spawn((PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(0.5, 0.5, 0.3).into()),
            ..default()
        },Qmesh));
    }
}