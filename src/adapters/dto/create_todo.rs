use crate::infra::console::dto::create_todo::CTodoInput;
use crate::domain::dto::create_todo::CreateTodoInputIF;

impl CreateTodoInputIF for CTodoInput{
    fn name(&self) -> &String{
        &self.name
    }
}
