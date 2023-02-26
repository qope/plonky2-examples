use plonky2::{
    field::{goldilocks_field::GoldilocksField, types::Field},
    iop::witness::{PartialWitness, WitnessWrite},
    plonk::{
        circuit_builder::CircuitBuilder, circuit_data::CircuitConfig,
        config::PoseidonGoldilocksConfig,
    },
};

use anyhow::Result;

// If main function returns Err, Rust prints error code and a debug information.
fn main() -> Result<()> {
    // Proof that "I know x that satisfies x^2 - 2x + 1 = 0"

    // config defines number of wires of gates, FRI strategies etc.
    let config = CircuitConfig::standard_recursion_config();

    // We use GoldilocksField as circuit arithmetization
    type F = GoldilocksField;

    // We use Poseidon hash on GoldilocksField as FRI hasher
    type C = PoseidonGoldilocksConfig;

    // We use the degree D extension Field when soundness is required.
    const D: usize = 2;

    let mut builder = CircuitBuilder::<F, D>::new(config.clone());

    let x_t = builder.add_virtual_target();
    let minus_x_t = builder.neg(x_t);
    let minus_2x_t = builder.mul_const(F::from_canonical_u64(2), minus_x_t);
    let x2_t = builder.exp_u64(x_t, 2);
    let one_t = builder.one();
    let zero_t = builder.zero();
    let poly_t = builder.add_many(&[x2_t, minus_2x_t, one_t]);
    builder.connect(poly_t, zero_t); // x^2 - 2x + 1 = 0

    let data = builder.build::<C>();
    let mut pw = PartialWitness::<F>::new();
    pw.set_target(x_t, GoldilocksField(1)); // set x = 1

    let proof = data.prove(pw)?;
    data.verify(proof)?;

    Ok(())
}
