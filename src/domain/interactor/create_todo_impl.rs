use crate::domain::dto::create_todo::{CreateTodoInput, CreateTodoOutput};
use crate::domain::usecases::create_todo::ICreateTodo;
use crate::domain::repositories::todo::ITodoRepository;

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