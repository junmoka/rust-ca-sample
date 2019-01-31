use crate::domain::dto::show_todo::{ShowTodoOutput};

pub trait IShowTodo{
    fn show(&self) -> ShowTodoOutput;
}
