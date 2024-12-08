use halo2_proofs::{circuit::Value, dev::MockProver, halo2curves::bn256::Fr as Fp};

mod simple_circuit;
use simple_circuit::SimpleCircuit;

fn main() {
    let circuit = SimpleCircuit {
        a: Value::known(Fp::from(3)),
        b: Value::known(Fp::from(4)),
    };
    let public_value = Fp::from(7);

    let k = 4;
    let prover = MockProver::run(k, &circuit, vec![vec![public_value]]).unwrap();
    assert!(prover.verify().is_ok());
}
