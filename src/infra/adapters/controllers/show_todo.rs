use crate::domain::usecases::show_todo::{IShowTodo, ShowTodoImpl};
use crate::domain::repositories::todo::ITodoRepository;

pub struct ShowTodoController {
    usecase: Box<IShowTodo>,
}

impl ShowTodoController{
    pub fn new(repository: Box<ITodoRepository>) -> ShowTodoController{
        ShowTodoController{usecase: Box::new(ShowTodoImpl::new(repository))}
    }

    pub fn show(&self){
        let v = self.usecase.show();

        println!("{:?}", v);
    }
}
