use halo2_proofs::{
    circuit::Value,
    dev::{CellValue, MockProver},
    halo2curves::{bn256::Fr as Fp, ff::PrimeField},
};

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
fn main() {
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
    let MODULUS = "21888242871839275222246405745257275088548364400416034343698204186575808495616"; // Fp::MODULUS;
    let modulus = Value::known(Fp::from_str_vartime(&MODULUS).unwrap());
    let target = Fp::from(950_000);

    let sum = deposits
        .iter()
        .fold(Value::known(Fp::zero()), |acc, val| acc + val);
    let fake_deposit = modulus - sum + Value::known(target) + Value::known(Fp::from(1));
    deposits.push(fake_deposit);

    let circuit = CasinoCircuitConstrained { deposits };
    let total = target;
    let prover = MockProver::run(4, &circuit, vec![vec![total]]).unwrap();
    println!("prover: {:#?}", prover);
    assert!(prover.verify().is_ok());
}
