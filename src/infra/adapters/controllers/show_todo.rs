use crate::infra::adapters::controller_macro;

use crate::domain::usecases::Handler;
use crate::infra::adapters::usecase_bus::*;
use crate::domain::usecases::show_todo::*;

use super::Controller;


def_controller!(ShowTodoController);

impl ShowTodoController{
    pub fn show(&self){
        let v = self.usecase_bus.handle(ShowTodoInput{});

        println!("{:?}", v);
    }
}