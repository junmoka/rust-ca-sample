use super::super::repositories::New;
use super::super::repositories::todo::*;
use super::{Usecase, Handler};
use rust_clean_architecture_derive::{New, UsecaseMacro};

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
#[derive(New, UsecaseMacro)]
pub struct ShowTodoImpl<T: ITodoRepository>{
    repository: T,
}

impl<T: ITodoRepository> Handler<ShowTodoInput, ShowTodoOutput> for ShowTodoImpl<T>{
    fn handle(&self, _input: ShowTodoInput) -> ShowTodoOutput{
        ShowTodoOutput::new(self.repository.show())
    }
}
