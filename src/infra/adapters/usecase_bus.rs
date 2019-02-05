use rust_clean_architecture_derive::UsecaseBusMacro;
use crate::domain::usecases::{Handler, IUsecaseBus};
use crate::domain::usecases::prelude::*;
use crate::infra::di::*;


#[derive(UsecaseBusMacro)]
pub struct UsecaseBus;
