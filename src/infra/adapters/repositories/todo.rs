use crate::domain::repositories::todo::ITodoRepository;
use crate::domain::repositories::New;
use crate::infra::adapters::gateway::kvs::Kvs;
use rust_clean_architecture_derive::New;

#[derive(New)]
pub struct TodoRepositoryImpl<T: Kvs> {
    kvs: T,
}

impl<T: Kvs> ITodoRepository for TodoRepositoryImpl<T> {
    fn save(&self, name: String) {
        self.kvs.write(name);
    }

    fn show(&self) -> Vec<String> {
        self.kvs.read()
    }
}
