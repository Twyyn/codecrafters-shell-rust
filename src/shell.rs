use crate::{
    command::Command,
    env::{Args, Environment},
    exec::{
        Execute,
        ExecutionResult::{self, Continue, Exit},
    },
};
use thiserror::Error;

use std::io::Write;

const PROMPT: &str = "$";

#[derive(Debug, Default)]
pub struct Shell {
    env: Environment,
}

impl Shell {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self) -> Result<i32, ShellError> {
        loop {
            write!(self.env.output(), "{PROMPT} ")?;
            self.env.output().flush()?;

            match self.evaluate() {
                Ok(Continue) => {}
                Ok(Exit(code)) => return Ok(code.into()),

                Err(err) => writeln!(self.env.output(), "{err}")?,
            }
        }
    }

    fn evaluate(&mut self) -> Result<ExecutionResult, ShellError> {
        let Some(input) = self.env.reader().read_line()? else {
            return Ok(Continue);
        };

        let mut args = Args::parse(input);

        let Some(command) = Command::from_args(&mut args)? else {
            return Ok(Continue);
        };

        Ok(command.execute(&mut self.env)?)
    }
}

#[derive(Debug, Error)]
pub enum ShellError {
    #[error(transparent)]
    Command(#[from] crate::command::CommandError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
