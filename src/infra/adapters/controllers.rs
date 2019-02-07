use crate::infra::adapters::usecase_bus::UsecaseBus;

pub trait Controller {
    fn new(usecase_bus: UsecaseBus) -> Self;
}

include!(concat!(env!("OUT_DIR"), "/pub_controllers.rs"));

pub_mod_controllers!();

pub mod prelude {
    pub use super::Controller;
    pub_use_controllers!();
}
