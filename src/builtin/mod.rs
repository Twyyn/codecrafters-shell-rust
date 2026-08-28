mod echo;
pub use echo::EchoCommand;

use crate::{
    env::Environment,
    exec::{Execute, ExecutionResult},
};
use thiserror::Error;

#[derive(Debug)]
pub enum Builtin {
    Exit,
    Echo(EchoCommand),
}

impl Execute for Builtin {
    type Error = BuiltinError;

    fn execute(&self, env: &mut Environment) -> Result<ExecutionResult, Self::Error> {
        match self {
            Self::Exit => Ok(ExecutionResult::Exit(0)),
            Self::Echo(echo_command) => echo_command.execute(env),
        }
    }
}

#[derive(Debug, Error)]
pub enum BuiltinError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
