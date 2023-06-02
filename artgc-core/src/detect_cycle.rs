use crate::circuit::{Circuit, Gate, WireId};
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct WireConnection {
    // List of id of gates which this wire goes into.
    to_ids: Vec<usize>,

    // Id of a gate which this wire comes out of.
    // Wires marked as inputs of a circuit has None.
    from_id: Option<usize>,
}

impl Default for WireConnection {
    fn default() -> Self {
        WireConnection {
            to_ids: vec![],
            from_id: None,
        }
    }
}

/// Check if given circuit has cyclic paths in it.
/// If it has any, returns pair of gate id and wire id of the starting node of the cycle.
///
/// Do Depth First Search to detect cyclic path in a circuit
pub fn detect_cycle(circuit: &Circuit) -> Option<(usize, WireId)> {
    // prepare DFS
    // scan all the gates and store how gates are connected.
    let mut wire_connections = vec![WireConnection::default(); circuit.get_wire_count()];

    for gate in circuit.get_all_gates() {
        let (id, x, y, out): (usize, usize, usize, usize) = match gate {
            Gate::Add { id, x, y, out } => (*id, x.into(), y.into(), out.into()),
            Gate::Mul { id, x, y, out } => (*id, x.into(), y.into(), out.into()),
        };

        wire_connections[x].to_ids.push(id);
        wire_connections[y].to_ids.push(id);
        wire_connections[out].from_id = Some(id)
    }

    let mut gate_visited = vec![0; circuit.get_gate_count()];
    let gates = circuit.get_all_gates();

    // Do DFS
    fn dfs(
        gate_id: usize,
        wire_id: usize,
        gates: &[Gate],
        gate_visited: &mut Vec<usize>,
        wire_connections: &Vec<WireConnection>,
    ) -> Option<(usize, usize)> {
        let gate = &gates[gate_id];
        let (id, out): (usize, usize) = match gate {
            Gate::Add { id, out, .. } => (*id, out.into()),
            Gate::Mul { id, out, .. } => (*id, out.into()),
        };
        if gate_visited[id] != 0 {
            // this gate has been visited at least once.
            // which means this node is a part of a cyclic path in the circuit
            return Some((gate_id, wire_id));
        }

        gate_visited[id] += 1;

        // get out wire of this gate
        let wire = wire_connections.get(out).unwrap();

        for next_gate_id in wire.to_ids.iter() {
            if let Some(pair) = dfs(*next_gate_id, out, gates, gate_visited, wire_connections) {
                return Some(pair);
            }
        }

        gate_visited[id] -= 1;
        None
    }

    let circuit_inputs = circuit.get_all_inputs();
    let mut input_gates = HashSet::<usize>::new();

    for i in circuit_inputs.iter() {
        let id: usize = i.into();
        if let Some(conn) = wire_connections.get(id) {
            for id in conn.to_ids.iter() {
                input_gates.insert(*id);
            }
        }
    }
    for g in input_gates.into_iter() {
        if let Some((gate_id, wire_id)) = dfs(g, 0, gates, &mut gate_visited, &wire_connections) {
            return Some((gate_id, WireId::from(wire_id)));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::detect_cycle;
    use crate::circuit::*;

    #[test]
    fn circuit_with_no_cycles() {
        //
        //       `out2`
        //         │
        //       ┌───┐
        //       │ * │ gate2
        //       └───┘
        //   ┌────┘ └───────┐
        //   │              │ `out1`
        // `in3`          ┌───┐
        //                │ + │ gate1
        //                └───┘
        //            ┌────┘ └────┐
        //            │           │
        //          `in1`       `in2`
        //
        let mut circuit = Circuit::new();

        // create gate1
        let in1 = circuit.create_new_wire();
        let in2 = circuit.create_new_wire();
        let out1 = circuit.create_new_wire();
        circuit.add_gate(GateType::Add, in1, in2, out1);

        // create gate2
        let in3 = circuit.create_new_wire();
        let out2 = circuit.create_new_wire();
        circuit.add_gate(GateType::Mul, in3, out1, out2);

        circuit.mark_input(in1);
        circuit.mark_input(in2);
        circuit.mark_input(in3);
        circuit.mark_output(out2);

        assert_eq!(detect_cycle(&circuit), None, "No cycle should be detected.");
    }

    #[test]
    fn circuit_with_a_cycle() {
        //
        //       `out`
        //         ├──────────────┐
        //         │              │
        //       ┌───┐            │
        //       │ * │ gate0      │
        //       └───┘            │
        //   ┌────┘ └───────┐     │
        //   │              │     │
        // `in1`          ┌───┐   │
        //          gate1 │ + │   │
        //                └───┘   │
        //           ┌─────┘ └────┘
        //           │
        //         `in2`
        //
        let mut circuit = Circuit::new();

        // create gate1
        let x1_id = circuit.create_new_wire();
        let y1_id = circuit.create_new_wire();
        let out1_id = circuit.create_new_wire();
        circuit.add_gate(GateType::Mul, x1_id, y1_id, out1_id);

        let x2_id = circuit.create_new_wire();
        circuit.add_gate(GateType::Add, x2_id, out1_id, y1_id);

        circuit.mark_input(x1_id);
        circuit.mark_input(x2_id);
        circuit.mark_output(out1_id);

        assert!(detect_cycle(&circuit).is_some(), "Cycle should be detected");
    }

    #[test]
    fn circuit_with_a_cycle_2() {
        //
        //       `out0`         `out1`
        //          │     ┌───────┼──────────────┐
        //        ┌───┐   │       │              │
        //  gate0 │ + │   │     ┌───┐            │
        //        └───┘   │     │ + │ gate1      │
        //    ┌────┘ └────┘     └───┘            │
        //    │             ┌────┘ └───────┐     │
        //  `in0`           │       `mid0` │     │
        //                `in1`          ┌───┐   │
        //                         gate2 │ + │   │
        //                               └───┘   │
        //                          ┌─────┘ └────┘
        //                          │
        //                        `in2`
        //
        let mut circuit = Circuit::new();

        // create gate1
        let in0 = circuit.create_new_wire();
        let in1 = circuit.create_new_wire();
        let in2 = circuit.create_new_wire();
        let mid0 = circuit.create_new_wire();
        let out0 = circuit.create_new_wire();
        let out1 = circuit.create_new_wire();

        let _gate0 = circuit.add_gate(GateType::Add, in0, out1, out0);
        let _gate1 = circuit.add_gate(GateType::Add, in1, mid0, out1);
        let _gate2 = circuit.add_gate(GateType::Add, in2, out1, mid0);

        circuit.mark_input(in0);
        circuit.mark_input(in1);
        circuit.mark_input(in2);
        circuit.mark_output(out0);
        circuit.mark_output(out1);

        assert!(detect_cycle(&circuit).is_some(), "Cycle should be detected");
    }
}
