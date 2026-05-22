//! Circuito ZK mínimo: entrada privada `age`, salida pública `meets` (1 si age >= min_age).
#![no_main]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    let age: u8 = env::read();
    let min_age: u8 = env::read();
    let meets: u8 = u8::from(age >= min_age);
    // Público: umbral pedido + resultado (la edad sigue siendo privada).
    env::commit(&(min_age, meets));
}
