use crate::circuit::Circuit;
use crate::ring::Ring;

#[derive(Debug, PartialEq, Eq)]
pub enum EvalLocalError {
    EmptyWire,
}

/// In order to keep track of wire value and layer
/// evaluating the gate requires the all the input wires
/// have to have actual values
#[derive(Clone, Copy)]
struct Wire<T> {
    pub layer: Option<usize>,
    pub value: Option<T>,
}

/// This method simply evaluates a given circuit with given inputs locally.
/// It doesn't involve any circuit garbling or networking operations.
/// Mostly used for debugging purpose
pub fn eval_local<T: Ring>(circuit: &Circuit, _inputs: Vec<T>) -> Result<Vec<T>, EvalLocalError> {
    // variable to keep track of actual wire values of type T and layer number
    let wires: Vec<Wire<T>> = vec![
        Wire {
            layer: None,
            value: None
        };
        circuit.get_wire_count()
    ];

    // put layer number to each wire as following rule.
    // 1. put 0 to wire marked as input

    // check if all the gates are evaluated
    let res = wires.iter().all(|w| w.value.is_some() && w.layer.is_some());
    if !res {
        return Err(EvalLocalError::EmptyWire);
    }

    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::eval_local;
    use crate::{circuit::*, ring::Ring};
    use ff::PrimeField;

    // Use finite field as a Ring
    // ff implements similar
    #[derive(PrimeField)]
    #[PrimeFieldModulus = "52435875175126190479447740508185965837690552500527637822603658699938581184513"]
    #[PrimeFieldGenerator = "7"]
    #[PrimeFieldReprEndianness = "little"]
    struct Fp([u64; 4]);
    impl Ring for Fp {}

    #[ignore]
    #[test]
    fn test_add_gate() {
        // Maybe it's better to use builder pattern

        let mut circuit = Circuit::new();

        // This circuit has a simple add gate
        // x + y = out
        let x_id = circuit.create_new_wire();
        let y_id = circuit.create_new_wire();
        let out_id = circuit.create_new_wire();
        circuit.add_gate(GateType::Add, x_id, y_id, out_id);

        circuit.mark_input(x_id);
        circuit.mark_input(y_id);
        circuit.mark_output(out_id);
        // check circuit is valid
        assert!(circuit.is_valid().is_ok(), "Circuit should be valid");

        // inputs: [x_id, y_id]
        let inputs: Vec<Fp> = vec![2.into(), 3.into()];
        let result = eval_local(&circuit, inputs);
        assert_eq!(result, Ok(vec![5.into()]), "Circuit: 2 + 3 should output 5");
    }

    #[ignore]
    #[test]
    fn test_mul_gate() {
        // Maybe it's better to use builder pattern

        let mut circuit = Circuit::new();

        // This circuit has a simple add gate
        // x * y = out
        let x_id = circuit.create_new_wire();
        let y_id = circuit.create_new_wire();
        let out_id = circuit.create_new_wire();
        circuit.add_gate(GateType::Mul, x_id, y_id, out_id);

        circuit.mark_input(x_id);
        circuit.mark_input(y_id);
        circuit.mark_output(out_id);
        // check circuit is valid
        assert!(circuit.is_valid().is_ok(), "Circuit should be valid");

        // inputs: [x_id, y_id]
        let inputs: Vec<Fp> = vec![2.into(), 3.into()];
        let result = eval_local(&circuit, inputs);
        assert_eq!(result, Ok(vec![6.into()]), "Circuit: 2 * 3 should output 6");
    }
}
