use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    halo2curves::bn256::Fr as Fp,
    plonk::{
        Advice, Circuit, Column, ConstraintSystem, ErrorFront, Instance, Selector, TableColumn,
    },
    poly::Rotation,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct SquareRootCircuit {
    pub root: Value<Fp>,
}

#[derive(Clone, Debug)]
pub struct SquareRootConfig {
    input: Column<Advice>,
    squared: Column<Advice>,
    instance: Column<Instance>,
    myselector: Selector,
    lookup_table: TableColumn,
}

impl Circuit<Fp> for SquareRootCircuit {
    type Config = SquareRootConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        let input = meta.advice_column();
        let squared = meta.advice_column();
        let instance = meta.instance_column();
        meta.enable_equality(input);
        meta.enable_equality(squared);
        meta.enable_equality(instance);

        let myselector = meta.selector();

        meta.create_gate("square", |meta| {
            let s = meta.query_selector(myselector);
            let root = meta.query_advice(input, Rotation::cur());
            let square = meta.query_advice(squared, Rotation::cur());
            vec![s * (root.clone() * root - square)]
        });

        let lookup_table = meta.lookup_table_column();

        meta.lookup("range_check_constraint", |meta| {
            let value = meta.query_advice(input, Rotation::cur());
            vec![(value, lookup_table)]
        });

        SquareRootConfig {
            input,
            squared,
            instance,
            myselector,
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
                for (i, v) in (0..=10).into_iter().enumerate() {
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
                region.assign_advice(|| "a", config.input, 0, || self.root)?;
                region.assign_advice(|| "square", config.squared, 0, || self.root * self.root)
            },
        )?;

        layouter.constrain_instance(out.cell(), config.instance, 0)?;
        Ok(())
    }
}
