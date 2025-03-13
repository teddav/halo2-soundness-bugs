use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    halo2curves::bn256::Fr as Fp,
    plonk::{Advice, Circuit, Column, ConstraintSystem, ErrorFront, Instance},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct MultiplicationCircuit {
    pub a: Value<Fp>,
    pub b: Value<Fp>,
}

#[derive(Clone, Debug)]
pub struct MultiplicationConfig {
    advice: Column<Advice>,
    #[allow(dead_code)]
    instance: Column<Instance>,
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

        MultiplicationConfig { advice, instance }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), ErrorFront> {
        layouter.assign_region(
            || "main region",
            |mut region| {
                let a = region.assign_advice(|| "a", config.advice, 0, || self.a)?;
                let b = region.assign_advice(|| "b", config.advice, 1, || self.b)?;
                region.assign_advice(|| "out", config.advice, 2, || a.value() * b.value())
            },
        )?;

        Ok(())
    }
}
