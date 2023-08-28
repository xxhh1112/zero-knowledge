extern crate curve25519_dalek;
extern crate rand;

use curve25519_dalek::constants::RISTRETTO_BASEPOINT_TABLE;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use rand::thread_rng;
use rand::Rng;

fn main() {
    // Prover's side
    let secret = Scalar::random(&mut thread_rng());
    let public = &secret * &RISTRETTO_BASEPOINT_TABLE;

    // Random value for the commitment
    let random_value = Scalar::random(&mut thread_rng());
    let commitment = &random_value * &RISTRETTO_BASEPOINT_TABLE;

    // Challenge generation
    let mut rng = thread_rng();
    let challenge: Scalar = rng.gen();

    // Response calculation
    let response = &secret + &(challenge * random_value);

    // Verifier's side
    let verifier_challenge = Scalar::random(&mut rng);
    let verifier_response = Scalar::random(&mut rng);

    // Prover's proof generation
    let proof_commitment = &verifier_challenge * &RISTRETTO_BASEPOINT_TABLE;
    let proof_response = &verifier_response + &(verifier_challenge * &secret);

    // Verifier's verification
    let proof_lhs: RistrettoPoint = &verifier_response * &RISTRETTO_BASEPOINT_TABLE;
    let proof_rhs: RistrettoPoint = &verifier_challenge * &public + &proof_commitment;

    // Verify if the proof is valid
    let valid_proof = proof_lhs == proof_rhs;

    println!("Proof is valid: {}", valid_proof);
}
