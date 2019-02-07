use super::repositories::New;

pub trait Handler<Input, Output> {
    fn handle(&self, input: Input) -> Output;
}

pub trait Usecase<Input, Output>: Handler<Input, Output> + New {}
pub trait IUsecaseBus<Input, Output>: Handler<Input, Output> {}

include!(concat!(env!("OUT_DIR"), "/pub_usecases.rs"));

pub_mod_usecases!();

pub mod prelude {
    pub use super::Handler;
    pub use super::IUsecaseBus;
    pub use super::Usecase;
    pub_use_usecases!();
}
