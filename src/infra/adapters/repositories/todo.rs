use crate::domain::repositories::New;
use crate::domain::repositories::todo::ITodoRepository;
use super::super::gateway::kvs::Kvs;

pub struct TodoRepositoryImpl<T: Kvs>{
    kvs: T,
}

impl<T: Kvs> ITodoRepository for TodoRepositoryImpl<T>{

    fn save(&self, name: String){
        self.kvs.write(name);
    }

    fn show(&self) -> Vec<String>{
        self.kvs.read()
    }
}

impl<T: Kvs> New for TodoRepositoryImpl<T>{
    fn new() -> Self{
        let kvs = T::new();
        TodoRepositoryImpl{ kvs }
    }
}