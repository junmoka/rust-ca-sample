use crate::infra::db::kvs::KvsImpl;
use crate::infra::adapters::repositories::todo::TodoRepositoryImpl;
use crate::domain::usecases::create_todo::CreateTodoImpl;
use crate::domain::usecases::show_todo::ShowTodoImpl;

//DI定義
// KVS
pub type DefaultKVS = KvsImpl;

// Repository
pub type DefaultTodoRepositoryImpl = TodoRepositoryImpl<DefaultKVS>;

// usecase
pub type DefaultCreateTodoImpl = CreateTodoImpl<DefaultTodoRepositoryImpl>;
pub type DefaultShowTodoImpl = ShowTodoImpl<DefaultTodoRepositoryImpl>;

