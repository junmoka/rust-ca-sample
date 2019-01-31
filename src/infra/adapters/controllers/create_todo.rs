use crate::domain::usecases::create_todo::{ICreateTodo, DefaultCreateTodoImpl, CreateTodoInput};
use crate::domain::repositories::todo::ITodoRepository;

pub struct CreateTodoController {
    usecase: Box<ICreateTodo>,
}

impl CreateTodoController{
    pub fn new(repository: Box<ITodoRepository>) -> CreateTodoController{
        CreateTodoController{usecase: Box::new(DefaultCreateTodoImpl::new(repository))}
    }

    pub fn create(&self, name: String){
        let input = CreateTodoInput::new(name);
        self.usecase.create(input);
    }
}
