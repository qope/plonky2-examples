use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::hash::poseidon::PoseidonHash;
use plonky2::iop::witness::PartialWitness;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::config::{PoseidonGoldilocksConfig, Hasher};

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;

fn main() {
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, 2>::new(config);
    let x = GoldilocksField(1); // x = 1
    let x_t = builder.constant(x.clone());
    let hash_x = PoseidonHash::hash_no_pad(&[x.clone()]); // hash_x = hash(x)
    println!("x = {x}, hash(x) = {hash_x:?}");

    let a_t = builder.add_virtual_target(); // add a variable
    let hash_a_t = builder.add_virtual_hash(); // add a hash variable

    builder.connect(x_t, a_t); // x = a
    for i in 0..4 {
        let i_th_hash_x_t = builder.constant(hash_x.elements[i]);
        builder.connect(hash_a_t.elements[i], i_th_hash_x_t); // hash(a)[i] = hash(x)[i]
    }

    let data = builder.build::<C>();
    let pw = PartialWitness::<F>::new();
    let proof = data.prove(pw).unwrap();
    match  data.verify(proof) {
        Ok(()) => println!("Ok!"),
        Err(x) => println!("{}", x)
    }
}