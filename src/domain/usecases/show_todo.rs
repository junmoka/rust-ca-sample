use super::super::repositories::todo::*;

#[derive(Debug)]
pub struct ShowTodoOutput{
    todos: Vec<String>,
}

impl ShowTodoOutput{
    pub fn new(todos: Vec<String>) -> ShowTodoOutput{
        ShowTodoOutput{todos}
    }
}

pub trait IShowTodo{
    fn show(&self) -> ShowTodoOutput;
}

// impl
pub struct ShowTodoImpl<T: ITodoRepository>{
    repository: T,
}

impl<T: ITodoRepository> IShowTodo for ShowTodoImpl<T>{
    fn show(&self) -> ShowTodoOutput{
        ShowTodoOutput::new(self.repository.show())
    }
}

impl<T: ITodoRepository> ShowTodoImpl<T>{
    pub fn new(repository: T) -> ShowTodoImpl<T>{
        ShowTodoImpl{repository}
    }
}