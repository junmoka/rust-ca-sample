macro_rules! def_controller {
    ($e:ident) => {
        pub struct $e {
            usecase_bus: UsecaseBus,
        }

        impl Controller for $e{
            fn new(usecase_bus: UsecaseBus) -> Self{
                Self{usecase_bus}
            }
        }
    }
}