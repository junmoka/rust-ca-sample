use lazy_static::lazy_static;
use rust_clean_architecture_derive::New;
use std::sync::Mutex;

use crate::domain::repositories::New;
use crate::infra::adapters::gateway::kvs::Kvs;

lazy_static! {
    pub static ref KVS: Mutex<Vec<String>> = Mutex::new(vec![]);
}

#[derive(New)]
pub struct KvsImpl;

impl Kvs for KvsImpl {
    fn read(&self) -> Vec<String> {
        KVS.lock().unwrap().clone()
    }

    fn write(&self, val: String) {
        KVS.lock().unwrap().push(val);
    }
}
