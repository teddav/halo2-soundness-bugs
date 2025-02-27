use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    halo2curves::bn256::Fr as Fp,
    plonk::{
        Advice, Circuit, Column, ConstraintSystem, ErrorFront, Instance, Selector, TableColumn,
    },
    poly::Rotation,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct UnderconstrainedCircuit {
    pub a: Value<Fp>,
    pub b: Value<Fp>,
}

#[derive(Clone, Debug)]
pub struct UnderconstrainedConfig {
    advice: Column<Advice>,
    instance: Column<Instance>,
    lookup_table: TableColumn,
    myselector: Selector,
}

impl Circuit<Fp> for UnderconstrainedCircuit {
    type Config = UnderconstrainedConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        let advice = meta.advice_column();
        let instance = meta.instance_column();
        meta.enable_equality(advice);
        meta.enable_equality(instance);

        let lookup_table = meta.lookup_table_column();

        let myselector = meta.selector();

        meta.create_gate("random_gate", |meta| {
            let s = meta.query_selector(myselector);
            let a = meta.query_advice(advice, Rotation(0));
            let b = meta.query_advice(advice, Rotation(1));
            let c = meta.query_advice(advice, Rotation(2));
            vec![s * (a + b - c)]
        });

        meta.lookup("range_check_constraint", |meta| {
            let value = meta.query_advice(advice, Rotation::cur());
            vec![(value, lookup_table)]
        });

        UnderconstrainedConfig {
            advice,
            instance,
            lookup_table,
            myselector,
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
                for (i, v) in (0..9).into_iter().enumerate() {
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
                config.myselector.enable(&mut region, 0)?;

                let a = region.assign_advice(|| "a", config.advice, 0, || self.a)?;
                let b = region.assign_advice(|| "b", config.advice, 1, || self.b)?;
                region.assign_advice(|| "out", config.advice, 2, || a.value() + b.value())
            },
        )?;

        layouter.constrain_instance(out.cell(), config.instance, 0)?;

        Ok(())
    }
}
