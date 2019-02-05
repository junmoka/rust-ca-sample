pub mod create_todo;
pub mod show_todo;

pub mod prelude{
    pub use super::create_todo::*;
    pub use super::show_todo::*;
}

pub trait Handler<Input, Output>{
    fn handle(&self, input: Input) -> Output;
}

pub trait Usecase<Input, Output>: Handler<Input, Output>{}
pub trait IUsecaseBus<Input, Output>: Handler<Input, Output>{}
