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
pub struct CreateTodoImpl{
    repository: Box<ITodoRepository>,
}

impl ICreateTodo for CreateTodoImpl{
    fn create(&self, input: CreateTodoInput) -> CreateTodoOutput{
        self.repository.save(input.name.clone());
        CreateTodoOutput::new(input.name.clone())
    }
}

impl CreateTodoImpl{
    pub fn new(repository: Box<ITodoRepository>) -> CreateTodoImpl{
        CreateTodoImpl{repository}
    }
}

pub type DefaultCreateTodoImpl = CreateTodoImpl;