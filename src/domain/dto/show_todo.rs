#[derive(Debug)]
pub struct ShowTodoOutput{
    todos: Vec<String>,
}

impl ShowTodoOutput{
    pub fn new(todos: Vec<String>) -> ShowTodoOutput{
        ShowTodoOutput{todos}
    }
}