use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::iop::witness::{PartialWitness,Witness};
use plonky2::plonk::circuit_data::{CircuitConfig, VerifierCircuitTarget};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::config::PoseidonGoldilocksConfig;

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;

fn main() {
    let config = CircuitConfig::standard_recursion_config();

    // First proof
    let mut builder = CircuitBuilder::<F, 2>::new(config.clone());
    let x_t = builder.add_virtual_target();
    let x2_t = builder.exp_u64(x_t, 2);
    let lhs_t = builder.sub(x2_t, x_t); // lhs = x^2 - x
    let zero_t = builder.zero();
    builder.connect(lhs_t, zero_t); // x^2 - x = 0
    let data = builder.build::<C>();
    let mut pw = PartialWitness::<F>::new();
    pw.set_target(x_t, GoldilocksField(1)); // x = 1
    let proof = data.prove(pw).unwrap();
    match  data.verify(proof.clone()) {
        Ok(()) => println!("First proof: Ok!"),
        Err(x) => println!("{}", x)
    }
    
    // Recursive proof
    let mut builder_recursive = CircuitBuilder::<F, 2>::new(config);
    let mut pw_recursive = PartialWitness::<F>::new();
    let proof_t = builder_recursive.add_virtual_proof_with_pis(&data.common);
    pw_recursive.set_proof_with_pis_target(&proof_t, &proof);
    let vd_target = VerifierCircuitTarget {
        constants_sigmas_cap: builder_recursive.add_virtual_cap(data.common.config.fri_config.cap_height),
    };
    pw_recursive.set_cap_target(&vd_target.constants_sigmas_cap, &data.verifier_only.constants_sigmas_cap);
    builder_recursive.verify_proof(proof_t, &vd_target, &data.common);
    let data_recursive = builder_recursive.build::<C>();
    let proof_recursive = data_recursive.prove(pw_recursive).unwrap();
    match  data_recursive.verify(proof_recursive) {
        Ok(()) => println!("Recursive proof: Ok!"),
        Err(x) => println!("{}", x)
    }
}