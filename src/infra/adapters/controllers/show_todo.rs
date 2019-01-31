use crate::domain::usecases::show_todo::IShowTodo;

pub struct ShowTodoController<T: IShowTodo> {
    usecase: T,
}

impl<T: IShowTodo> ShowTodoController<T>{
    pub fn new(usecase: T) -> ShowTodoController<T>{
        ShowTodoController{usecase}
    }

    pub fn show(&self){
        let v = self.usecase.show();

        println!("{:?}", v);
    }
}
