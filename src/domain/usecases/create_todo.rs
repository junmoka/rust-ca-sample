use super::super::repositories::todo::*;

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

//usecase
pub trait ICreateTodo{
    fn create(&self, input: CreateTodoInput) -> CreateTodoOutput;
}

//impl
pub struct CreateTodoImpl<T: ITodoRepository>{
    repository: T,
}

impl<T: ITodoRepository> ICreateTodo for CreateTodoImpl<T>{
    fn create(&self, input: CreateTodoInput) -> CreateTodoOutput{
        self.repository.save(input.name.clone());
        CreateTodoOutput::new(input.name.clone())
    }
}

impl<T: ITodoRepository> CreateTodoImpl<T>{
    pub fn new(repository: T) -> CreateTodoImpl<T>{
        CreateTodoImpl{repository}
    }
}