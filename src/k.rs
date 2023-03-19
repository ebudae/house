mod k{
  
  #[derive(Component, Default)]
  struct K{
      pl:     Place,
      entity: Option<Entity>,
      busy:   Option<Entity>,
      camera_y: f32,
      camera_x: f32,
  }
  impl K{
      fn new()
      -> Self{
          K{
              pl: Place::new(),
              entity: None,
              busy : None,
              camera_y: 0.0,
              camera_x: 0.0,
          }
      }
      fn forward( &self )
      -> Vec3{
          Vec3::new( - self.camera_y.cos() * self.camera_x.cos(), self.camera_x.sin(), self.camera_y.sin() * self.camera_x.cos())
      }
      fn accum_forward( &self )
      -> Vec3{
          Vec3::new( self.pl.i - self.camera_y.cos() * self.camera_x.cos(), self.pl.j + self.camera_x.sin(),self.pl. k + self.camera_y.sin() * self.camera_x.cos())
      }
      fn right( &self )
      -> Vec3{
          Vec3::new( self.camera_y.sin()* - self.camera_x.cos() , 0.0, - self.camera_y.cos() * self.camera_x.cos())
      }
      fn unbusy( &mut self ){
          self.busy = None;
      }
  }
  
  
}