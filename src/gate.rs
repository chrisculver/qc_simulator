extern crate nalgebra as na;
type Complex = na::Complex<f64>;
use na::base::SMatrix as SMatrix;
use na::Matrix2;
use na::DMatrix;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Gate {
    pub name: String,
    pub target: usize,
    pub control: Option<usize>,
}

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        if self.control.is_none() {
            write!(f, "{}({})", self.name, self.target)
        }
        else {
            write!(f, "{}({},{})", self.name, self.control.unwrap(), self.target)
        }
    }
}

impl Gate {
    pub fn get_matrix(&self, nq: usize) -> DMatrix<Complex> 
    where
    {
        let zero = Complex::new(0.,0.);
        let one = Complex::new(1.,0.);
        let id = SMatrix::<Complex,2,2>::new(one,zero,zero,one);
        
        let two:usize = 2;
        let size:usize = two.pow(nq as u32);

        //println!("expected size = {}",size);

        if self.name != String::from("CX") {
            let start;
            if self.target == nq-1 {
                start = self.get_single_qubit_gate();
            }
            else {
                start = id;
            }
            let mut matrix = DMatrix::from_fn(2,2,|r,c| start[(r,c)]);

            //println!("start={}",matrix); 
            
            let mut i=(nq-2) as isize;
            while i>(-1) {
                //println!("i={}",i);

                let tmp = matrix;
                
                let s = tmp.shape().0*2;
                let new;
                //println!("s={}",s); 
                if self.target as isize==i {
                    let gate = self.get_single_qubit_gate();
                    let t = tmp.kronecker(&gate);
                    new = DMatrix::<Complex>::from_fn(s,s,|r,c| t[(r,c)]);
                }
                else {
                    let t = tmp.kronecker(&id);
                    new = DMatrix::<Complex>::from_fn(s,s,|r,c| t[(r,c)]);
                }

                matrix = new;
                //println!("matrix={}",matrix);
                i=i-1;
            }
           
            //println!("size of matrix = {},{}", matrix.shape().0, matrix.shape().1);

            return DMatrix::<Complex>::from_fn(size, size, |r,c| matrix[(r,c)])

        } else if self.name == String::from("CX") {
            return DMatrix::<Complex>::from_fn(size, size, |r,c| self.cnot_elem(r,c,nq));
        } else {
            return DMatrix::<Complex>::from_fn(size, size, |_r,_c| Complex::new(0.,0.)); 
        }
    }
    
    fn cnot_elem(&self, i: usize, j: usize, nq: usize) -> Complex {
        //TODO: with masks and shifts will be quicker then converting to string.
        let mut res = Complex::new(0.0,0.0);
        let ibin = format!("{:0width$b}",i,width=nq);
        let jbin = format!("{:0width$b}",j,width=nq);
//        println!("i={},j={},c={},t={}",i,j,self.control.unwrap(),self.target);
//        println!(" ibin={}, jbin={}", ibin, jbin);

        let mut iother = ibin.to_string();
        let mut jother = jbin.to_string();

        if nq>2 && self.target>self.control.unwrap() {
            iother.remove(nq-self.target-1);
            jother.remove(nq-self.target-1);
            iother.remove(nq-self.control.unwrap()-2); // because this string has one less char already
            jother.remove(nq-self.control.unwrap()-2);
        } else if nq>2 && self.target<self.control.unwrap() {
            iother.remove(nq-self.control.unwrap()-1); 
            jother.remove(nq-self.control.unwrap()-1); 
            iother.remove(nq-self.target-2);
            jother.remove(nq-self.target-2);
        } else {
            iother = "".to_string();
            jother = "".to_string();
        }

//        println!("  iother={}, jother={}", iother, jother);

        let ti = ibin.chars().nth(nq-self.target-1).unwrap().to_digit(10).unwrap();
        let tj = jbin.chars().nth(nq-self.target-1).unwrap().to_digit(10).unwrap();
        let ci = ibin.chars().nth(nq-self.control.unwrap()-1).unwrap().to_digit(10).unwrap();
        let cj = jbin.chars().nth(nq-self.control.unwrap()-1).unwrap().to_digit(10).unwrap();

        if ci==0 && cj==0 && i==j {
            res = Complex::new(1.0,0.0);
        } else if ci==1 && cj==1 {
            if ti!=tj && iother==jother {
                res = Complex::new(1.0,0.0);
            }
        }
        
 //       println!("  res={}", res);
        res
    }

    fn get_single_qubit_gate(&self) -> Matrix2<Complex> {
        //let mut map = HashMap::new();
        let zero = Complex::new(0.,0.);
        let one = Complex::new(1.,0.);
        let im = Complex::new(0.,1.);

        type Mat2x2 = SMatrix::<Complex,2,2>;
        //map.insert(String::from("X"), 
        let x_mat = Mat2x2::new(zero,one,
                               one,zero);
        //map.insert(String::from("Y"), 
        let y_mat = Mat2x2::new(zero,-im,
                               im,zero);
        //map.insert(String::from("Z"), 
        let z_mat = Mat2x2::new(one,zero,
                               zero,-one);
        let two = 2.0_f64;
        let coef = Complex::new(1./two.sqrt(),0.);
        //map.insert(String::from("H"), 
        let h_mat = Mat2x2::new(coef,coef,
                               coef,-coef);
        //map
        match self.name.as_str() {
            "X" => return x_mat,
            "Y" => return y_mat,
            "Z" => return z_mat,
            "H" => return h_mat,
            _ => return Mat2x2::new(zero,zero,zero,zero),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate nalgebra as na;
    use super::Gate;
    type Complex = na::Complex<f64>;
    use na::base::SMatrix as SMatrix;

    #[test]
    fn test_kron_qubit_gates() {
        type Mat4x4 = SMatrix::<Complex,4,4>;
        let zero = Complex::new(0.,0.);
        let one = Complex::new(1.,0.);
        let id_tens_x = Mat4x4::new(zero,one,zero,zero,
                                    one,zero,zero,zero,
                                    zero,zero,zero,one,
                                    zero,zero,one,zero);
    
        let x_tens_id = Mat4x4::new(zero,zero,one,zero,
                                    zero,zero,zero,one,
                                    one,zero,zero,zero,
                                    zero,one,zero,zero);

        let xgate0 = Gate { 
            name: String::from("X"),            
            target: 0,
            control: None,
        };
        let xgate1 = Gate { 
            name: String::from("X"),            
            target: 1,
            control: None,
        };

        assert_eq!(xgate1.get_matrix(2),x_tens_id);
        assert_eq!(xgate0.get_matrix(2),id_tens_x);
    }
}
