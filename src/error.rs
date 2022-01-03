use std::any::Any;

pub use anyhow::Result;

use bellperson::SynthesisError;

/// Custom error types
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unclassified error: {}", _0)]
    Unclassified(String),
    #[error("Invalid parameters file: {}", _0)]
    InvalidParameters(String),
    #[error("No useful post server for now")]
    NoUsefulPostServer,
    #[error("post server {} is not reachable with error: {}", _0, _1)]
    PostServerNotReachable(String, String),
    #[error("no task running on this server")]
    NoTaskRunningOnSever,
    #[error("Task is still running, not completed")]
    TaskStillRunning,
    #[error("task failed with error: {}", _0)]
    TaskFailedWithError(String),
    #[error("retried times limited with last error: {}", _0)]
    TriedTimesLimitedWithLastError(String),
}

impl From<Box<dyn Any + Send>> for Error {
    fn from(inner: Box<dyn Any + Send>) -> Error {
        Error::Unclassified(format!("{:?}", dbg!(inner)))
    }
}