use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::adapters::gateway::kvs::Kvs;

lazy_static! {
  pub static ref KVS: Mutex<Vec<String>> = Mutex::new(vec![]);
}

pub struct KvsImpl;

impl Kvs for KvsImpl{
    fn read(&self) -> Vec<String>{
        KVS.lock().unwrap().clone()
    }

    fn write(&self, val: String){
        KVS.lock().unwrap().push(val);
    }
}
