use halo2_poseidon::poseidon::primitives::{ConstantLength, Hash, P128Pow5T3 as OrchardNullifier};
use halo2_proofs::{circuit::Value, dev::MockProver, halo2curves::pasta::pallas::Base as PallasFp};
use std::marker::PhantomData;

use super::merkle_circuit::MerkleCircuit;

pub fn merke_circuit_with_hash() {
    let leaves = [
        PallasFp::from(1),
        PallasFp::from(2),
        PallasFp::from(3),
        PallasFp::from(4),
    ];
    let h1 =
        Hash::<_, OrchardNullifier, ConstantLength<2>, 3, 2>::init().hash([leaves[0], leaves[1]]);
    let h2 =
        Hash::<_, OrchardNullifier, ConstantLength<2>, 3, 2>::init().hash([leaves[2], leaves[3]]);
    let root = Hash::<_, OrchardNullifier, ConstantLength<2>, 3, 2>::init().hash([h1, h2]);
    println!("root: {root:?}");

    let circuit = MerkleCircuit::<OrchardNullifier, 3, 2> {
        leaf: Value::known(leaves[0]),
        path_elements: vec![Value::known(leaves[1]), Value::known(h2)],
        path_indices: vec![
            Value::known(PallasFp::from(0)),
            Value::known(PallasFp::from(0)),
        ],
        _spec: PhantomData,
    };

    let prover = MockProver::run(10, &circuit, vec![vec![root]]).unwrap();
    assert!(prover.verify().is_ok());
}
