use plonky2::{
    field::{goldilocks_field::GoldilocksField, types::Field},
    hash::poseidon::PoseidonHash,
    iop::witness::{PartialWitness, WitnessWrite},
    plonk::{
        circuit_builder::CircuitBuilder,
        circuit_data::CircuitConfig,
        config::{Hasher, PoseidonGoldilocksConfig},
    },
};

use anyhow::Result;

fn main() -> Result<()> {
    const D: usize = 2;
    type F = GoldilocksField;
    type C = PoseidonGoldilocksConfig;
    type H = PoseidonHash;

    let a = F::from_canonical_u64(42);
    let hash_a = H::hash_no_pad(&[a.clone()]); // hash(a)

    println!("a = {}, hash(a) = {:?}", a, hash_a);

    // Proof that "I know x that hash(x) = hash_a"
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    let hash_a_t = builder.constant_hash(hash_a); // add constant hash target
    let x_t = builder.add_virtual_target();
    let hash_x_t = builder.hash_n_to_hash_no_pad::<H>(vec![x_t]);

    builder.connect_hashes(hash_x_t, hash_a_t); // hash(x) = hash_a

    let data = builder.build::<C>();

    let mut pw = PartialWitness::<F>::new();
    pw.set_target(x_t, F::from_canonical_u64(42)); // x = 42

    let proof = data.prove(pw)?;
    data.verify(proof)?;

    Ok(())
}
