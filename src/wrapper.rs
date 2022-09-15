use plonky2::{plonk::{proof::{ProofWithPublicInputsTarget, ProofTarget}, circuit_data::VerifierCircuitTarget}, fri::proof::FriProofTarget, gadgets::polynomial::PolynomialCoeffsExtTarget};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Wrapper<T>(pub T);


impl<const D: usize> Clone for Wrapper<ProofWithPublicInputsTarget<D>> {
    fn clone(&self) -> Self {
        Wrapper(ProofWithPublicInputsTarget {
            proof: ProofTarget {
                wires_cap: self.0.proof.wires_cap.clone(),
                plonk_zs_partial_products_cap: self.0.proof.plonk_zs_partial_products_cap.clone(),
                quotient_polys_cap: self.0.proof.quotient_polys_cap.clone(),
                openings: self.0.proof.openings.clone(),
                opening_proof: FriProofTarget {
                    commit_phase_merkle_caps: self.0.proof.opening_proof.commit_phase_merkle_caps.clone(),
                    query_round_proofs: self.0.proof.opening_proof.query_round_proofs.clone(),
                    final_poly: PolynomialCoeffsExtTarget(self.0.proof.opening_proof.final_poly.0.clone()),
                    pow_witness: self.0.proof.opening_proof.pow_witness.clone(),
                },
            },
            public_inputs: self.0.public_inputs.clone(),
        })
    }
}

impl Clone for Wrapper<VerifierCircuitTarget> {
    fn clone(&self) -> Self {
        Wrapper(VerifierCircuitTarget {
            constants_sigmas_cap: self.0.constants_sigmas_cap.clone()
        })
    }
}