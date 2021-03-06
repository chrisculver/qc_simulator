mod circuit;
mod simulator;
mod gate;

extern crate nalgebra as na;


fn main() {
    println!("Hello, world!");

    let mut c = circuit::Circuit::new(4);
    c.x(1);
    c.h(2);
    c.h(3);
    c.cx(0,3);
    c.cx(2,0);
    c.cx(1,3);
    c.cx(2,1);
    c.h(0);
    c.y(0);
    c.z(0);
    c.y(3);
    c.cx(0,3);
    c.x(3);
    c.y(1);

    println!("gate[4]={}",c.gates[4].get_matrix(4));

    let mut s = simulator::Simulator::new(c);

    s.run();

    println!("final state = {}",s.state.print());
}
