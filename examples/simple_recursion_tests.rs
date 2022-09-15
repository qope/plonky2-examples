use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::iop::witness::{PartialWitness,Witness};
use plonky2::plonk::circuit_data::{CircuitConfig, VerifierCircuitTarget};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::proof::{ProofWithPublicInputs, ProofWithPublicInputsTarget};
use plonky2_examples::wrapper::Wrapper;

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;

fn make_circuit_and_proof(config:CircuitConfig,exp_number:u64,solution:u64)
->(CircuitData<F, C, 2>,ProofWithPublicInputs<F, C, 2>){

    // First proof "x = 1 satisfies x^(exp_number) - x = 0"
    let mut builder = CircuitBuilder::<F, 2>::new(config.clone());
    let x_t = builder.add_virtual_target();
    let x2_t = builder.exp_u64(x_t, exp_number);
    let lhs_t = builder.sub(x2_t, x_t);
    let zero_t = builder.zero();
    builder.connect(lhs_t, zero_t);
    let data = builder.build::<C>();

    let mut pw = PartialWitness::<F>::new();
    pw.set_target(x_t, GoldilocksField(solution));
    let proof = data.prove(pw).unwrap();
    match  data.verify(proof.clone()) {
        Ok(()) => println!("First proof: Ok!"),
        Err(x) => println!("{}", x)
    }
    
    (data, proof)
}

fn make_recursive_circuit(config: CircuitConfig, data: &CircuitData<F, C, 2>, proof: ProofWithPublicInputs<F,C,2>)
    ->(CircuitData<F,C,2>,Wrapper<ProofWithPublicInputsTarget<2>>,Wrapper<VerifierCircuitTarget>){
    // Recursive proof
    let mut builder_recursive = CircuitBuilder::<F, 2>::new(config);
    let proof_t = builder_recursive.add_virtual_proof_with_pis(&data.common);
    let wrapped_proof_t = Wrapper(proof_t);
    let vd_target = VerifierCircuitTarget {
        constants_sigmas_cap: builder_recursive.add_virtual_cap(data.common.config.fri_config.cap_height),
    };
    let wrapped_vd_target = Wrapper(vd_target);
    builder_recursive.verify_proof(wrapped_proof_t.clone().0, &wrapped_vd_target.clone().0, &data.common);
    let data_recursive = builder_recursive.build::<C>();

    (data_recursive, wrapped_proof_t, wrapped_vd_target)
}

fn make_recursive_proof(data_recursive: &CircuitData<F,C,2>,proof: ProofWithPublicInputs<F,C,2>,
    data:CircuitData<F,C,2>,wrapped_proof_t:Wrapper<ProofWithPublicInputsTarget<2>>,wrapped_vd_target:Wrapper<VerifierCircuitTarget>){
    let mut pw_recursive = PartialWitness::<F>::new();
    pw_recursive.set_proof_with_pis_target(&wrapped_proof_t.0, &proof);
    pw_recursive.set_cap_target(&wrapped_vd_target.0.constants_sigmas_cap, &data.verifier_only.constants_sigmas_cap);
    let proof_recursive = data_recursive.prove(pw_recursive).unwrap();
    match  data_recursive.verify(proof_recursive) {
        Ok(()) => println!("Recursive proof: Ok!"),
        Err(x) => println!("{}", x)
    }
}

fn main() {
    
    let config = CircuitConfig::standard_recursion_config();

    // Here we input the proof where x=1 for x^2-x=0. 
    let (data,proof) = make_circuit_and_proof(config.clone(),2,1);
    let c_data = make_recursive_circuit(config.clone(),&data,proof.clone());
    make_recursive_proof(&c_data.0,proof,data,c_data.1.clone(),c_data.2.clone());

    // Here we input the proof where x=0 for x^2-x=0. 
    let (another_data,another_proof) = make_circuit_and_proof(config.clone(),2,0);
    make_recursive_proof(&c_data.0,another_proof,another_data,c_data.1.clone(),c_data.2.clone());

}

#[test]
#[should_panic]
fn test_another_circuits_proof() {
    // This test should fail since we input the proof of another circuit(x^3-x=0) 
    // to check if the recursive verifier is restricted to a certain circuit(x-2-x=0)

    let config = CircuitConfig::standard_recursion_config();

    let (data,proof) = make_circuit_and_proof(config.clone(),2,1);
    let c_data = make_recursive_circuit(config.clone(),&data,proof.clone());
    make_recursive_proof(&c_data.0,proof,data,c_data.1.clone(),c_data.2.clone());

    let (another_circuit_data,another_circuit_proof) = make_circuit_and_proof(config.clone(),3,1);
    make_recursive_proof(&c_data.0,another_circuit_proof,another_circuit_data,c_data.1,c_data.2);

}