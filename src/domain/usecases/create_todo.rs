use super::super::repositories::todo::*;
use super::Usecase;

// IO
pub struct CreateTodoInput{
    pub name: String,
}

impl CreateTodoInput{
    pub fn new(name: String) -> CreateTodoInput{
        CreateTodoInput{name}
    }
}

pub struct CreateTodoOutput{
    pub name: String,
}

impl CreateTodoOutput{
    pub fn new(name: String) -> CreateTodoOutput{
        CreateTodoOutput{name}
    }
}

//impl
pub struct CreateTodoImpl<T: ITodoRepository>{
    repository: T,
}

impl<T: ITodoRepository> Usecase<CreateTodoInput, CreateTodoOutput> for CreateTodoImpl<T>{
    fn handle(&self, input: CreateTodoInput) -> CreateTodoOutput{
        self.repository.save(input.name.clone());
        CreateTodoOutput::new(input.name.clone())
    }
}

impl<T: ITodoRepository> CreateTodoImpl<T>{
    pub fn new(repository: T) -> CreateTodoImpl<T>{
        CreateTodoImpl{repository}
    }
}