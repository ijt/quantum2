// main.rs

use computer::QuantumComputer;
use algorithms::deutsch;
use gates;

// Let's do something simple of a 3-qubit system.
let mut c1 = QuantumComputer::new(3);
c1.initialize(5);
c1.apply(gates::identity(3));
c1.collapse();
assert_eq!(5, c1.value());

// Now let's perform a coin flip using the Hadamard transform.
let mut c2 = QuantumComputer::new(1);
c2.initialize(0);
c2.apply(gates::hadamard(1));
c2.collapse();
let result = if 1 == c2.value() { "heads" } else { "tails" };
println!("coin flip: {}", result);

// Finally let's determine whether f: {0, 1} -> {0, 1} is constant
// or balanced using Deutsch's algorithm.
// (see http://physics.stackexchange.com/q/3400)
let mut c3 = QuantumComputer::new(2);
c3.initialize(1);
c3.apply(gates::hadamard(2));
c3.apply(deutsch::deutsch_gate(f));
c3.apply(gates::hadamard(2));
c3.collapse();
let result = if 1 == c3.value() { "constant" } else { "balanced" };
println!("f is: {}", result);
