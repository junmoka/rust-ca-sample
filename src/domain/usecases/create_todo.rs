use super::super::repositories::New;
use super::super::repositories::todo::*;
use super::{Usecase, Handler};

use rust_clean_architecture_derive::UsecaseMacro;

// IO
pub struct CreateTodoInput{
    pub name: String,
}

impl CreateTodoInput{
    pub fn new(name: String) -> Self{
        CreateTodoInput{name}
    }
}

pub struct CreateTodoOutput{
    pub name: String,
}

impl CreateTodoOutput{
    pub fn new(name: String) -> Self{
        CreateTodoOutput{name}
    }
}

//impl
#[derive(UsecaseMacro)]
pub struct CreateTodoImpl<T: ITodoRepository, T2: ITodoRepository>{
    repository: T,
    repository2: T2,
}

impl<T: ITodoRepository, T2: ITodoRepository> Handler<CreateTodoInput, CreateTodoOutput> for CreateTodoImpl<T, T2>{
    fn handle(&self, input: CreateTodoInput) -> CreateTodoOutput{
        self.repository.save(input.name.clone());
        CreateTodoOutput::new(input.name.clone())
    }
}
