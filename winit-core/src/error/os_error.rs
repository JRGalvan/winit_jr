use std::error::Error;

pub trait OsError: Error + Send + Sync + 'static {}
