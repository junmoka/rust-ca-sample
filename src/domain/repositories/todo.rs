pub trait ITodoRepository{
    fn save(&self, name: String);
    fn show(&self) -> Vec<String>;
}
