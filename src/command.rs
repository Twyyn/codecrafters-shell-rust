use crate::{
    builtin::{Builtin, EchoCommand},
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
            "echo" => Ok(Some(Self::Builtin(Builtin::Echo(EchoCommand::new(
                args.into_iter().map(str::to_string).collect(),
            ))))),

            _ => Err(CommandError::NotFound(name.into())),
        }
    }
}

impl Execute for Command {
    type Error = CommandError;

    fn execute(&self, env: &mut Environment) -> Result<ExecutionResult, Self::Error> {
        match self {
            Self::Builtin(builtin) => Ok(builtin.execute(env)?),
        }
    }
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error(transparent)]
    Builtin(#[from] crate::builtin::BuiltinError),

    #[error("{0}: command not found")]
    NotFound(String),
}
