def_use!(create_todo);
def_controller!(CreateTodoController);

impl CreateTodoController {
    pub fn create(&self, name: String) {
        let input = CreateTodoInput::new(name);
        self.usecase_bus.handle(input);
    }
}
