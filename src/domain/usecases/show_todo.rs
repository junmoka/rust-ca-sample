use super::super::repositories::todo::*;
use super::{Usecase, Handler};

pub struct ShowTodoInput;

#[derive(Debug)]
pub struct ShowTodoOutput{
    todos: Vec<String>,
}

impl ShowTodoOutput{
    pub fn new(todos: Vec<String>) -> Self{
        ShowTodoOutput{todos}
    }
}

// impl
pub struct ShowTodoImpl<T: ITodoRepository>{
    repository: T,
}

impl<T: ITodoRepository> ShowTodoImpl<T>{
    pub fn new(repository: T) -> Self{
        ShowTodoImpl{repository}
    }
}

impl<T: ITodoRepository> Usecase<ShowTodoInput, ShowTodoOutput> for ShowTodoImpl<T>{}

impl<T: ITodoRepository> Handler<ShowTodoInput, ShowTodoOutput> for ShowTodoImpl<T>{
    fn handle(&self, _input: ShowTodoInput) -> ShowTodoOutput{
        ShowTodoOutput::new(self.repository.show())
    }
}


