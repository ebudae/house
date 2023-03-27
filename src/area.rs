pub mod area{
    use bevy::prelude::*;

    #[derive(Component, Default)]
    pub struct Area{
        radius:f32,
        x:f32,
        z:f32,
    }
    impl Area{
        fn m(self,x:f32,z:f32){
            if ( x-self.x )*( x-self.x ) +( z-self.z )*( z-self.z ) < self.radius*self.radius{
                
            }
        }
    }
}
