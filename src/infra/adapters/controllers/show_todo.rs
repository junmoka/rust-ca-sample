use crate::domain::usecases::Handler;
use crate::infra::adapters::usecase_bus::*;
use crate::domain::usecases::show_todo::*;

pub struct ShowTodoController {
    usecase_bus: UsecaseBus,
}

impl ShowTodoController{
    pub fn new(usecase_bus: UsecaseBus) -> Self{
        ShowTodoController{usecase_bus}
    }

    pub fn show(&self){
        let v = self.usecase_bus.handle(ShowTodoInput{});

        println!("{:?}", v);
    }
}
