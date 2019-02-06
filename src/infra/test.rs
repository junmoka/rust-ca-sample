#![allow(unused_imports)]

use crate::domain::usecases::IUsecaseBus;
use crate::infra::adapters::usecase_bus::*;
use crate::domain::usecases::create_todo::{CreateTodoInput};
use crate::domain::usecases::show_todo::{ShowTodoInput};

#[allow(dead_code)]
pub fn run(){
    //let bus = UsecaseBus{};
    //bus.handle(CreateTodoInput{name:"".to_string()});
    //bus.handle(ShowTodoInput{});
}