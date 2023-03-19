mod ocean{
#[derive(Component, Default)]
struct Ocean{
    entity: Option<Entity>,
}
impl Ocean{
    fn create(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ){
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let mut vert = vec![];
        let mut indices = Vec::new();
        struct Vertex {
            x: f32,
            y: f32,
            z: f32,
        }
        for i in 0..=20 {
            for j in 0..=20 {
                let x = (i as f32 - 10.0) * 1.0;
                let z = (j as f32 - 10.0) * 1.0;
                vert.push([x,0.2*((x+z)*0.2).sin(),z]);
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

        commands.spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(0.0, 0.1, 0.3).into()),
            ..default()
        });
    }
    fn get_level(
        time: Res<Time>
    )
    -> f32{
        0.0
    }
}
}