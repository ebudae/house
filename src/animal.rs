pub mod animal{
    use bevy::prelude::*;

    #[derive(Component, Default)]
    pub struct Animal{}
    trait IAnimal{
        fn update(){}
    }

    #[derive(Component, Default)]
    pub struct Fish{
        pub pl:     crate::place::place::Place,
        pub entity: Option<Entity>,
    }
    impl Fish{
        //pub fn new()
        //-> Self{
        //    Fish{
        //        pl:  crate::place::place::Place::new(),
        //        entity: None,
        //    }
        //}
    }
    impl IAnimal for Fish{
        fn update(){}
    }

    #[derive(Component, Default)]
    pub struct Bir{
        pub pl:     crate::place::place::Place,
        pub entity: Option<Entity>,
    }
    impl Bir{
        //pub fn new()
        //-> Self{
        //    Bir{
        //        pl:  crate::place::place::Place::new(),
        //        entity: None,
        //    }
        //}
    }
    impl IAnimal for Bir{
        fn update(){}
    }
}
