use crate::env::Environment;

pub trait Execute {
    type Error;

    fn execute(&self, env: &mut Environment) -> Result<ExecutionResult, Self::Error>;
}

#[derive(Debug)]
#[repr(u8)]
pub enum ExecutionResult {
    Continue,
    Exit(u8),
}
