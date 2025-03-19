use halo2_proofs::{
    arithmetic::Field,
    circuit::{AssignedCell, Layouter, SimpleFloorPlanner, Value},
    halo2curves::pasta::Fp,
    plonk::{
        Advice, Circuit, Column, ConstraintSystem, ErrorFront, Expression, Instance, Selector,
    },
    poly::Rotation,
};

#[derive(Debug, Clone)]
pub struct MerkleConfig {
    pub merkle: [Column<Advice>; 3],
    pub swap_selector: Selector,
    pub swap_bit_bool_selector: Selector,
    pub hash_selector: Selector,
    pub root_hash: Column<Instance>,
}

#[derive(Debug, Default, Clone)]
pub struct MerkleCircuitNoHash1 {
    pub leaf: Value<Fp>,
    pub path_elements: Vec<Value<Fp>>,
    pub path_indices: Vec<Value<Fp>>,
}

impl Circuit<Fp> for MerkleCircuitNoHash1 {
    type Config = MerkleConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> MerkleConfig {
        let advice = [
            meta.advice_column(),
            meta.advice_column(),
            meta.advice_column(),
        ];
        meta.enable_equality(advice[0]);
        meta.enable_equality(advice[1]);
        meta.enable_equality(advice[2]);

        let root_hash = meta.instance_column();
        meta.enable_equality(root_hash);

        let swap_selector = meta.selector();
        let swap_bit_bool_selector = meta.selector();
        let hash_selector = meta.selector();

        meta.create_gate("bool constraint", |meta| {
            let s = meta.query_selector(swap_bit_bool_selector);
            let swap_bit = meta.query_advice(advice[2], Rotation::cur());
            vec![s * swap_bit.clone() * (Expression::Constant(Fp::from(1)) - swap_bit)]
        });

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

        meta.create_gate("root hash constraint", |meta| {
            let s = meta.query_selector(hash_selector);
            let left = meta.query_advice(advice[0], Rotation::cur());
            let right = meta.query_advice(advice[1], Rotation::cur());
            let hash = meta.query_advice(advice[2], Rotation::cur());
            vec![s * (left + right - hash)]
        });

        MerkleConfig {
            merkle: advice,
            swap_selector,
            swap_bit_bool_selector,
            hash_selector,
            root_hash,
        }
    }

    fn synthesize(
        &self,
        config: MerkleConfig,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), ErrorFront> {
        let leaf_cell = layouter.assign_region(
            || "assign leaf",
            |mut region| region.assign_advice(|| "assign leaf", config.merkle[0], 0, || self.leaf),
        )?;

        let mut digest: AssignedCell<Fp, Fp> = leaf_cell;
        for i in 0..self.path_elements.len() {
            digest = self.merkle_prove_layer(
                config.clone(),
                layouter.namespace(|| "prove tree"),
                &digest,
                self.path_elements[i],
                self.path_indices[i],
            )?;
        }

        layouter.constrain_instance(digest.cell(), config.root_hash, 0)
    }
}

impl MerkleCircuitNoHash1 {
    pub fn merkle_prove_layer(
        &self,
        config: MerkleConfig,
        mut layouter: impl Layouter<Fp>,
        node_cell: &AssignedCell<Fp, Fp>,
        neighbor: Value<Fp>,
        swap_bit: Value<Fp>,
    ) -> Result<AssignedCell<Fp, Fp>, ErrorFront> {
        layouter.assign_region(
            || "merkle prove",
            |mut region| {
                config.swap_selector.enable(&mut region, 0)?;
                config.swap_bit_bool_selector.enable(&mut region, 0)?;
                // we enable the hash selector on the second row
                config.hash_selector.enable(&mut region, 1)?;

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

                region.assign_advice(|| "left node to be hashed", config.merkle[0], 1, || left)?;
                region.assign_advice(
                    || "right node to be hashed",
                    config.merkle[1],
                    1,
                    || right,
                )?;
                Ok(region.assign_advice(|| "result", config.merkle[2], 1, || left + right)?)
            },
        )
    }
}
