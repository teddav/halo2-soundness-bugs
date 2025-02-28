use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    halo2curves::bn256::Fr as Fp,
    plonk::{
        Advice, Circuit, Column, ConstraintSystem, ErrorFront, Instance, Selector, TableColumn,
    },
    poly::Rotation,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct NonDeterministicCircuit {
    pub a: Value<Fp>,
    pub b: Value<Fp>,
}

#[derive(Clone, Debug)]
pub struct NonDeterministicConfig {
    advice: Column<Advice>,
    instance: Column<Instance>,
    myselector: Selector,
}

impl Circuit<Fp> for NonDeterministicCircuit {
    type Config = NonDeterministicConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        let advice = meta.advice_column();
        let instance = meta.instance_column();
        meta.enable_equality(advice);
        meta.enable_equality(instance);

        let myselector = meta.selector();

        meta.create_gate("square", |meta| {
            let s = meta.query_selector(myselector);
            let a = meta.query_advice(advice, Rotation::cur());
            let b = meta.query_advice(advice, Rotation::next());
            vec![s * (a.clone() * a - b)]
        });

        NonDeterministicConfig {
            advice,
            instance,
            myselector,
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
                config.myselector.enable(&mut region, 0)?;

                region.assign_advice(|| "a", config.advice, 0, || self.a)?;
                region.assign_advice(|| "b", config.advice, 1, || self.b)
            },
        )?;

        layouter.constrain_instance(out.cell(), config.instance, 0)?;
        Ok(())
    }
}
