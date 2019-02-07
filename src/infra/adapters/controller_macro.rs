// params: ControllerName
// ex) def_controller!(TodoController)
macro_rules! def_controller {
    ($i:ident) => {
        pub struct $i {
            usecase_bus: UsecaseBus,
        }

        impl Controller for $i {
            fn new(usecase_bus: UsecaseBus) -> Self {
                Self { usecase_bus }
            }
        }
    };
}

// params: usecase
// ex) def_use!(usecase1, usecase2)
macro_rules! def_use {
    (
        $($i:ident),*
    ) => {
        use crate::infra::adapters::controllers::Controller;
        use crate::infra::adapters::usecase_bus::UsecaseBus;
        use crate::domain::usecases::Handler;
        $(
            use crate::domain::usecases::$i::*;
        )*
    }
}
