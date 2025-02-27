use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    halo2curves::bn256::Fr as Fp,
    plonk::{
        Advice, Circuit, Column, ConstraintSystem, ErrorFront, Instance, Selector, TableColumn,
    },
    poly::Rotation,
};

#[derive(Clone, Debug)]
pub struct ChipSimpleConfig {
    pub lookup_table: TableColumn,
    pub myselector: Selector,
}

pub struct ChipSimple;

#[derive(Debug, Default, Clone, Copy)]
pub struct SimpleCircuit {
    pub a: Value<Fp>,
    pub b: Value<Fp>,
}

impl ChipSimple {
    pub fn construct() -> Self {
        Self {}
    }

    pub fn configure(
        meta: &mut ConstraintSystem<Fp>,
        values: Column<Advice>,
        lookup_table: TableColumn,
    ) -> ChipSimpleConfig {
        let myselector = meta.selector();

        meta.create_gate("random_gate", |meta| {
            let s = meta.query_selector(myselector);
            let a = meta.query_advice(values, Rotation(0));
            let b = meta.query_advice(values, Rotation(1));
            let c = meta.query_advice(values, Rotation(2));
            vec![s * (a + b - c)]
        });

        meta.lookup("range_check_constraint", |meta| {
            let value = meta.query_advice(values, Rotation::cur());
            vec![(value, lookup_table)]
        });

        ChipSimpleConfig {
            lookup_table,
            myselector,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CircuitSimpleConfig {
    advice: Column<Advice>,
    instance: Column<Instance>,
    rangecheck_config: ChipSimpleConfig,
}

impl Circuit<Fp> for SimpleCircuit {
    type Config = CircuitSimpleConfig;
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

        let rangecheck_config = ChipSimple::configure(meta, advice, lookup_table);

        CircuitSimpleConfig {
            advice,
            instance,
            rangecheck_config,
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
                        config.rangecheck_config.lookup_table,
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
                config.rangecheck_config.myselector.enable(&mut region, 0)?;

                let a = region.assign_advice(|| "a", config.advice, 0, || self.a)?;
                let b = region.assign_advice(|| "b", config.advice, 1, || self.b)?;
                region.assign_advice(|| "out", config.advice, 2, || a.value() + b.value())
            },
        )?;

        layouter.constrain_instance(out.cell(), config.instance, 0)?;

        Ok(())
    }
}
