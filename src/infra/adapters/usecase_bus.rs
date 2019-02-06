use rust_clean_architecture_derive::UsecaseBusMacro;

use crate::domain::repositories::New;
use crate::domain::usecases::prelude::*;
use crate::infra::di::*;

#[derive(UsecaseBusMacro)]
pub struct UsecaseBus;
