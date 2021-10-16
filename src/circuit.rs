extern crate nalgebra as na;
type Complex = na::Complex<f64>;
use na::base::SMatrix as SMatrix;
use na::Matrix2;
use na::DMatrix;

#[derive(Debug, Clone, PartialEq)]
pub struct Gate {
    pub name: String,
    pub target: usize,
    pub control: Option<usize>,
}


impl Gate {
    pub fn get_matrix(self, nq: usize) -> DMatrix<Complex> 
    where
    {
        let zero = Complex::new(0.,0.);
        let one = Complex::new(1.,0.);
        let id = SMatrix::<Complex,2,2>::new(one,zero,zero,one);
        
        let two:usize = 2;
        let size:usize = two.pow(nq as u32);

        if self.name != String::from("CX") {
            let mut start = SMatrix::<Complex,2,2>::new(zero,zero,zero,zero);
            if self.target == 0 {
                start = self.get_single_qubit_gate();
            }
            else {
                start = id;
            }
            
            let matrix = start;

            for i in 1..nq {
                if self.target==i {
                    let gate = self.get_single_qubit_gate();
                    let matrix = matrix.kronecker(&gate);
                }
                else {
                    let matrix = matrix.kronecker(&id);
                }
            }
            
            return DMatrix::<Complex>::from_fn(size, size, |r,c| matrix[(r,c)])

        } else if self.name == String::from("CX") {
            return DMatrix::<Complex>::from_fn(size, size, |r,c| self.cnot_elem(r,c));
        } else {
            return DMatrix::<Complex>::from_fn(size,size, |r,c| Complex::new(0.,0.)); 
        }
    }
    
    fn cnot_elem(&self, i: usize, j: usize) -> Complex {
        Complex::new(0.,0.)
    }

    fn get_single_qubit_gate(&self) -> Matrix2<Complex> {
        //let mut map = HashMap::new();
        let zero = Complex::new(0.,0.);
        let one = Complex::new(1.,0.);
        let im = Complex::new(0.,1.);

        type Mat2x2 = SMatrix::<Complex,2,2>;
        //map.insert(String::from("X"), 
        let Xmat = Mat2x2::new(zero,one,
                               one,zero);
        //map.insert(String::from("Y"), 
        let Ymat = Mat2x2::new(zero,-im,
                               im,zero);
        //map.insert(String::from("Z"), 
        let Zmat = Mat2x2::new(one,zero,
                               zero,-one);
        let two = 2.0_f64;
        let coef = Complex::new(1./two.sqrt(),0.);
        //map.insert(String::from("H"), 
        let Hmat = Mat2x2::new(coef,coef,
                               coef,-coef);
        //map
        match self.name.as_str() {
            "X" => return Xmat,
            "Y" => return Ymat,
            "Z" => return Zmat,
            "H" => return Hmat,
            _ => return Mat2x2::new(zero,zero,zero,zero),
        }
    }
}


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
    use super::Gate;
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
