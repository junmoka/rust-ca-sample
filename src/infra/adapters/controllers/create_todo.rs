use crate::domain::usecases::Handler;
use crate::infra::adapters::usecase_bus::*;
use crate::domain::usecases::create_todo::*;

pub struct CreateTodoController{
    usecase_bus: UsecaseBus,
}

impl CreateTodoController{
    pub fn new(usecase_bus: UsecaseBus) -> CreateTodoController{
        CreateTodoController{usecase_bus}
    }

    pub fn create(&self, name: String){
        let input = CreateTodoInput::new(name);
        self.usecase_bus.handle(input);
    }
}
