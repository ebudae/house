pub mod k{
  
    use bevy::prelude::*;

    #[derive(Component, Default)]
    pub struct K{
        pub pl:     crate::Place,
        pub entity: Option<Entity>,
        pub busy:   Option<Entity>,
        pub camera_y: f32,
        pub camera_x: f32,
    }
    impl K{
        pub fn new()
        -> Self{
            K{
                pl: crate::Place::new(),
                entity: None,
                busy : None,
                camera_y: 0.0,
                camera_x: 0.0,
            }
        }
        pub fn forward( &self )
        -> Vec3{
            Vec3::new( - self.camera_y.cos() * self.camera_x.cos(), self.camera_x.sin(), self.camera_y.sin() * self.camera_x.cos())
        }
        pub fn accum_forward( &self )
        -> Vec3{
            Vec3::new( self.pl.i - self.camera_y.cos() * self.camera_x.cos(), self.pl.j + self.camera_x.sin(),self.pl. k + self.camera_y.sin() * self.camera_x.cos())
        }
        pub fn right( &self )
        -> Vec3{
            Vec3::new( self.camera_y.sin()* - self.camera_x.cos() , 0.0, - self.camera_y.cos() * self.camera_x.cos())
        }
        pub fn unbusy( &mut self ){
            self.busy = None;
        }
    }
      
  
}