mod vehicle {
  
#[derive(Component, Default)]
struct Vehicle{
    pl:     Place,
    entity: Option<Entity>,
    camera_y: f32,
    camera_x: f32,
}
impl Vehicle{
    fn new()
    -> Self{
        Vehicle{
            pl: Place::new(),
            entity: None,
            camera_y: 0.0,
            camera_x: 0.0,
        }
    }
    fn forward( &self )
    -> Vec3{
        Vec3::new( self.camera_y.cos() * self.camera_x.cos(), self.camera_x.sin(), - self.camera_y.sin() * self.camera_x.cos())
    }
    fn passenger( &self )
    -> Vec3{
        Vec3{ x: self.pl.i, y: self.pl.j+1.0, z: self.pl.k }
    }
}

}