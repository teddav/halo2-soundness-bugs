use halo2_proofs::{
    circuit::Value,
    dev::{CellValue, MockProver},
    halo2curves::bn256::Fr as Fp,
};

mod casino1;
mod casino2;
mod hash;
mod merkle;
mod mul;
mod non_deterministic;

use casino1::CasinoCircuit;
use casino2::CasinoCircuitConstrained;
use non_deterministic::NonDeterministicCircuit;

fn main() {
    mul::multiplication();

    let a = Fp::from(3);
    let fake_a = Fp::zero() - a;
    println!("fake_a: {fake_a:?}");
    let b = a.square();
    let circuit = NonDeterministicCircuit {
        a: Value::known(fake_a),
        b: Value::known(b),
    };
    let total = b;
    let prover = MockProver::run(4, &circuit, vec![vec![total]]).unwrap();
    assert!(prover.verify().is_ok());

    // hash::hash_circuit();
    // merkle::merke_nohash1();
    // merkle::merke_nohash2();
    // merkle::merke_nohash3();
    // merkle::merke_nohash4();
    // merkle::merke_nohash5();
    // merkle::merke_circuit_with_hash();
}

fn casino() {
    let circuit = CasinoCircuit {
        deposits: vec![
            Value::known(Fp::from(300_000)),
            Value::known(Fp::from(400_000)),
            Value::known(Fp::from(500_000)),
        ],
    };
    let total = Fp::from(950_000);
    let mut prover = MockProver::run(4, &circuit, vec![vec![total]]).unwrap();

    // let's modify the advice column
    let advice = prover.advice_mut(0);
    advice[3] = CellValue::Assigned(total);

    assert!(prover.verify().is_ok());

    // Fake the constrained circuit
    let mut deposits = vec![
        Value::known(Fp::from(300_000)),
        Value::known(Fp::from(400_000)),
        Value::known(Fp::from(500_000)),
    ];
    let target = Fp::from(950_000);

    let sum = deposits
        .iter()
        .fold(Value::known(Fp::zero()), |acc, val| acc + val);
    let fake_deposit = Value::known(Fp::zero()) - sum + Value::known(target);
    deposits.push(fake_deposit);

    let circuit = CasinoCircuitConstrained { deposits };
    let total = target;
    let prover = MockProver::run(4, &circuit, vec![vec![total]]).unwrap();
    assert!(prover.verify().is_ok());
}
