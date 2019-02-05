use rust_clean_architecture_derive::UsecaseBusMacro;
use crate::domain::usecases::Usecase;
use crate::domain::usecases::prelude::*;
use crate::infra::di::*;

pub trait IUsecaseBus<Input, Output>{
    fn handle(&self, input: Input) -> Output;
}

#[derive(UsecaseBusMacro)]
pub struct UsecaseBus;
