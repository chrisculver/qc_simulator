use crate::circuit;
extern crate nalgebra as na;
type Complex = na::Complex<f64>;
use na::base::DVector as DVector;

struct State
{
    /// note you can probably figure out the size at compile time
    coefs: DVector<Complex>,
}

impl State {
    fn new(n: Option<usize>) -> State {
        match n {
            None => State { coefs: DVector::<Complex>::from(vec![Complex::new(0.,0.); 0]) },
            Some(_usize) => { 
                let mut s = State { 
                    coefs: DVector::<Complex>::from(vec![Complex::new(0.,0.); usize::pow(2, n.unwrap() as u32)]),
                };
                s.coefs[0]=Complex::new(1.,0.);
                s
            }
        }
    }

    // TODO: Probably just move this into display

    fn print(&self) -> String {
        let mut s: String = "".to_owned();

        let width=(self.coefs.len() as f64).log2() as usize;

        for (i,coef) in self.coefs.iter().enumerate() {
            if i==(self.coefs.len()-1) {
                s.push_str(&format!("{}: {:0width$b}",coef,i,width=width));
            } else {
                s.push_str(&format!("{}: {:0width$b}, ",coef,i,width=width));
            }
        }
        s
    }
}



pub struct Simulator {
    circ: circuit::Circuit,
    state: State,
}


impl Simulator {
    fn new(c: circuit::Circuit) -> Simulator {
        let nq = c.qubits;
        Simulator { circ: c, state: State::new(Some(nq)) } 
    }

    fn run(&mut self) {
        for g in self.circ.gates {
            self.state.coefs = g.get_matrix(self.circ.qubits)*self.state.coefs;
        }
    }
}

// tests for state
#[cfg(test)]
mod tests {
    use super::State;
    use super::Simulator;
    use crate::circuit;
    extern crate nalgebra as na;
    type Complex = na::Complex<f64>;
    use na::base::DVector as DVector;

    #[test]
    fn no_state() {
        let s = State::new(None);
        assert_eq!(s.coefs, DVector::<Complex>::from(vec![Complex::new(0.,0.); 0]));
    }

    #[test]
    fn qubit_states() {
        let s = State::new(Some(1));
        let mut expected = DVector::<Complex>::from(vec![Complex::new(0.,0.); 2]);
        expected[0]=Complex::new(1.,0.);
        assert_eq!(s.coefs,expected);
    }

    #[test]
    fn test_print() {
        let mut s = State::new(Some(3));
        s.coefs[0]=Complex::new(1.,0.);
        s.coefs[2]=Complex::new(-1.,-3.14);
        s.coefs[5]=Complex::new(0.5,0.5);
        s.coefs[7]=Complex::new(0.,1.);

        let expected = "1+0i: 000, 0+0i: 001, -1-3.14i: 010, 0+0i: 011, 0+0i: 100, 0.5+0.5i: 101, 0+0i: 110, 0+1i: 111";

        assert_eq!(s.print(), expected);
    }

    #[test]
    fn test_simple_circuits() {
        let mut c = circuit::Circuit::new(3);
        c.x(0);
        c.x(2);
        let mut s = Simulator::new(c);
        s.run();
        
        assert_eq!(s.state.print(), "test".to_string());
    }
}

