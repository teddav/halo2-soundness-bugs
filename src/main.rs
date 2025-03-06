use std::marker::PhantomData;

use halo2_poseidon::poseidon::primitives::{ConstantLength, Hash, P128Pow5T3 as OrchardNullifier};
use halo2_proofs::{
    arithmetic::Field,
    circuit::Value,
    dev::{CellValue, MockProver},
    halo2curves::{bn256::Fr as Fp, pasta::pallas::Base as PallasFp},
    plonk::Circuit,
};
use rand::rngs::OsRng;

// mod simple_circuit;
// use simple_circuit::SimpleCircuit;

// fn main() {
//     let circuit = SimpleCircuit {
//         a: Value::known(Fp::from(3)),
//         b: Value::known(Fp::from(4)),
//     };
//     let public_value = Fp::from(7);

//     let k = 4;
//     let prover = MockProver::run(k, &circuit, vec![vec![public_value]]).unwrap();
//     println!("prover: {:#?}", prover);
//     assert!(prover.verify().is_ok());
// }

mod casino1;
use casino1::CasinoCircuit;

mod casino2;
use casino2::CasinoCircuitConstrained;

mod non_deterministic;
use non_deterministic::NonDeterministicCircuit;

mod hash;
use hash::HashCircuit;

mod merkle;
use merkle::MerkleCircuit;

mod merkle_nohash1;
mod merkle_nohash2;
mod merkle_nohash3;
mod merkle_nohash4;
mod merkle_nohash5;
use merkle_nohash1::MerkleCircuitNoHash1;
use merkle_nohash2::MerkleCircuitNoHash2;
use merkle_nohash3::MerkleCircuitNoHash3;
use merkle_nohash4::MerkleCircuitNoHash4;
use merkle_nohash5::MerkleCircuitNoHash5;

// fn main() {
//     let circuit = CasinoCircuit {
//         deposits: vec![
//             Value::known(Fp::from(300_000)),
//             Value::known(Fp::from(400_000)),
//             Value::known(Fp::from(500_000)),
//         ],
//     };
//     let total = Fp::from(950_000);
//     let mut prover = MockProver::run(4, &circuit, vec![vec![total]]).unwrap();

//     // let's modify the advice column
//     let advice = prover.advice_mut(0);
//     advice[3] = CellValue::Assigned(total);

//     assert!(prover.verify().is_ok());

//     // Fake the constrained circuit
//     let mut deposits = vec![
//         Value::known(Fp::from(300_000)),
//         Value::known(Fp::from(400_000)),
//         Value::known(Fp::from(500_000)),
//     ];
//     let target = Fp::from(950_000);

//     let sum = deposits
//         .iter()
//         .fold(Value::known(Fp::zero()), |acc, val| acc + val);
//     let fake_deposit = Value::known(Fp::zero()) - sum + Value::known(target);
//     deposits.push(fake_deposit);

//     let circuit = CasinoCircuitConstrained { deposits };
//     let total = target;
//     let prover = MockProver::run(4, &circuit, vec![vec![total]]).unwrap();
//     assert!(prover.verify().is_ok());

//     let a = Fp::from(3);
//     let fake_a = Fp::zero() - a;
//     println!("fake_a: {fake_a:?}");
//     let b = a.square();
//     let circuit = NonDeterministicCircuit {
//         a: Value::known(fake_a),
//         b: Value::known(b),
//     };
//     let total = b;
//     let prover = MockProver::run(4, &circuit, vec![vec![total]]).unwrap();
//     assert!(prover.verify().is_ok());
// }

// fn main() {
//     let message = [PallasFp::from(1), PallasFp::from(2), PallasFp::from(3)];
//     println!("message: {:#?}", message);
//     let output = Hash::<_, OrchardNullifier, ConstantLength<3>, 3, 2>::init().hash(message);
//     println!("output: {:#?}", output);

//     let circuit = HashCircuit::<OrchardNullifier, 3, 2, 3> {
//         message: Value::known(message),
//         output: Value::known(output),
//         _spec: PhantomData,
//     };

//     let prover = MockProver::run(7, &circuit, vec![]).unwrap();
//     println!("prover: {:#?}", prover);
//     assert!(prover.verify().is_ok());
// }

fn main() {
    // let leaves = [
    //     PallasFp::from(1),
    //     PallasFp::from(2),
    //     PallasFp::from(3),
    //     PallasFp::from(4),
    // ];
    // let h1 =
    //     Hash::<_, OrchardNullifier, ConstantLength<2>, 3, 2>::init().hash([leaves[0], leaves[1]]);
    // let h2 =
    //     Hash::<_, OrchardNullifier, ConstantLength<2>, 3, 2>::init().hash([leaves[2], leaves[3]]);
    // let root = Hash::<_, OrchardNullifier, ConstantLength<2>, 3, 2>::init().hash([h1, h2]);
    // println!("root: {root:?}");

    // let circuit = MerkleCircuit::<OrchardNullifier, 3, 2> {
    //     leaf: Value::known(leaves[0]),
    //     path_elements: vec![Value::known(leaves[1]), Value::known(h2)],
    //     path_indices: vec![
    //         Value::known(PallasFp::from(0)),
    //         Value::known(PallasFp::from(0)),
    //     ],
    //     _spec: PhantomData,
    // };

    // let prover = MockProver::run(10, &circuit, vec![]).unwrap();
    // assert!(prover.verify().is_ok());

    // merke_nohash1();
    // merke_nohash2();
    // merke_nohash3();
    // merke_nohash4();
    merke_nohash5();
}

fn merke_nohash1() {
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

fn merke_nohash2() {
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

fn merke_nohash3() {
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

fn merke_nohash4() {
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

fn merke_nohash5() {
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
