pub trait Kvs{
    fn read(&self) -> Vec<String>;
    fn write(&self, val: String);
}