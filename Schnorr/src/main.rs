extern crate rand;
extern crate num_bigint;

use rand::Rng;
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};
use crate::num_bigint::ToBigUint;

fn main() {
    // Generate a prime number as the base for the proof
    let mut rng = rand::thread_rng();
    let prime: BigUint = generate_prime_number();

    // Generate a random number as the secret
    let secret: BigUint = rng.gen::<u64>().to_biguint().unwrap();

    // Generate the public key
    let generator: BigUint = generate_generator(&prime);
    let public_key: BigUint = generator.modpow(&secret, &prime);

    println!("Prime: {}", prime);
    println!("Generator: {}", generator);
    println!("Secret: {}", secret);
    println!("Public Key: {}", public_key);

    // Prover's side
    let commitment: BigUint = commit(&prime, &generator, &secret);
    let challenge: BigUint = generate_challenge();
    let response: BigUint = respond(&secret, &challenge, &prime);

    // Verifier's side
    let verified: bool = verify(&prime, &generator, &public_key, &commitment, &challenge, &response);

    if verified {
        println!("Proof verified successfully!");
    } else {
        println!("Proof verification failed!");
    }
}

fn generate_prime_number() -> BigUint {
    // Generate a random number and check if it's prime
    let mut rng = rand::thread_rng();
    let mut prime: BigUint = rng.gen::<u64>().to_biguint().unwrap();

    while !is_prime(&prime) {
        prime += BigUint::one();
    }

    prime
}

fn is_prime(n: &BigUint) -> bool {
    // Check if a number is prime using a simple primality test
    let two: BigUint = BigUint::from(2u8);
    let sqrt_n: BigUint = n.sqrt();
    let mut divisor: BigUint = BigUint::from(2u8);

    while &divisor <= &sqrt_n {
        if n % &divisor == BigUint::zero() {
            return false;
        }

        divisor += BigUint::one();
    }

    true
}

fn generate_generator(prime: &BigUint) -> BigUint {
    // Generate a generator for the given prime number
    let mut generator: BigUint = BigUint::from(2u8);

    while generator.modpow(prime, prime) != BigUint::one() {
        generator += BigUint::one();
    }

    generator
}

fn commit(prime: &BigUint, generator: &BigUint, secret: &BigUint) -> BigUint {
    // Compute the commitment value for the given secret
    let mut rng = rand::thread_rng();
    let r: BigUint = rng.gen::<u64>().to_biguint().unwrap();

    let commitment: BigUint = generator.modpow(&r, prime);
    let secret_power: BigUint = secret.modpow(prime, prime);
    let commitment_value: BigUint = (commitment * secret_power) % prime;

    commitment_value
}

fn generate_challenge() -> BigUint {
    // Generate a random challenge value
    let mut rng = rand::thread_rng();
    rng.gen_biguint(256);
}

fn respond(secret: &BigUint, challenge: &BigUint, prime: &BigUint) -> BigUint {
    // Compute the response value for the given secret and challenge
    let response: BigUint = (secret + (challenge * secret)) % prime;

    response
}

fn verify(
    prime: &BigUint,
    generator: &BigUint,
    public_key: &BigUint,
    commitment: &BigUint,
    challenge: &BigUint,
    response: &BigUint,
) -> bool {
    // Verify the zero-knowledge proof
    let commitment_check: BigUint = generator.modpow(response, prime)
        * public_key.modpow(challenge, prime)
        % prime;

    commitment_check == *commitment
}
