use crate::infra::db::kvs::KvsImpl;

use crate::domain::usecases::prelude::*;
use crate::infra::adapters::repositories::prelude::*;

//DI定義
// KVS
pub type DefaultKVS = KvsImpl;

// Repository
pub type DefaultTodoRepositoryImpl = TodoRepositoryImpl<DefaultKVS>;

// usecase
pub type DefaultCreateTodoImpl =
    CreateTodoImpl<DefaultTodoRepositoryImpl, DefaultTodoRepositoryImpl>;
pub type DefaultShowTodoImpl = ShowTodoImpl<DefaultTodoRepositoryImpl>;
