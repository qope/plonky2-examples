use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::iop::witness::{PartialWitness, Witness};
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::config::PoseidonGoldilocksConfig;

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;

fn main() {
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, 2>::new(config);
    let x_t = builder.add_virtual_target();
    let x2_t = builder.exp_u64(x_t, 2);
    let lhs_t = builder.sub(x2_t, x_t); // lhs = x^2 - x
    let zero_t = builder.zero();
    builder.connect(lhs_t, zero_t); // x^2 - x = 0
    let data = builder.build::<C>();
    let mut pw = PartialWitness::<F>::new();
    pw.set_target(x_t, GoldilocksField(1)); // x = 1
    let proof = data.prove(pw).unwrap();
    match  data.verify(proof) {
        Ok(()) => println!("Ok!"),
        Err(x) => println!("{}", x)
    }
}