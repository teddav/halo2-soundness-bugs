use halo2_proofs::{circuit::Value, dev::MockProver, halo2curves::bn256::Fr as Fp};

mod sroot0;
mod sroot1;

pub fn square_root() {
    let n = Fp::from(9);
    let root = Fp::from(3);

    // we want the square root of 9
    // the circuit expects 3, but we can generate a fake proof for -3
    let fake_root = Fp::zero() - root; // -3
    let circuit = sroot0::SquareRootCircuit {
        root: Value::known(fake_root),
    };
    let prover = MockProver::run(3, &circuit, vec![vec![n]]).unwrap();
    assert!(prover.verify().is_ok());

    // ======================================================
    // if we try to pass a fake root, the proof will fail
    let fake_root = Fp::zero() - root; // -3
    let circuit = sroot1::SquareRootCircuit {
        root: Value::known(fake_root),
    };
    let prover = MockProver::run(5, &circuit, vec![vec![n]]).unwrap();
    assert!(prover.verify().is_err());

    let circuit = sroot1::SquareRootCircuit {
        root: Value::known(root),
    };
    let prover = MockProver::run(5, &circuit, vec![vec![n]]).unwrap();
    assert!(prover.verify().is_ok());
}
