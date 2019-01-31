use crate::domain::dto::create_todo::{CreateTodoInput, CreateTodoOutput};

pub trait ICreateTodo{
    fn create(&self, input: CreateTodoInput) -> CreateTodoOutput;
}
