use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    halo2curves::bn256::Fr as Fp,
    plonk::{Advice, Circuit, Column, ConstraintSystem, ErrorFront, Instance, Selector},
    poly::Rotation,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct MultiplicationCircuit {
    pub a: Value<Fp>,
    pub b: Value<Fp>,
}

#[derive(Clone, Debug)]
pub struct MultiplicationConfig {
    advice: Column<Advice>,
    instance: Column<Instance>,
    myselector: Selector,
}

impl Circuit<Fp> for MultiplicationCircuit {
    type Config = MultiplicationConfig;
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

        meta.create_gate("mul gate", |meta| {
            let s = meta.query_selector(myselector);
            let a = meta.query_advice(advice, Rotation(0));
            let b = meta.query_advice(advice, Rotation(1));
            let c = meta.query_advice(advice, Rotation(2));
            vec![s * (a * b - c)]
        });

        MultiplicationConfig {
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

                let a = region.assign_advice(|| "a", config.advice, 0, || self.a)?;
                let b = region.assign_advice(|| "b", config.advice, 1, || self.b)?;
                region.assign_advice(|| "out", config.advice, 2, || a.value() * b.value())
            },
        )?;

        layouter.constrain_instance(out.cell(), config.instance, 0)?;

        Ok(())
    }
}
