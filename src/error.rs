use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("Usage: subbox <tesla.com>")]
    CliUsage,
}