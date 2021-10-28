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
        for (i,coef) in self.coefs.iter().enumerate() {
            s.push_str(&format!("{}: {:b}",coef,i));
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

    fn run(mut self) {
        for g in self.circ.gates {
            self.state.coefs = g.get_matrix(self.circ.qubits)*self.state.coefs;
        }
    }
}

// tests for state
#[cfg(test)]
mod tests {
    use super::State;
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
}

#[cfg(test)]
mod tests {

}
