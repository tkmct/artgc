use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum CircuitError {
    EmptyInput,
    EmptyOutput,
}

pub type CircuitResult<E> = Result<E, CircuitError>;

impl std::error::Error for CircuitError {}

impl Display for CircuitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            CircuitError::EmptyInput => {
                write!(f, "This circuit has no input.")
            }
            CircuitError::EmptyOutput => {
                write!(f, "This circuit has no output.")
            }
        }
    }
}
