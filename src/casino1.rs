use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    halo2curves::bn256::Fr as Fp,
    plonk::{Advice, Circuit, Column, ConstraintSystem, ErrorFront, Instance},
    poly::Rotation,
};

#[derive(Debug, Default, Clone)]
pub struct CasinoCircuit {
    pub deposits: Vec<Value<Fp>>,
}

#[derive(Clone, Debug)]
pub struct CasinoConfig {
    advice: Column<Advice>,
    instance: Column<Instance>,
}

impl Circuit<Fp> for CasinoCircuit {
    type Config = CasinoConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        let advice = meta.advice_column();
        let instance = meta.instance_column();
        meta.enable_equality(advice);
        meta.enable_equality(instance);

        // meta.create_gate("random_gate", |meta| {
        //     let s = meta.query_selector(myselector);
        //     let a = meta.query_advice(advice, Rotation(0));
        //     let b = meta.query_advice(advice, Rotation(1));
        //     let c = meta.query_advice(advice, Rotation(2));
        //     vec![s * (a + b - c)]
        // });

        CasinoConfig { advice, instance }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), ErrorFront> {
        let out = layouter.assign_region(
            || "main region",
            |mut region| {
                let mut deposits = vec![];
                let mut i = 0;

                for deposit in self.deposits.iter() {
                    deposits.push(region.assign_advice(
                        || "deposit",
                        config.advice,
                        i,
                        || *deposit,
                    )?);
                    i += 1;
                }

                let total = deposits
                    .iter()
                    .fold(Value::known(Fp::zero()), |acc, d| acc + d.value());

                region.assign_advice(|| "out", config.advice, i, || total)
            },
        )?;

        layouter.constrain_instance(out.cell(), config.instance, 0)?;

        Ok(())
    }
}
