use crate::domain::repositories::todo::ITodoRepository;
//use crate::infra::db::kvs::KVS;
use crate::adapters::gateway::kvs::Kvs;

pub struct TodoRepositoryImpl{
    kvs: Box<Kvs>,
}

impl TodoRepositoryImpl{
    pub fn new(kvs: Box<Kvs>) -> TodoRepositoryImpl{
        TodoRepositoryImpl{ kvs }
    }
}

impl ITodoRepository for TodoRepositoryImpl{
    fn save(&self, name: String){
        self.kvs.write(name);
    }

    fn show(&self) -> Vec<String>{
        self.kvs.read()
    }
}