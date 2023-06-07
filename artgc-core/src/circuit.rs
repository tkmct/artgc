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

use crate::error::{CircuitError, CircuitResult};

/// Wire is a representation of a value carrier in garbled circuit.
/// It does not carry a value directly. Rather, it has encoded representation of the value called label.
/// In this specific instance of wire, we only have an id so that the two party can agree on the structure of
/// the circuit they are talking about.
// TODO: have a hashability by adding Derive serde
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WireId(pub usize);

impl From<usize> for WireId {
    fn from(val: usize) -> Self {
        WireId(val)
    }
}

impl Into<usize> for WireId {
    fn into(self) -> usize {
        self.0
    }
}

impl<'a> Into<usize> for &'a WireId {
    fn into(self) -> usize {
        self.0
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

impl Gate {
    pub fn get_output(&self) -> WireId {
        match self {
            Gate::Add { out, .. } => *out,
            Gate::Mul { out, .. } => *out,
        }
    }

    pub fn get_inputs(&self) -> (WireId, WireId) {
        match self {
            Gate::Add { x, y, .. } => (*x, *y),
            Gate::Mul { x, y, .. } => (*x, *y),
        }
    }
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
    // TODO: should make CircuitBuilder to do circuit construction
    pub fn new() -> Self {
        Circuit {
            inputs: vec![],
            outputs: vec![],
            gates: vec![],

            wire_count: 0,
            gate_count: 0,
        }
    }

    pub fn get_wire_count(&self) -> usize {
        self.wire_count
    }

    pub fn get_gate_count(&self) -> usize {
        self.gate_count
    }

    pub fn get_all_gates(&self) -> &[Gate] {
        &self.gates
    }

    pub fn get_gate(&self, id: usize) -> Option<&Gate> {
        self.gates.get(id)
    }

    pub fn get_all_inputs(&self) -> &[WireId] {
        &self.inputs
    }

    pub fn get_all_outputs(&self) -> &[WireId] {
        &self.outputs
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
            // } else if let Some(cycles) = check_cycles(self) {
            //     return Err();
        }

        Ok(())
    }

    /// Create a gate and add it to circuit
    /// gate_type: Type of Gate. GateType::Add or GateType::Mul
    /// x_id: wire id of the first input of the gate
    /// y_id: wire id of the second input of the gate
    /// out_id: id of the wire of output from this gate
    pub fn add_gate(
        &mut self,
        gate_type: GateType,
        x_id: WireId,
        y_id: WireId,
        out_id: WireId,
    ) -> usize {
        let id = self.gate_count;
        let gate = match gate_type {
            GateType::Add => Gate::Add {
                id,
                x: x_id,
                y: y_id,
                out: out_id,
            },
            GateType::Mul => Gate::Mul {
                id,
                x: x_id,
                y: y_id,
                out: out_id,
            },
        };

        self.gates.push(gate);
        self.gate_count += 1;

        id
    }

    /// Create a wire with a given value.
    /// Increment self.wire_len and return the wire instance.
    pub fn create_new_wire(&mut self) -> WireId {
        let wire_id = WireId::from(self.wire_count);
        self.wire_count += 1;
        wire_id
    }

    /// Create a wire instance and push it to the inputs vector.
    /// Return id of the newly created wire.
    pub fn mark_input(&mut self, wire_id: WireId) {
        self.inputs.push(wire_id);
    }

    /// Create a wire instance and push it to the outputs vector.
    /// Return id of the newly created wire.
    pub fn mark_output(&mut self, wire_id: WireId) {
        self.outputs.push(wire_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::CircuitError;

    #[test]
    fn simple_valid_circuit() {
        let mut circuit = Circuit::new();
        let input = circuit.create_new_wire();
        circuit.mark_input(input);

        let output = circuit.create_new_wire();
        circuit.mark_output(output);

        assert!(circuit.is_valid().is_ok(), "Circuit should be valid");
    }

    #[test]
    fn circuit_without_input_should_be_invalid() {
        let mut circuit = Circuit::new();
        let output = circuit.create_new_wire();
        circuit.mark_output(output);
        let res = circuit.is_valid();
        assert!(res.is_err(), "Circuit should be invalid");
        assert_eq!(
            res,
            Err(CircuitError::EmptyInput),
            "Result should be CircuitError::EmptyInput"
        );
    }

    #[test]
    fn circuit_without_output_should_be_invalid() {
        let mut circuit = Circuit::new();
        let input = circuit.create_new_wire();
        circuit.mark_input(input);

        let res = circuit.is_valid();
        assert!(res.is_err(), "Circuit should be invalid");
        assert_eq!(
            res,
            Err(CircuitError::EmptyOutput),
            "Result should be CircuitError::EmptyOutput"
        );
    }
}
