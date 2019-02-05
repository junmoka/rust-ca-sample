use super::super::repositories::todo::*;
use super::Usecase;

pub struct ShowTodoInput;

#[derive(Debug)]
pub struct ShowTodoOutput{
    todos: Vec<String>,
}

impl ShowTodoOutput{
    pub fn new(todos: Vec<String>) -> ShowTodoOutput{
        ShowTodoOutput{todos}
    }
}

// impl
pub struct ShowTodoImpl<T: ITodoRepository>{
    repository: T,
}

impl<T: ITodoRepository> Usecase<ShowTodoInput, ShowTodoOutput> for ShowTodoImpl<T>{
    fn handle(&self, _input: ShowTodoInput) -> ShowTodoOutput{
        ShowTodoOutput::new(self.repository.show())
    }
}

impl<T: ITodoRepository> ShowTodoImpl<T>{
    pub fn new(repository: T) -> ShowTodoImpl<T>{
        ShowTodoImpl{repository}
    }
}