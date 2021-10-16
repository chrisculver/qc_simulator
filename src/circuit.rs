use crate::gate;
use gate::Gate;

pub struct Circuit {
    pub qubits: usize,
    pub gates: Vec<Gate>,
}

impl Circuit {
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
