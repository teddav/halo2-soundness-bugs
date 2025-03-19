use halo2_proofs::{
    circuit::Value,
    dev::{CellValue, MockProver},
    halo2curves::pasta::pallas::Base as PallasFp,
};

mod merkle_nohash0;
mod merkle_nohash2;
mod merkle_nohash3;
mod merkle_nohash4;
mod merkle_nohash5;

// mod with_hash;
// pub use with_hash::merke_circuit_with_hash;

pub use merkle_nohash0::MerkleCircuitNoHash0;
// pub use no_hash::{merke_nohash1, merke_nohash2, merke_nohash3, merke_nohash4, merke_nohash5};

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

// pub fn merke_nohash2() {
//     let leaves = [
//         PallasFp::from(2),
//         PallasFp::from(5),
//         PallasFp::from(11),
//         PallasFp::from(20),
//     ];
//     let h1 = leaves[0] + leaves[1];
//     let h2 = leaves[2] + leaves[3];
//     let root = h1 + h2;

//     let circuit = MerkleCircuitNoHash2 {
//         leaf: Value::known(leaves[0]),
//         path_elements: vec![Value::known(leaves[1]), Value::known(h2)],
//         path_indices: vec![
//             Value::known(PallasFp::from(0)),
//             Value::known(PallasFp::from(0)),
//         ],
//     };

//     let prover = MockProver::run(4, &circuit, vec![vec![root]]).unwrap();
//     assert!(prover.verify().is_ok());

//     // Let's fake the proof

//     let circuit = MerkleCircuitNoHash2 {
//         leaf: Value::known(root),
//         path_elements: vec![],
//         path_indices: vec![],
//     };

//     let prover = MockProver::run(4, &circuit, vec![vec![root]]).unwrap();
//     // println!("prover: {:#?}", prover);
//     assert!(prover.verify().is_ok());
// }

// pub fn merke_nohash3() {
//     let leaves = [
//         PallasFp::from(2),
//         PallasFp::from(5),
//         PallasFp::from(11),
//         PallasFp::from(20),
//     ];
//     let h1 = leaves[0] + leaves[1];
//     let h2 = leaves[2] + leaves[3];
//     let root = h1 + h2;

//     let circuit = MerkleCircuitNoHash3 {
//         leaf: Value::known(h1),
//         path_elements: vec![Value::known(h2)],
//         path_indices: vec![Value::known(PallasFp::from(0))],
//     };

//     let prover = MockProver::run(4, &circuit, vec![vec![root]]).unwrap();
//     assert!(prover.verify().is_ok());
// }

// pub fn merke_nohash4() {
//     let hash_leaf = |v: PallasFp| v + v;
//     let leaves = [
//         PallasFp::from(2),
//         hash_leaf(PallasFp::from(5)),
//         hash_leaf(PallasFp::from(11)),
//         hash_leaf(PallasFp::from(20)),
//     ];
//     let h1 = hash_leaf(leaves[0]) + leaves[1];
//     let h2 = leaves[2] + leaves[3];
//     let root = h1 + h2;

//     let circuit = MerkleCircuitNoHash4 {
//         leaf: Value::known(leaves[0]),
//         path_elements: vec![Value::known(leaves[1]), Value::known(h2)],
//         path_indices: vec![
//             Value::known(PallasFp::from(0)),
//             Value::known(PallasFp::from(0)),
//         ],
//     };

//     let prover = MockProver::run(4, &circuit, vec![vec![root]]).unwrap();
//     assert!(prover.verify().is_ok());

//     // Let's fake the proof
//     let fake_root = PallasFp::from(0x1234);
//     let mut prover = MockProver::run(4, &circuit, vec![vec![fake_root]]).unwrap();
//     let advice = prover.advice_mut(2);
//     advice[7] = CellValue::Assigned(fake_root);
//     assert!(prover.verify().is_ok());
// }

// pub fn merke_nohash5() {
//     let hash_leaf = |v: PallasFp| v + v;
//     let leaves = [
//         PallasFp::from(2),
//         hash_leaf(PallasFp::from(5)),
//         hash_leaf(PallasFp::from(11)),
//         hash_leaf(PallasFp::from(20)),
//     ];
//     let h1 = hash_leaf(leaves[0]) + leaves[1];
//     let h2 = leaves[2] + leaves[3];
//     let root = h1 + h2;

//     let circuit = MerkleCircuitNoHash5 {
//         leaf: Value::known(leaves[0]),
//         path_elements: vec![Value::known(leaves[1]), Value::known(h2)],
//         path_indices: vec![
//             Value::known(PallasFp::from(0)),
//             Value::known(PallasFp::from(0)),
//         ],
//     };

//     let prover = MockProver::run(4, &circuit, vec![vec![root]]).unwrap();
//     assert!(prover.verify().is_ok());

//     // Let's fake the proof
//     let fake_root = PallasFp::from(0x1234);
//     let mut prover = MockProver::run(4, &circuit, vec![vec![fake_root]]).unwrap();
//     let advice = prover.advice_mut(2);
//     advice[4] = CellValue::Assigned(fake_root);
//     assert!(prover.verify().is_err());
// }
