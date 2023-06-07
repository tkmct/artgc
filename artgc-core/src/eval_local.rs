use crate::circuit::{Circuit, GateType};
use crate::ring::Ring;
use std::cmp::max;

#[derive(Debug, PartialEq, Eq)]
pub enum EvalLocalError {
    EmptyWire,
}

/// In order to keep track of wire value and layer
/// evaluating the gate requires the all the input wires
/// have to have actual values
/// Wires marked as inputs of the circuit have layer as 0
/// Output wires of gates have layer number of max(input1_layer, input2_layer) + 1.
#[derive(Clone, Copy, Debug)]
struct Wire<T: Ring> {
    pub layer: Option<usize>,
    pub value: Option<T>,
}

/// Returns information related to layer of gates and wire
/// scan the circuit and put layer number to all gates and wires.
/// First element of returend tuple is vector of vector of gate_id.
/// Gates are grouped with layer number represented with index of outer vector.
fn label_wires_with_layer<T: Ring>(circuit: &Circuit) -> (Vec<Vec<usize>>, Vec<Wire<T>>) {
    let mut wires = vec![
        Wire {
            layer: None,
            value: None
        };
        circuit.get_wire_count()
    ];

    let mut gate_layers: Vec<Vec<usize>> = vec![];

    // put 0 to input layer
    let input_wires = circuit.get_all_inputs();
    for input in input_wires {
        wires[input.0].layer = Some(0);
    }

    let gates = circuit.get_all_gates();

    // iterate through gates until all of the wire has layer value
    // TODO: optimize iteration
    let mut i = 0;
    while !wires.iter().all(|w| w.layer.is_some()) {
        let wire_id = gates[i].get_output();
        let wire = wires[wire_id.0];

        if !wire.layer.is_some() {
            // check if the two input wires of the gate has layer or not
            let (x, y) = gates[i].get_inputs();
            let x_layer = wires[x.0].layer;
            let y_layer = wires[y.0].layer;
            if x_layer.is_some() && y_layer.is_some() {
                let current_layer = max(x_layer.unwrap(), y_layer.unwrap());
                wires[wire_id.0].layer = Some(current_layer + 1);

                // TODO: possible skip if optimization is set to true
                // provide max layer number using config file
                if gate_layers.get(current_layer).is_none() {
                    gate_layers.resize(current_layer + 1, Vec::<usize>::new());
                }
                gate_layers.get_mut(current_layer).unwrap().push(i);
            }
        }

        if i == wires.len() - 1 {
            i = 0;
        } else {
            i += 1;
        }
    }

    return (gate_layers, wires);
}

/// This method simply evaluates a given circuit with given inputs locally.
/// It doesn't involve any circuit garbling or networking operations.
/// Mostly used for debugging purpose
pub fn eval_local<T: Ring>(
    circuit: &Circuit,
    input_values: Vec<T>,
) -> Result<Vec<T>, EvalLocalError> {
    // variable to keep track of actual wire values of type T and layer number
    // put layer number to all layers and gates
    let (gate_layers, mut wires) = label_wires_with_layer::<T>(circuit);
    println!("gate_layers: {:?}, wires: {:?}", gate_layers, wires);

    let all_gates = circuit.get_all_gates();
    let all_inputs = circuit.get_all_inputs();

    // put value to input wires
    for (i, wire_id) in all_inputs.into_iter().enumerate() {
        let wire = wires.get_mut(wire_id.0).unwrap();
        wire.value = Some(input_values[i]);
    }

    for current_layer in 0..(gate_layers.len()) {
        for gate_id in gate_layers[current_layer].iter() {
            let gate = &all_gates[*gate_id];
            let (in1, in2) = gate.get_inputs();
            let out = gate.get_output();

            let in1 = wires.get(in1.0).unwrap().value.unwrap();
            let in2 = wires.get(in2.0).unwrap().value.unwrap();
            let mut out = wires.get_mut(out.0).unwrap();

            out.value = match gate.gate_type() {
                GateType::Add => Some(in1 + in2),
                GateType::Mul => Some(in1 * in2),
            };
        }
    }

    // check if all the gates are evaluated
    let res = wires.iter().all(|w| w.value.is_some() && w.layer.is_some());
    if !res {
        return Err(EvalLocalError::EmptyWire);
    }

    Ok(circuit
        .get_all_outputs()
        .iter()
        .map(|out| wires.get(out.0).unwrap().value.unwrap())
        .collect())
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

    #[test]
    fn test_add_gate() {
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

    #[test]
    fn test_mul_gate() {
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

    #[test]
    fn test_multiple_outputs() {
        let mut circuit = Circuit::new();

        // Circuit
        // out1 = in1 + in2
        // out2 = (in1 + in2) * in3

        // gate1
        let in1 = circuit.create_new_wire();
        let in2 = circuit.create_new_wire();
        let out1 = circuit.create_new_wire();
        circuit.add_gate(GateType::Add, in1, in2, out1);

        // gate2
        let in3 = circuit.create_new_wire();
        let out2 = circuit.create_new_wire();
        circuit.add_gate(GateType::Mul, in3, out1, out2);

        circuit.mark_input(in1);
        circuit.mark_input(in2);
        circuit.mark_input(in3);
        circuit.mark_output(out1);
        circuit.mark_output(out2);

        let inputs: Vec<Fp> = vec![1.into(), 2.into(), 3.into()];
        let result = eval_local(&circuit, inputs);
        assert_eq!(
            result,
            Ok(vec![3.into(), 9.into()]),
            "Circuit output2 two values: [3, 9]"
        );
    }
}
