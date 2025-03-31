use halo2_poseidon::poseidon::primitives::{ConstantLength, Hash, P128Pow5T3 as OrchardNullifier};
use halo2_proofs::{
    circuit::Value,
    dev::{CellValue, MockProver},
    halo2curves::pasta::pallas::Base as PallasFp,
};
use std::marker::PhantomData;

mod merkle_nohash0;
mod merkle_nohash1;
mod merkle_nohash2;
mod merkle_nohash3;
mod merkle_nohash4;

pub use merkle_nohash0::MerkleCircuitNoHash0;
pub use merkle_nohash1::MerkleCircuitNoHash1;
pub use merkle_nohash2::MerkleCircuitNoHash2;
pub use merkle_nohash3::MerkleCircuitNoHash3;
pub use merkle_nohash4::MerkleCircuitNoHash4;

pub fn merke_nohash0() {
    // let's use 4 leaves for the example
    let leaves = [
        PallasFp::from(2),
        PallasFp::from(5),
        PallasFp::from(11),
        PallasFp::from(20),
    ];

    // as stated in the article, we don't hash leaves but just add them together
    let h1 = leaves[0] + leaves[1];
    let h2 = leaves[2] + leaves[3];
    let root = h1 + h2;

    let circuit = MerkleCircuitNoHash0 {
        // we prove knowledge of the first leaf
        leaf: Value::known(leaves[0]),
        path_elements: vec![Value::known(leaves[1]), Value::known(h2)],
        path_indices: vec![
            Value::known(PallasFp::from(0)),
            Value::known(PallasFp::from(0)),
        ],
    };

    let prover = MockProver::run(4, &circuit, vec![vec![root]]).unwrap();
    assert!(prover.verify().is_ok());
}

pub fn merke_nohash1() {
    let leaves = [
        PallasFp::from(2),
        PallasFp::from(5),
        PallasFp::from(11),
        PallasFp::from(20),
    ];
    let h1 = leaves[0] + leaves[1];
    let h2 = leaves[2] + leaves[3];
    let root = h1 + h2;

    let circuit = MerkleCircuitNoHash1 {
        leaf: Value::known(leaves[0]),
        path_elements: vec![Value::known(leaves[1]), Value::known(h2)],
        path_indices: vec![
            Value::known(PallasFp::from(0)),
            Value::known(PallasFp::from(0)),
        ],
    };

    let prover = MockProver::run(4, &circuit, vec![vec![root]]).unwrap();
    assert!(prover.verify().is_ok());

    // Let's fake the proof
    // we just pass the root hash as the leaf, and no other layer
    let circuit = MerkleCircuitNoHash1 {
        leaf: Value::known(root),
        path_elements: vec![],
        path_indices: vec![],
    };
    let prover = MockProver::run(4, &circuit, vec![vec![root]]).unwrap();
    assert!(prover.verify().is_ok());
}

pub fn merke_nohash2() {
    let leaves = [
        PallasFp::from(2),
        PallasFp::from(5),
        PallasFp::from(11),
        PallasFp::from(20),
    ];
    let h1 = leaves[0] + leaves[1];
    let h2 = leaves[2] + leaves[3];
    let root = h1 + h2;

    let circuit = MerkleCircuitNoHash2 {
        leaf: Value::known(h1),
        path_elements: vec![Value::known(h2)],
        path_indices: vec![Value::known(PallasFp::from(0))],
    };

    let prover = MockProver::run(4, &circuit, vec![vec![root]]).unwrap();
    assert!(prover.verify().is_ok());
}

pub fn merke_nohash3() {
    let hash_leaf = |v: PallasFp| v + v;
    let leaves = [
        PallasFp::from(2),
        hash_leaf(PallasFp::from(5)),
        hash_leaf(PallasFp::from(11)),
        hash_leaf(PallasFp::from(20)),
    ];
    let h1 = hash_leaf(leaves[0]) + leaves[1];
    let h2 = leaves[2] + leaves[3];
    let root = h1 + h2;

    let circuit = MerkleCircuitNoHash3 {
        leaf: Value::known(leaves[0]),
        path_elements: vec![Value::known(leaves[1]), Value::known(h2)],
        path_indices: vec![
            Value::known(PallasFp::from(0)),
            Value::known(PallasFp::from(0)),
        ],
    };
    let prover = MockProver::run(4, &circuit, vec![vec![root]]).unwrap();
    assert!(prover.verify().is_ok());

    // you know the drill by now... let's fake the proof again!
    let random_leaf = PallasFp::from(15);
    let circuit = MerkleCircuitNoHash3 {
        leaf: Value::known(random_leaf),
        path_elements: vec![Value::known(h2)],
        path_indices: vec![Value::known(PallasFp::from(0))],
    };
    let mut prover = MockProver::run(4, &circuit, vec![vec![root]]).unwrap();
    // Ok this one is a bit tricky, we need to change the advice for the first and third columns
    // because the value is copied multiple times
    let advice0 = prover.advice_mut(0);
    advice0[1] = CellValue::Assigned(h1);
    advice0[2] = CellValue::Assigned(h1);
    let advice2 = prover.advice_mut(2);
    advice2[0] = CellValue::Assigned(h1);
    advice2[2] = CellValue::Assigned(root);
    assert!(prover.verify().is_ok());
}

pub fn merke_nohash4() {
    let hash_leaf = |v: PallasFp| v + v;
    let leaves = [
        PallasFp::from(2),
        hash_leaf(PallasFp::from(5)),
        hash_leaf(PallasFp::from(11)),
        hash_leaf(PallasFp::from(20)),
    ];
    let h1 = hash_leaf(leaves[0]) + leaves[1];
    let h2 = leaves[2] + leaves[3];
    let root = h1 + h2;

    let circuit = MerkleCircuitNoHash4 {
        leaf: Value::known(leaves[0]),
        path_elements: vec![Value::known(leaves[1]), Value::known(h2)],
        path_indices: vec![
            Value::known(PallasFp::from(0)),
            Value::known(PallasFp::from(0)),
        ],
    };
    let prover = MockProver::run(4, &circuit, vec![vec![root]]).unwrap();
    assert!(prover.verify().is_ok());
}

mod merkle_circuit;
use merkle_circuit::MerkleCircuit;

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
