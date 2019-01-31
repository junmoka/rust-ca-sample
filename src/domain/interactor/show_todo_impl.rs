use crate::domain::dto::show_todo::{ShowTodoOutput};
use crate::domain::usecases::show_todo::IShowTodo;
use crate::domain::repositories::todo::ITodoRepository;

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