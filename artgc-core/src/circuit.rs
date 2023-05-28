//! Arithmetic Circuit is a DAG (Directed Acynclic Graph).
//! The following types of wires and gates shape DAGs as its nodes.
//! Interemediate wires act as edges of the DAG.
//!
//! Input wire: Starting nodes. Carry a single value. Either Variable or Constant in F
//! Output wire: Ending nodes. One input, no output.
//! Add gate: Two input, one output. Calculate addition of two input values.
//! Mul gate: Two input, one output. Calculate multiplication of two input values.
//
//! Input gate, Add gate, Mul gate can be input to other gates.
//! Output gate cannot be input to other gates

// TODO: remove
// use std::fmt::Debug;
// use std::ops::{Add, Mul};

use crate::error::{CircuitError, CircuitResult};

// TODO: remove Ring to garbling instance
// /// T is Ring
// /// Ring defines addition and multiplication in Group
// pub trait Ring:
//     Sized
//     + Eq
//     + Copy
//     + Clone
//     + Send
//     + Sync
//     + Debug
//     + 'static
//     + Add<Output = Self>
//     + Mul<Output = Self>
//     + for<'a> Add<&'a Self, Output = Self>
//     + for<'a> Mul<&'a Self, Output = Self>
// {
// }

/// Wire is a representation of a value carrier in garbled circuit.
/// It does not carry a value directly. Rather, it has encoded representation of the value called label.
/// In this specific instance of wire, we only have an id so that the two party can agree on the structure of
/// the circuit they are talking about.
// TODO: have a hashability by adding Derive serde
#[derive(Clone, Copy)]
pub struct WireId(usize);

impl WireId {
    fn new(id: usize) -> Self {
        Self(id)
    }
}

/// A gate has id, input x, input y and out as members.
pub enum Gate {
    Add {
        id: usize,
        x: WireId,
        y: WireId,
        out: WireId,
    },
    Mul {
        id: usize,
        x: WireId,
        y: WireId,
        out: WireId,
    },
}

pub enum GateType {
    Add,
    Mul,
}

impl Gate {
    pub fn gate_type(&self) -> GateType {
        match self {
            Gate::Add { .. } => GateType::Add,
            Gate::Mul { .. } => GateType::Mul,
        }
    }
}

pub struct Circuit {
    inputs: Vec<WireId>,
    outputs: Vec<WireId>,
    gates: Vec<Gate>,
    wire_count: usize,
    gate_count: usize,
}

impl Circuit {
    pub fn new() -> Self {
        Circuit {
            inputs: vec![],
            outputs: vec![],
            gates: vec![],
            wire_count: 0,
            gate_count: 0,
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

    /// Create a gate and add it to
    pub fn add_gate(
        &mut self,
        gate_type: GateType,
        x_id: usize,
        y_id: usize,
        out_id: usize,
    ) -> usize {
        let id = self.gate_count;
        let gate = match gate_type {
            GateType::Add => Gate::Add {
                id,
                x: WireId(x_id),
                y: WireId(y_id),
                out: WireId(out_id),
            },
            GateType::Mul => Gate::Mul {
                id,
                x: WireId(x_id),
                y: WireId(y_id),
                out: WireId(out_id),
            },
        };

        self.gates.push(gate);
        self.gate_count += 1;

        id
    }

    pub fn add_mul_gate(&mut self, x_id: usize, y_id: usize, out_id: usize) -> usize {
        let id = self.gate_count;
        let gate = Gate::Mul {
            id,
            x: WireId(x_id),
            y: WireId(y_id),
            out: WireId(out_id),
        };
        self.gates.push(gate);
        self.gate_count += 1;

        id
    }

    /// Create a wire with a given value.
    /// Increment self.wire_len and return the wire instance.
    fn create_wire(&mut self) -> WireId {
        let wire_id = WireId::new(self.wire_count);
        self.wire_count += 1;
        wire_id
    }

    /// Create a wire instance and push it to the inputs vector.
    /// Return id of the newly created wire.
    pub fn add_input(&mut self) -> WireId {
        let wire_id = self.create_wire();
        self.inputs.push(wire_id);
        wire_id
    }

    /// Create a wire instance and push it to the outputs vector.
    /// Return id of the newly created wire.
    pub fn add_output(&mut self) -> WireId {
        let wire_id = self.create_wire();
        self.outputs.push(wire_id);
        wire_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::CircuitError;
    use ff::{Field, PrimeField};

    // Use finite field as a Ring
    // ff implements similar
    #[derive(PrimeField)]
    #[PrimeFieldModulus = "52435875175126190479447740508185965837690552500527637822603658699938581184513"]
    #[PrimeFieldGenerator = "7"]
    #[PrimeFieldReprEndianness = "little"]
    struct Fp([u64; 4]);

    #[test]
    fn simple_valid_circuit() {
        let mut circuit = Circuit::new();
        circuit.add_input();
        circuit.add_output();
        assert!(circuit.is_valid().is_ok());
    }

    #[test]
    fn circuit_without_input_should_be_invalid() {
        let mut circuit = Circuit::new();
        circuit.add_output();
        let res = circuit.is_valid();
        assert!(res.is_err());
        assert_eq!(res, Err(CircuitError::EmptyInput));
    }

    #[test]
    fn circuit_without_output_should_be_invalid() {
        let mut circuit = Circuit::new();
        circuit.add_input();
        let res = circuit.is_valid();
        assert!(res.is_err());
        assert_eq!(res, Err(CircuitError::EmptyOutput));
    }
}
