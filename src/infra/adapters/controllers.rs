use crate::infra::adapters::usecase_bus::UsecaseBus;

pub mod create_todo;
pub mod show_todo;
pub mod todo;

pub trait Controller{
    fn new(usecase_bus: UsecaseBus) -> Self;
}

pub mod prelude{
    pub use super::create_todo::*;
    pub use super::show_todo::*;
    pub use super::todo::*;
    pub use super::Controller;
}
