use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    halo2curves::bn256::Fr as Fp,
    plonk::{Advice, Circuit, Column, ConstraintSystem, ErrorFront, Instance, Selector},
    poly::Rotation,
};

#[derive(Debug, Default, Clone)]
pub struct CasinoCircuitConstrained {
    pub deposits: Vec<Value<Fp>>,
}

#[derive(Clone, Debug)]
pub struct CasinoConfig {
    deposits: Column<Advice>,
    sum: Column<Advice>,
    public: Column<Instance>,
    myselector: Selector,
    myselector2: Selector,
}

impl Circuit<Fp> for CasinoCircuitConstrained {
    type Config = CasinoConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        let deposits = meta.advice_column();
        let sum = meta.advice_column();
        let public = meta.instance_column();
        let myselector = meta.selector();
        let myselector2 = meta.selector();
        meta.enable_equality(deposits);
        meta.enable_equality(sum);
        meta.enable_equality(public);

        meta.create_gate("running sum", |meta| {
            let s = meta.query_selector(myselector);

            let dcur = meta.query_advice(deposits, Rotation::cur());
            let sumprev = meta.query_advice(sum, Rotation::prev());
            let sumcur = meta.query_advice(sum, Rotation::cur());

            vec![s * (sumprev + dcur - sumcur)]
        });

        meta.create_gate("total", |meta| {
            let s = meta.query_selector(myselector2);

            let sumprev = meta.query_advice(sum, Rotation::prev());
            let sumcur = meta.query_advice(sum, Rotation::cur());

            vec![s * (sumprev - sumcur)]
        });

        CasinoConfig {
            deposits,
            sum,
            public,
            myselector,
            myselector2,
        }
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
                let mut sum = Value::known(Fp::zero());

                for deposit in self.deposits.iter() {
                    if i > 0 {
                        config.myselector.enable(&mut region, i)?;
                    }

                    deposits.push(region.assign_advice(
                        || "deposit",
                        config.deposits,
                        i,
                        || *deposit,
                    )?);

                    sum = sum + *deposit;
                    region.assign_advice(|| "sum", config.sum, i, || sum)?;

                    i += 1;
                }

                config.myselector2.enable(&mut region, i)?;

                let total = deposits
                    .iter()
                    .fold(Value::known(Fp::zero()), |acc, d| acc + d.value());

                region.assign_advice(|| "out", config.sum, i, || total)
            },
        )?;

        layouter.constrain_instance(out.cell(), config.public, 0)?;

        Ok(())
    }
}
