pub mod place{
    use bevy::prelude::*;
    use rand::Rng;

    #[derive(Component, Default, Clone)]
    pub struct Place{
        pub i: f32,
        pub j: f32,
        pub k: f32,
    }
    impl Place{
    #[allow(dead_code)]
        pub fn new()
        -> Self{
            Place{
                i: 0.0,
                j: 0.0,
                k: 0.0,
            } 
        }
        pub fn rand()
        -> Place{
            Place{
                i: 50.0 + rand::thread_rng().gen_range(-40.0..40.0),
                j: 0.0,
                k: rand::thread_rng().gen_range(-40.0..40.0),
            } 
        }
        pub fn from_vec3( &mut self, k:Vec3 ){
            self.i = k.x;
            self.j = k.y;
            self.k = k.z;
        }
        pub fn to_vec3( &self )
        -> Vec3{
            Vec3::new( self.i, self.j, self.k )
        }
    }
}
