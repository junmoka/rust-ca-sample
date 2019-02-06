def_use!(show_todo);
def_controller!(ShowTodoController);

impl ShowTodoController{
    pub fn show(&self){
        let v = self.usecase_bus.handle(ShowTodoInput{});
        println!("{:?}", v);
    }
}