use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Vehicle{
    pub pl:     crate::Place,
    pub entity: Option<Entity>,
    pub camera_y: f32,
    pub camera_x: f32,
}
impl Vehicle{
    pub fn new()
    -> Self{
        Vehicle{
            pl:  crate::Place::new(),
            entity: None,
            camera_y: 0.0,
            camera_x: 0.0,
        }
    }
    pub fn forward( &self )
    -> Vec3{
        Vec3::new( self.camera_y.cos() * self.camera_x.cos(), self.camera_x.sin(), - self.camera_y.sin() * self.camera_x.cos())
    }
    pub fn passenger( &self )
    -> Vec3{
        Vec3{ x: self.pl.i, y: self.pl.j+1.0, z: self.pl.k }
    }
}

