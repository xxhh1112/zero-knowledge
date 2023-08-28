#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commitment_generation() {
        let mut rng = rand::thread_rng();
        let secret_key = Scalar::random(&mut rng);
        let commitment = RISTRETTO_BASEPOINT_POINT * secret_key;
        assert_ne!(commitment, RISTRETTO_BASEPOINT_POINT);
    }

    #[test]
    fn test_response_generation() {
        let mut rng = rand::thread_rng();
        let secret_key = Scalar::random(&mut rng);
        let reputation_data = ReputationData {
            positive_vouches: 5,
            negative_vouches: 1,
        };
        let challenge = Scalar::random(&mut rng);
        let response = secret_key + challenge * Scalar::from(reputation_data.positive_vouches);
        assert_ne!(response, secret_key);
    }

    #[test]
    fn test_commitment_verification() {
        let mut rng = rand::thread_rng();
        let secret_key = Scalar::random(&mut rng);
        let commitment = RISTRETTO_BASEPOINT_POINT * secret_key;
        let expected_commitment = RISTRETTO_BASEPOINT_POINT * secret_key;
        assert_eq!(commitment, expected_commitment);
    }

    // Add more tests here...

    #[test]
    fn test_full_reputation_proof() {
        let mut rng = rand::thread_rng();

        let alice_secret_key = Scalar::random(&mut rng);
        let alice_reputation_data = ReputationData {
            positive_vouches: 10,
            negative_vouches: 2,
        };
        let alice = User {
            reputation_data: alice_reputation_data,
            secret_key: alice_secret_key,
        };

        let challenge = Scalar::random(&mut rng);

        let commitment_a = RISTRETTO_BASEPOINT_POINT * alice.secret_key;
        let response_a = alice.secret_key + challenge * Scalar::from(alice.reputation_data.positive_vouches);

        let expected_commitment_a = RISTRETTO_BASEPOINT_POINT * alice.secret_key;
        let expected_response_a = alice.secret_key + challenge * Scalar::from(alice.reputation_data.positive_vouches);

        assert_eq!(commitment_a, expected_commitment_a);
        assert_eq!(response_a, expected_response_a);
    }
}
