use std::io::Write;

use super::BuiltinError;
use crate::{
    env::Environment,
    exec::{Execute, ExecutionResult},
};

#[derive(Debug)]
pub struct EchoCommand {
    args: Vec<String>,
}

impl EchoCommand {
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }
}

impl Execute for EchoCommand {
    type Error = BuiltinError;

    fn execute(&self, env: &mut Environment) -> Result<ExecutionResult, Self::Error> {
        writeln!(env.output(), "{}", self.args.join(" "))?;

        Ok(ExecutionResult::Continue)
    }
}
