use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    halo2curves::bn256::Fr as Fp,
    plonk::{
        Advice, Circuit, Column, ConstraintSystem, ErrorFront, Instance, Selector, TableColumn,
    },
    poly::Rotation,
};

#[derive(Debug, Default, Clone)]
pub struct CasinoCircuit {
    pub deposits: Vec<Value<Fp>>,
}

#[derive(Clone, Debug)]
pub struct CasinoConfig {
    deposits: Column<Advice>,
    sum: Column<Advice>,
    public: Column<Instance>,
    selector_running_sum: Selector,
    selector_first_row: Selector,
    lookup_table: TableColumn,
}

impl Circuit<Fp> for CasinoCircuit {
    type Config = CasinoConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        let deposits = meta.advice_column();
        let sum = meta.advice_column();
        let public = meta.instance_column();
        meta.enable_equality(deposits);
        meta.enable_equality(sum);
        meta.enable_equality(public);

        let selector_running_sum = meta.selector();
        let selector_first_row = meta.selector();

        meta.create_gate("first row", |meta| {
            let s = meta.query_selector(selector_first_row);
            let deposit = meta.query_advice(deposits, Rotation::cur());
            let sum = meta.query_advice(sum, Rotation::cur());
            vec![s.clone() * deposit, s * sum]
        });

        meta.create_gate("running sum", |meta| {
            let s = meta.query_selector(selector_running_sum);

            let dcur = meta.query_advice(deposits, Rotation::cur());
            let sumprev = meta.query_advice(sum, Rotation::prev());
            let sumcur = meta.query_advice(sum, Rotation::cur());

            vec![s * (sumprev + dcur - sumcur)]
        });

        let lookup_table = meta.lookup_table_column();
        meta.lookup("range_check_constraint", |meta| {
            let value = meta.query_advice(deposits, Rotation::cur());
            vec![(value, lookup_table)]
        });

        CasinoConfig {
            deposits,
            sum,
            public,
            selector_running_sum,
            selector_first_row,
            lookup_table,
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), ErrorFront> {
        layouter.assign_table(
            || "assign lookup table",
            |mut table| {
                for (i, v) in (0..1000).into_iter().enumerate() {
                    table.assign_cell(
                        || "assign cell in lookup table",
                        config.lookup_table,
                        i,
                        || Value::known(Fp::from(v as u64)),
                    )?;
                }
                Ok(())
            },
        )?;

        let out = layouter.assign_region(
            || "main region",
            |mut region| {
                // We enable the first row selector
                // values should be 0 on the first row
                config.selector_first_row.enable(&mut region, 0)?;
                region.assign_advice(
                    || "deposit0",
                    config.deposits,
                    0,
                    || Value::known(Fp::zero()),
                )?;
                let mut total =
                    region.assign_advice(|| "sum0", config.sum, 0, || Value::known(Fp::zero()))?;

                // then we loop over deposits, starting from row 1
                let mut deposits = vec![];
                let mut i = 1;
                let mut sum = Value::known(Fp::zero());

                for deposit in self.deposits.iter() {
                    // we enable the running sum selector
                    if i > 1 {
                        config.selector_running_sum.enable(&mut region, i)?;
                    }

                    deposits.push(region.assign_advice(
                        || "deposit",
                        config.deposits,
                        i,
                        || *deposit,
                    )?);

                    sum = sum + *deposit;
                    total = region.assign_advice(|| "sum", config.sum, i, || sum)?;

                    i += 1;
                }

                Ok(total)
            },
        )?;

        layouter.constrain_instance(out.cell(), config.public, 0)?;

        Ok(())
    }
}
