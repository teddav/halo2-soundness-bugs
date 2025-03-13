use halo2_proofs::{
    circuit::Value,
    dev::{CellValue, MockProver},
    halo2curves::pasta::pallas::Base as PallasFp,
};

use super::merkle_nohash1::MerkleCircuitNoHash1;
use super::merkle_nohash2::MerkleCircuitNoHash2;
use super::merkle_nohash3::MerkleCircuitNoHash3;
use super::merkle_nohash4::MerkleCircuitNoHash4;
use super::merkle_nohash5::MerkleCircuitNoHash5;

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

    let circuit = MerkleCircuitNoHash2 {
        leaf: Value::known(root),
        path_elements: vec![],
        path_indices: vec![],
    };

    let prover = MockProver::run(4, &circuit, vec![vec![root]]).unwrap();
    // println!("prover: {:#?}", prover);
    assert!(prover.verify().is_ok());
}

pub fn merke_nohash3() {
    let leaves = [
        PallasFp::from(2),
        PallasFp::from(5),
        PallasFp::from(11),
        PallasFp::from(20),
    ];
    let h1 = leaves[0] + leaves[1];
    let h2 = leaves[2] + leaves[3];
    let root = h1 + h2;

    let circuit = MerkleCircuitNoHash3 {
        leaf: Value::known(h1),
        path_elements: vec![Value::known(h2)],
        path_indices: vec![Value::known(PallasFp::from(0))],
    };

    let prover = MockProver::run(4, &circuit, vec![vec![root]]).unwrap();
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

    // Let's fake the proof
    let fake_root = PallasFp::from(0x1234);
    let mut prover = MockProver::run(4, &circuit, vec![vec![fake_root]]).unwrap();
    let advice = prover.advice_mut(2);
    advice[7] = CellValue::Assigned(fake_root);
    assert!(prover.verify().is_ok());
}

pub fn merke_nohash5() {
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

    let circuit = MerkleCircuitNoHash5 {
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
    let fake_root = PallasFp::from(0x1234);
    let mut prover = MockProver::run(4, &circuit, vec![vec![fake_root]]).unwrap();
    let advice = prover.advice_mut(2);
    advice[4] = CellValue::Assigned(fake_root);
    assert!(prover.verify().is_err());
}
