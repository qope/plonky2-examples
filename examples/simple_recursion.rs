use anyhow::Result;
use plonky2::{
    field::{extension::Extendable, goldilocks_field::GoldilocksField, types::Field},
    hash::hash_types::RichField,
    iop::witness::{PartialWitness, WitnessWrite},
    plonk::{
        circuit_builder::CircuitBuilder,
        circuit_data::{CircuitConfig, CommonCircuitData, VerifierCircuitTarget},
        config::{GenericConfig, PoseidonGoldilocksConfig},
        proof::ProofWithPublicInputs,
    },
};

fn make_first_proof<F, C, const D: usize>(
) -> Result<(CommonCircuitData<F, D>, ProofWithPublicInputs<F, C, D>)>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F>,
{
    let config = CircuitConfig::standard_recursion_config();

    // First proof that "x satisfies x^2 = 4"
    let mut builder = CircuitBuilder::<F, D>::new(config.clone());
    let x_t = builder.add_virtual_target();
    builder.register_public_input(x_t); // Register x as public input
    let x2_t = builder.exp_u64(x_t, 2);
    let four_t = builder.constant(F::from_canonical_u64(4));
    builder.connect(x2_t, four_t);

    let data = builder.build::<C>();
    let mut pw = PartialWitness::<F>::new();
    pw.set_target(x_t, F::from_canonical_u64(2)); // x = 2
    let proof = data.prove(pw)?;
    data.verify(proof.clone())?;
    Ok((data.common, proof))
}

fn main() -> Result<()> {
    const D: usize = 2;
    type F = GoldilocksField;
    type C = PoseidonGoldilocksConfig;
    println!("First proof passed!");

    // Recursive proof
    let mut builder_recursive = CircuitBuilder::<F, 2>::new(config);
    let mut pw_recursive = PartialWitness::<F>::new();
    let proof_t = builder_recursive.add_virtual_proof_with_pis(&data.common);
    builder_recursive.register_public_inputs(&proof_t.public_inputs);
    pw_recursive.set_proof_with_pis_target(&proof_t, &proof);
    let vd_target = VerifierCircuitTarget {
        constants_sigmas_cap: builder_recursive
            .add_virtual_cap(data.common.config.fri_config.cap_height),
    };
    pw_recursive.set_cap_target(
        &vd_target.constants_sigmas_cap,
        &data.verifier_only.constants_sigmas_cap,
    );
    builder_recursive.verify_proof(proof_t, &vd_target, &data.common);
    let data_recursive = builder_recursive.build::<C>();
    let proof_recursive = data_recursive.prove(pw_recursive).unwrap();
    match data_recursive.verify(proof_recursive.clone()) {
        Ok(()) => println!("Recursive proof: Ok!"),
        Err(x) => println!("{}", x),
    }
    println!("public inputs :{:?}", proof_recursive.public_inputs);

    Ok(())
}
