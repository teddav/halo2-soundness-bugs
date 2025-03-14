use halo2_proofs::{
    circuit::Value,
    dev::{CellValue, MockProver},
    halo2curves::bn256::Fr as Fp,
};

mod casino0;
mod casino1;
mod casino2;

pub fn casino() {
    // this is the total amount of money in the casino
    // that's what the owner needs to prove
    let total = Fp::from(0x950);
    let mut deposits = vec![
        Value::known(Fp::from(0x300)),
        Value::known(Fp::from(0x400)),
        Value::known(Fp::from(0x500)),
    ];

    let circuit = casino0::CasinoCircuit {
        deposits: deposits.clone(),
    };
    let mut prover = MockProver::run(4, &circuit, vec![vec![total]]).unwrap();

    // if we decide to cheat, we can modify the advice column
    let advice = prover.advice_mut(0);
    // set the last deposit to the `total`
    advice[3] = CellValue::Assigned(total);
    // and the proof succeeds
    assert!(prover.verify().is_ok());

    // ======================================================
    //in `casino1` we added gate, so the previous technique won't work anymore. But...
    // the values are not range checked, so we can add a fake value and overflow the Field prime 😱
    let sum = deposits
        .iter()
        .fold(Value::known(Fp::zero()), |acc, val| acc + val);
    let fake_deposit = Value::known(Fp::zero()) - sum + Value::known(total);
    deposits.push(fake_deposit);

    let circuit = casino1::CasinoCircuit {
        deposits: deposits.clone(),
    };
    let prover = MockProver::run(4, &circuit, vec![vec![total]]).unwrap();
    // the proof succeeds, but the input is not valid!
    assert!(prover.verify().is_ok());

    // ======================================================
    // in `casino2` we added a lookup table, so the previous technique won't work anymore.
    // notice that we need to make K higher since we now use at least 1000 rows
    let circuit = casino2::CasinoCircuit { deposits };
    let prover = MockProver::run(10, &circuit, vec![vec![total]]).unwrap();
    // the proof fails! 🙏
    assert!(prover.verify().is_err());

    // let's try with deposits within range
    let deposits = vec![
        Value::known(Fp::from(300)),
        Value::known(Fp::from(400)),
        Value::known(Fp::from(500)),
    ];
    let circuit = casino2::CasinoCircuit { deposits };
    let prover = MockProver::run(10, &circuit, vec![vec![total]]).unwrap();
    // the proof fails again, since the deposits do not equal to the total
    assert!(prover.verify().is_err());

    // we make the deposits equal to the total (0x950 == 2384)
    let deposits = vec![
        Value::known(Fp::from(700)),
        Value::known(Fp::from(800)),
        Value::known(Fp::from(884)),
    ];
    let circuit = casino2::CasinoCircuit { deposits };
    let prover = MockProver::run(10, &circuit, vec![vec![total]]).unwrap();
    assert!(prover.verify().is_ok());
}
