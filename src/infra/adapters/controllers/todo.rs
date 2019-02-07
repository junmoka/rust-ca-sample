def_use!(create_todo, show_todo);
def_controller!(TodoController);

impl TodoController {
    pub fn create(&self, name: String) {
        let input = CreateTodoInput::new(name);
        self.usecase_bus.handle(input);
    }

    pub fn show(&self) {
        let v = self.usecase_bus.handle(ShowTodoInput {});
        println!("{:?}", v);
    }
}
