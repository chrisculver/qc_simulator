use crate::gate;
use gate::Gate as Gate;

///Hold the information of a quantum circuit, can be passed into simulator
///to get the output from an perfect quantum computer.
///
///The [`Circuit::new`] method lets you create a circuit of q qubits and no gates.
///To add gates to the circuit you can call any of the implemented gates, currently
///there is support for x,y,z,h, and cx.  Each of these are implemented as methods
///with arguments corresponding to the qubits you want the gates to act on. 
///
///The cx gate operates on two qubits, taking the control first and target second.
///
///Examples:
///
///```
///let c = Circuit::new(4);
///c.x(0);
///c.x(3);
///c.cx(3,1);
///c.h(1);
///c.z(2);
///c.cx(1,2);
///```
pub struct Circuit {
    pub qubits: usize,
    pub gates: Vec<Gate>,
}

impl Circuit {
    ///Creates a new circuit with q qubits and no gates
    ///
    ///# Example for a 3 qubit circuit
    ///
    ///```
    ///let c = Circuit::new(3);
    ///```
    pub fn new(q: usize) -> Circuit {
        Circuit { qubits: q, gates: Vec::<Gate>::new()}
    } 

    pub fn x(&mut self, q: usize ) {
        self.gates.push(Gate { 
            name: String::from("X"),
            target: q,
            control: None
        })
    }
    
    pub fn y(&mut self, q: usize ) {
        self.gates.push(Gate { 
            name: String::from("Y"),
            target: q,
            control: None
        })
    }
    
    pub fn z(&mut self, q: usize ) {
        self.gates.push(Gate { 
            name: String::from("Z"),
            target: q,
            control: None
        })
    }
    
    pub fn h(&mut self, q: usize ) {
        self.gates.push(Gate { 
            name: String::from("H"),
            target: q,
            control: None
        })
    }
    
    pub fn cx(&mut self, c: usize, t: usize) {
        self.gates.push(Gate { 
            name: String::from("CX"),
            control: Some(c),
            target: t,
        })
    }

}


#[cfg(test)]
mod tests {
    use super::Circuit;
    use crate::gate;
    use gate::Gate;

    #[test]
    fn empty_circuit() {
        let circ = Circuit {
            qubits: 0,
            gates: Vec::new(),
        };

        assert_eq!(circ.qubits,0);
        assert_eq!(circ.gates,Vec::<Gate>::new());
    }
    
    #[test]
    fn one_of_each() {
        let mut circ = Circuit {
            qubits: 4,
            gates: Vec::new(),
        };
        circ.x(0);
        circ.y(1);
        circ.z(2);
        circ.h(3);
        circ.cx(0,1);

        let mut expected = Vec::<Gate>::new();
        expected.push( Gate { 
            name: String::from("X"),
            target: 0,
            control: None,
        } );
        expected.push( Gate { 
            name: String::from("Y"),
            target: 1,
            control: None,
        } );
        expected.push( Gate { 
            name: String::from("Z"),
            target: 2,
            control: None,
        } );
        expected.push( Gate { 
            name: String::from("H"),
            target: 3,
            control: None,
        } );
        expected.push( Gate { 
            name: String::from("CX"),
            control: Some(0),
            target: 1,
        } );

        assert_eq!(circ.qubits,4);
        assert_eq!(circ.gates,expected);
    }

    #[test]
    fn test_gate_matrices() {
        
    }
}
