use std::convert::Infallible;

use crate::{
    env::Environment,
    exec::{Execute, ExecutionResult},
};

#[derive(Debug)]
pub enum Builtin {
    Exit,
}

impl Execute for Builtin {
    type Error = Infallible;

    fn execute(&self, _env: &mut Environment) -> Result<ExecutionResult, Self::Error> {
        match self {
            Self::Exit => Ok(ExecutionResult::Exit(0)),
        }
    }
}
