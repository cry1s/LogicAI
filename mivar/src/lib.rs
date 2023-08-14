use js_sandbox::AnyError;
use thiserror::Error;

pub mod builder;
mod graph_solver;
pub mod model;
pub use model::KnowledgeBase;

mod utils;

pub type Result<T> = std::result::Result<T, KnowledgeBaseError>;

#[derive(Error, Debug)]
pub enum KnowledgeBaseError {
    #[error("This name already exists")]
    NameAlreadyExists,
    #[error("{0}")]
    BadCode(AnyError),
    #[error("Way not found")]
    SolveError,
    #[error("The number of arguments does not match")]
    ArgCountError,
}

#[cfg(test)]
mod tests;
