use std::env::VarError;
use std::io;
use std::net::AddrParseError;
use derive_more::{Display, From};

#[derive(Debug, From, Display)]
pub enum Error {
    NamedVar(NamedVarError),
    AddrParse(AddrParseError),
    Io(io::Error),
}

#[derive(Debug, Display)]
#[display("{} (named {})", error, name)]
pub struct NamedVarError {
    pub error: VarError,
    pub name: &'static str,
}

pub trait IntoNamedVarError<T> {
    fn named(self, name: &'static str) -> Result<T, NamedVarError>;
}

impl<T> IntoNamedVarError<T> for Result<T, VarError> {
    fn named(self, name: &'static str) -> Result<T, NamedVarError> {
        self.map_err(|error| NamedVarError { error, name })
    }
}

pub(crate) type Res<T> = Result<T, Error>;