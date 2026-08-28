use crate::{
    builtin::Builtin,
    env::{Args, Environment},
    exec::{Execute, ExecutionResult},
};
use thiserror::Error;

#[derive(Debug)]
pub enum Command {
    Builtin(Builtin),
}

impl Command {
    pub fn from_args(args: &mut Args<'_>) -> Result<Option<Self>, CommandError> {
        let Some(name) = args.next() else {
            return Ok(None);
        };

        match name {
            "exit" => Ok(Some(Self::Builtin(Builtin::Exit))),
            _ => Err(CommandError::NotFound(name.into())),
        }
    }
}

impl Execute for Command {
    type Error = CommandError;

    fn execute(&self, env: &mut Environment) -> Result<ExecutionResult, Self::Error> {
        match self {
            Self::Builtin(builtin) => match builtin.execute(env) {
                Ok(result) => Ok(result),
                Err(never) => match never {},
            },
        }
    }
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("{0}: command not found")]
    NotFound(String),
}
