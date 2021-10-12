# Plan

A quantum circuit in the simplest set has two components:  some number of qubits and a list of operations applied to those qubits.

A first iteration will attempt to match the following syntax:

// To initialize a circuit, just tell us how many qubits there are
  let qc = QuantumCircuit {                   
              nq: 2   
            }

// Now you can add gates to either qubit
  qc.x(1) 
  qc.h(0)   
  qc.cx(0,1)    

// The following would give an error
//  qc.z(3)     


Then we will want to simulate the circuit exactly with a matrix simulation. 
  let simulator = Simulator {  
                    qc: qc     
                  }   
  simulator.run() 
  simulator.results()    
