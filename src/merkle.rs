use halo2_poseidon::poseidon::{
    primitives::{ConstantLength, Spec},
    Hash, Pow5Chip, Pow5Config,
};
use halo2_proofs::{
    arithmetic::Field,
    circuit::{AssignedCell, Layouter, SimpleFloorPlanner, Value},
    halo2curves::pasta::Fp,
    plonk::{Advice, Circuit, Column, ConstraintSystem, ErrorFront, Expression, Selector},
    poly::Rotation,
};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct MerkleConfig<F: Field, const WIDTH: usize, const RATE: usize> {
    pub pow5config: Pow5Config<F, WIDTH, RATE>,
    pub merkle: [Column<Advice>; 3],
    pub swap_selector: Selector,
    pub swap_bit_bool_selector: Selector,
}

#[derive(Debug, Default, Clone)]
pub struct MerkleCircuit<S: Spec<Fp, WIDTH, RATE>, const WIDTH: usize, const RATE: usize> {
    pub leaf: Value<Fp>,
    pub path_elements: Vec<Value<Fp>>,
    pub path_indices: Vec<Value<Fp>>,
    pub _spec: PhantomData<S>,
}

impl<S: Spec<Fp, WIDTH, RATE>, const WIDTH: usize, const RATE: usize> Circuit<Fp>
    for MerkleCircuit<S, WIDTH, RATE>
{
    type Config = MerkleConfig<Fp, WIDTH, RATE>;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {
            leaf: Value::unknown(),
            path_elements: vec![],
            path_indices: vec![],
            _spec: PhantomData,
        }
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> MerkleConfig<Fp, WIDTH, RATE> {
        let state = (0..WIDTH).map(|_| meta.advice_column()).collect::<Vec<_>>();
        let partial_sbox = meta.advice_column();

        let rc_a = (0..WIDTH).map(|_| meta.fixed_column()).collect::<Vec<_>>();
        let rc_b = (0..WIDTH).map(|_| meta.fixed_column()).collect::<Vec<_>>();

        meta.enable_constant(rc_b[0]);

        let pow5config = Pow5Chip::configure::<S>(
            meta,
            state.try_into().unwrap(),
            partial_sbox,
            rc_a.try_into().unwrap(),
            rc_b.try_into().unwrap(),
        );

        let advice = [
            meta.advice_column(),
            meta.advice_column(),
            meta.advice_column(),
        ];
        meta.enable_equality(advice[0]);
        meta.enable_equality(advice[1]);
        meta.enable_equality(advice[2]);

        let swap_selector = meta.selector();
        let swap_bit_bool_selector = meta.selector();

        // we check that the `swap_bit` (advice[2]) is either `0` or `1`
        meta.create_gate("bool constraint", |meta| {
            let s = meta.query_selector(swap_bit_bool_selector);
            let swap_bit = meta.query_advice(advice[2], Rotation::cur());
            vec![s * swap_bit.clone() * (Expression::Constant(Fp::from(1)) - swap_bit)]
        });

        // if the swap selector is on (on the first row)
        // then we check the `swap_bit`
        // If it's on (1) -> we make sure the leaves are swapped on the next row
        meta.create_gate("swap constraint", |meta| {
            let s = meta.query_selector(swap_selector);
            let swap_bit = meta.query_advice(advice[2], Rotation::cur());

            let left_cur = meta.query_advice(advice[0], Rotation::cur());
            let right_cur = meta.query_advice(advice[1], Rotation::cur());

            let left_next = meta.query_advice(advice[0], Rotation::next());
            let right_next = meta.query_advice(advice[1], Rotation::next());

            let constraint1 = s.clone()
                * ((right_cur.clone() - left_cur.clone()) * swap_bit.clone() + left_cur.clone()
                    - left_next);
            let constraint2 =
                s * ((left_cur - right_cur.clone()) * swap_bit + right_cur - right_next);
            vec![constraint1, constraint2]
        });

        MerkleConfig {
            pow5config,
            merkle: advice,
            swap_selector,
            swap_bit_bool_selector,
        }
    }

    fn synthesize(
        &self,
        config: MerkleConfig<Fp, WIDTH, RATE>,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), ErrorFront> {
        let chip = Pow5Chip::construct(config.pow5config.clone());

        let leaf_cell = layouter.assign_region(
            || "assign leaf",
            |mut region| region.assign_advice(|| "assign leaf", config.merkle[0], 0, || self.leaf),
        )?;

        let mut digest: AssignedCell<Fp, Fp> = leaf_cell;
        for i in 0..self.path_elements.len() {
            digest = self.merkle_prove_layer(
                config.clone(),
                chip.clone(),
                layouter.namespace(|| "prove tree"),
                &digest,
                self.path_elements[i],
                self.path_indices[i],
            )?;
        }
        println!("digest: {:?}", digest);
        // let output = hasher.hash(layouter.namespace(|| "hash"), message)?;

        // layouter.assign_region(
        //     || "constrain output",
        //     |mut region| {
        //         let expected_var = region.assign_advice(
        //             || "load output",
        //             config.pow5config.state[0],
        //             0,
        //             || self.output,
        //         )?;
        //         region.constrain_equal(output.cell(), expected_var.cell())
        //     },
        // )
        Ok(())
    }
}

impl<S: Spec<Fp, WIDTH, RATE>, const WIDTH: usize, const RATE: usize>
    MerkleCircuit<S, WIDTH, RATE>
{
    pub fn merkle_prove_layer(
        &self,
        config: MerkleConfig<Fp, WIDTH, RATE>,
        hash_chip: Pow5Chip<Fp, WIDTH, RATE>,
        mut layouter: impl Layouter<Fp>,
        node_cell: &AssignedCell<Fp, Fp>,
        neighbor: Value<Fp>,
        swap_bit: Value<Fp>,
    ) -> Result<AssignedCell<Fp, Fp>, ErrorFront> {
        let hasher = Hash::<_, _, S, ConstantLength<2>, WIDTH, RATE>::init(
            hash_chip,
            layouter.namespace(|| "init"),
        )?;

        let (left, right) = layouter.assign_region(
            || "merkle prove",
            |mut region| {
                config.swap_selector.enable(&mut region, 0)?;
                config.swap_bit_bool_selector.enable(&mut region, 0)?;

                node_cell.copy_advice(
                    || "copy previous node cell",
                    &mut region,
                    config.merkle[0],
                    0,
                )?;
                region.assign_advice(|| "set neighbor node", config.merkle[1], 0, || neighbor)?;
                region.assign_advice(|| "set swap bit", config.merkle[2], 0, || swap_bit)?;

                let mut left = node_cell.value().cloned();
                let mut right = neighbor;
                swap_bit.map(|f| {
                    (left, right) = if f == Fp::ZERO {
                        (left, right)
                    } else {
                        (right, left)
                    }
                });

                let left_cell = region.assign_advice(
                    || "left node to be hashed",
                    config.merkle[0],
                    1,
                    || left,
                )?;
                let right_cell = region.assign_advice(
                    || "right node to be hashed",
                    config.merkle[1],
                    1,
                    || right,
                )?;

                Ok((left_cell, right_cell))
            },
        )?;

        let result_hash_cell = hasher.hash(layouter.namespace(|| "hash row"), [left, right])?;
        Ok(result_hash_cell)
    }
}
