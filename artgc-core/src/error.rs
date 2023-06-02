use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum CircuitError {
    EmptyInput,
    EmptyOutput,
    CyclicPath { gate_id: usize, wire_id: usize },
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
            CircuitError::CyclicPath { gate_id, wire_id } => {
                write!(
                    f,
                    "This circuit has cyclic path. Gate with id{} has input wire with id{}.",
                    gate_id, wire_id
                )
            }
        }
    }
}
