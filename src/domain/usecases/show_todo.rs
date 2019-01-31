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
pub struct ShowTodoImpl{
    repository: Box<ITodoRepository>,
}

impl IShowTodo for ShowTodoImpl{
    fn show(&self) -> ShowTodoOutput{
        ShowTodoOutput::new(self.repository.show())
    }
}

impl ShowTodoImpl{
    pub fn new(repository: Box<ITodoRepository>) -> ShowTodoImpl{
        ShowTodoImpl{repository}
    }
}