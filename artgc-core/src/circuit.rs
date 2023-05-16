use std::fmt::Debug;
use std::ops::{Add, Mul};

use crate::error::{CircuitError, CircuitResult};

// Arithmetic Circuit is a DAG (Directed Acynclic Graph).
// The following types of gates shape DAGs as its nodes.
//
// Input gate: Starting nodes. No input, one output. Either Variable or Constant in F
// Output gate: Ending nodes. One input, no output.
// Add gate: Two input, one output. Calculate addition of two input values.
// Mul gate: Two input, one output. Calculate multiplication of two input values.
//
// Input gate, Add gate, Mul gate can be input to other gates.
// Output gate cannot be input to other gates
//
// T is Ring
// Ring defines addition and multiplication in Group

pub trait Ring:
    Sized
    + Eq
    + Copy
    + Clone
    + Send
    + Sync
    + Debug
    + 'static
    + Add<Output = Self>
    + Mul<Output = Self>
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> Mul<&'a Self, Output = Self>
{
}

// TODO: change type of inputs, outputs to vector of struct to form a DAG
// implement Input struct
// implement Output struct

pub struct Circuit<T: Ring> {
    inputs: Vec<T>,
    outputs: Vec<T>,
}

impl<T: Ring> Circuit<T> {
    pub fn new() -> Self {
        Circuit {
            inputs: vec![],
            outputs: vec![],
        }
    }

    /// Check if given circuit is valid circuit.
    /// Circuit validity is decided as following rules
    /// 1. Input length must be greater than 0.
    /// 2. Output length must be greater than 0.
    /// 3. All wires have to be connected to some other wires.
    /// 4. All wires have to have a path to at least one output wire.
    pub fn is_valid(&self) -> CircuitResult<()> {
        if self.inputs.is_empty() {
            return Err(CircuitError::EmptyInput);
        } else if self.outputs.is_empty() {
            return Err(CircuitError::EmptyOutput);
        }

        Ok(())
    }

    pub fn add_input(&mut self, input: T) {
        self.inputs.push(input)
    }

    pub fn add_output(&mut self, output: T) {
        self.outputs.push(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::CircuitError;
    use ff::{Field, PrimeField};

    // Use finite field as a Ring
    #[derive(PrimeField)]
    #[PrimeFieldModulus = "52435875175126190479447740508185965837690552500527637822603658699938581184513"]
    #[PrimeFieldGenerator = "7"]
    #[PrimeFieldReprEndianness = "little"]
    struct Fp([u64; 4]);

    // Mark Fp is Ring
    impl Ring for Fp {}

    #[test]
    fn simple_valid_circuit() {
        let mut circuit = Circuit::<Fp>::new();
        circuit.add_input(Fp::ZERO);
        circuit.add_output(Fp::ZERO);
        assert!(circuit.is_valid().is_ok());
    }

    #[test]
    fn circuit_without_input_should_be_invalid() {
        let mut circuit = Circuit::<Fp>::new();
        circuit.add_output(Fp::ZERO);
        let res = circuit.is_valid();
        assert!(res.is_err());
        assert_eq!(res, Err(CircuitError::EmptyInput));
    }

    #[test]
    fn circuit_without_output_should_be_invalid() {
        let mut circuit = Circuit::<Fp>::new();
        circuit.add_input(Fp::ZERO);
        let res = circuit.is_valid();
        assert!(res.is_err());
        assert_eq!(res, Err(CircuitError::EmptyOutput));
    }
}
