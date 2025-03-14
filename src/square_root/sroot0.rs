use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    halo2curves::bn256::Fr as Fp,
    plonk::{Advice, Circuit, Column, ConstraintSystem, ErrorFront, Instance, Selector},
    poly::Rotation,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct SquareRootCircuit {
    pub root: Value<Fp>,
}

#[derive(Clone, Debug)]
pub struct SquareRootConfig {
    advice: Column<Advice>,
    instance: Column<Instance>,
    myselector: Selector,
}

impl Circuit<Fp> for SquareRootCircuit {
    type Config = SquareRootConfig;
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
            let root = meta.query_advice(advice, Rotation::cur());
            let square = meta.query_advice(advice, Rotation::next());
            vec![s * (root.clone() * root - square)]
        });

        SquareRootConfig {
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
                region.assign_advice(|| "a", config.advice, 0, || self.root)?;
                region.assign_advice(|| "square", config.advice, 1, || self.root * self.root)
            },
        )?;

        layouter.constrain_instance(out.cell(), config.instance, 0)?;
        Ok(())
    }
}
