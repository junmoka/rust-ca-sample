use crate::domain::repositories::New;

pub trait Kvs: New {
    fn read(&self) -> Vec<String>;
    fn write(&self, val: String);
}
