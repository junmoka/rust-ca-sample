use crate::domain::usecases::create_todo::{ICreateTodo, CreateTodoInput};

pub struct CreateTodoController<T: ICreateTodo>{
    usecase: T,
}

impl<T: ICreateTodo> CreateTodoController<T>{
    pub fn new(usecase: T) -> CreateTodoController<T>{
        CreateTodoController{usecase}
    }

    pub fn create(&self, name: String){
        let input = CreateTodoInput::new(name);
        self.usecase.create(input);
    }
}
