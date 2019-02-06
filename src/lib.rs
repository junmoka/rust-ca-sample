mod infra;
pub mod domain;

// default system is console
//pub use self::infra::console as system;

pub use self::infra::test as system;

// default system is web
//pub use self::infra::web as system;
