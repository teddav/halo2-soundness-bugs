use halo2_proofs::{circuit::Value, dev::MockProver, halo2curves::bn256::Fr as Fp};

mod casino;
mod hash;
mod merkle;
mod mul;
mod non_deterministic;

use non_deterministic::NonDeterministicCircuit;

fn main() {
    mul::multiplication();
    casino::casino();

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
