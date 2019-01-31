use crate::domain::usecases::create_todo::ICreateTodo;
use crate::domain::interactor::create_todo_impl::CreateTodoImpl;
use crate::domain::dto::create_todo::{CreateTodoInput, CreateTodoInputIF};
use crate::domain::repositories::todo::ITodoRepository;

pub struct CreateTodoController {
    usecase: Box<ICreateTodo>,
}

impl CreateTodoController{
    pub fn new(repository: Box<ITodoRepository>) -> CreateTodoController{
        CreateTodoController{usecase: Box::new(CreateTodoImpl::new(repository))}
    }

    pub fn create(&self, name: String){
        let input = CreateTodoInput::new(name);
        self.usecase.create(input);
    }
}
