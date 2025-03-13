use halo2_proofs::{
    circuit::Value,
    dev::{CellValue, MockProver},
    halo2curves::bn256::Fr as Fp,
};

mod mul0;
mod mul1;
mod mul2;

pub fn multiplication() {
    let result = Fp::from(12);

    // ======================================================
    // mul0: our vulnerable circuit
    let circuit0 = mul0::MultiplicationCircuit {
        a: Value::known(Fp::from(2)),
        b: Value::known(Fp::from(3)),
    };
    let prover = MockProver::run(4, &circuit0, vec![vec![result]]).unwrap();

    // 2 * 3 == 12 ???
    // result is wrong, but the proof is still valid
    assert!(prover.verify().is_ok());

    // ======================================================
    // we need to constrain the result, that's what we do in mul1.rs

    let circuit1 = mul1::MultiplicationCircuit {
        a: Value::known(Fp::from(2)),
        b: Value::known(Fp::from(3)),
    };
    let prover = MockProver::run(4, &circuit1, vec![vec![result]]).unwrap();
    // we get an error, as we should!
    assert!(prover.verify().is_err());

    // ======================================================
    // now let's modify the witness in order to forge our proof
    let circuit1_bis = mul1::MultiplicationCircuit {
        a: Value::known(Fp::from(2)),
        b: Value::known(Fp::from(3)),
    };
    let mut prover = MockProver::run(4, &circuit1_bis, vec![vec![result]]).unwrap();
    // we modify directly in the advice the `out` cell (the result of `a * b`)
    let advice = prover.advice_mut(0);
    advice[2] = CellValue::Assigned(result);
    // now the proof is valid! 😱
    assert!(prover.verify().is_ok());

    // ======================================================
    // We added a gate in mul2, this should prevent a cheater from modifying the witness
    let circuit2 = mul2::MultiplicationCircuit {
        a: Value::known(Fp::from(2)),
        b: Value::known(Fp::from(3)),
    };
    let mut prover = MockProver::run(4, &circuit2, vec![vec![result]]).unwrap();
    // we modify directly in the advice the `out` cell (the result of `a * b`)
    let advice = prover.advice_mut(0);
    advice[2] = CellValue::Assigned(result);
    // even if we try to modify the witness, the proof is not valid
    assert!(prover.verify().is_err());

    let circuit2_bis = mul2::MultiplicationCircuit {
        a: Value::known(Fp::from(3)),
        b: Value::known(Fp::from(4)),
    };
    let prover = MockProver::run(4, &circuit2_bis, vec![vec![result]]).unwrap();
    assert!(prover.verify().is_ok());
}
