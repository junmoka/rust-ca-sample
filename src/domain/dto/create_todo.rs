pub struct CreateTodoInput{
    pub name: String,
}

impl CreateTodoInput{
    pub fn new(name: String) -> CreateTodoInput{
        CreateTodoInput{name}
    }
}

pub struct CreateTodoOutput{
    pub name: String,
}

impl CreateTodoOutput{
    pub fn new(name: String) -> CreateTodoOutput{
        CreateTodoOutput{name}
    }
}


pub trait CreateTodoInputIF{
    fn name(&self) -> &String;
}