use halo2_poseidon::poseidon::primitives::{ConstantLength, Hash, P128Pow5T3 as OrchardNullifier};
use halo2_proofs::{circuit::Value, dev::MockProver, halo2curves::pasta::pallas::Base as PallasFp};
use std::marker::PhantomData;

mod circuit;
use circuit::HashCircuit;

pub fn hash_circuit() {
    let message = [PallasFp::from(1), PallasFp::from(2), PallasFp::from(3)];
    println!("message: {:#?}", message);
    let output = Hash::<_, OrchardNullifier, ConstantLength<3>, 3, 2>::init().hash(message);
    println!("output: {:#?}", output);

    let circuit = HashCircuit::<OrchardNullifier, 3, 2, 3> {
        message: Value::known(message),
        output: Value::known(output),
        _spec: PhantomData,
    };

    let prover = MockProver::run(7, &circuit, vec![]).unwrap();
    // println!("prover: {:#?}", prover);
    assert!(prover.verify().is_ok());
}
