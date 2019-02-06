use crate::infra::adapters::controller_macro;

use crate::infra::adapters::usecase_bus::*;
use crate::infra::adapters::controllers::{Controller};
use crate::domain::usecases::Handler;
use crate::domain::usecases::create_todo::*;

def_controller!(CreateTodoController);

impl CreateTodoController{
    pub fn create(&self, name: String){
        let input = CreateTodoInput::new(name);
        self.usecase_bus.handle(input);
        println!("a");
    }
}