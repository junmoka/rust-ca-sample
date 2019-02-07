pub mod domain;
mod infra;

// default system is console
pub use self::infra::system::console as system;

//pub use self::infra::system::test as system;

// default system is web
//pub use self::infra::system::web as system;
