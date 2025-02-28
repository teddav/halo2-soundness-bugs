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
//     let rng = OsRng;

//     // let message = [PallasFp::random(rng), PallasFp::random(rng)];
//     let message = [PallasFp::from(1), PallasFp::from(2)];
//     println!("message: {:#?}", message);
//     let output = Hash::<_, OrchardNullifier, ConstantLength<2>, 3, 2>::init().hash(message);
//     println!("output: {:#?}", output);

//     let circuit = HashCircuit::<OrchardNullifier, 3, 2, 2> {
//         message: Value::known(message),
//         output: Value::known(output),
//         _spec: PhantomData,
//     };

//     let prover = MockProver::run(6, &circuit, vec![]).unwrap();
//     // println!("prover: {:#?}", prover);
//     assert!(prover.verify().is_ok());
// }

fn main() {
    let rng = OsRng;

    // let message = [PallasFp::random(rng), PallasFp::random(rng)];
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

    let prover = MockProver::run(10, &circuit, vec![]).unwrap();
    assert!(prover.verify().is_ok());
}
