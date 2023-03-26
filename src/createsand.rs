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
        mut images: ResMut<Assets<Image>>
    ){
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let mut vert = vec![];
        let mut uvs = vec![];
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
                uvs.push( [ x*6.0, z* 6.0 ] );
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
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(mesh::Indices::U32(indices)));
        
        let m = asset_server.load("0012-beach-sand-texture-seamless-hr.jpg");
        //images.get_mut( &m ).unwrap().sampler_descriptor.address_mode_u = AddressMode::Repeat;
        let mut image = images.get_mut( &m );
    if let Some(image) = image {
        let mut i = bevy::render::texture::ImageSampler::nearest_descriptor();
        i.address_mode_u = bevy::render::render_resource::AddressMode::Repeat;
        i.address_mode_v = bevy::render::render_resource::AddressMode::Repeat;
        i.address_mode_w = bevy::render::render_resource::AddressMode::Repeat;
        image.sampler_descriptor = bevy::render::texture::ImageSampler::Descriptor( i );
    }
        let mut material = StandardMaterial::from( m );
        material.base_color = Color::rgb(0.5, 0.5, 0.3);
        material.reflectance = 0.0;
        
        commands.spawn((PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(material),
            ..default()
        },Qmesh));
    }
}