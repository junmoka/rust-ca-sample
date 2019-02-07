use super::New;

pub trait ITodoRepository: New {
    fn save(&self, name: String);
    fn show(&self) -> Vec<String>;
}
