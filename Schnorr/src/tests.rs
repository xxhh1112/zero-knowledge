#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        let prime = BigUint::from(17u8);
        assert!(is_prime(&prime));

        let composite = BigUint::from(15u8);
        assert!(!is_prime(&composite));
    }

    #[test]
    fn test_generate_generator() {
        let prime = BigUint::from(23u8);
        let generator = generate_generator(&prime);
        assert_eq!(generator, BigUint::from(5u8));
    }

    #[test]
    fn test_verify() {
        let prime = BigUint::from(23u8);
        let generator = BigUint::from(5u8);
        let public_key = BigUint::from(8u8);
        let commitment = BigUint::from(19u8);
        let challenge = BigUint::from(7u8);
        let response = BigUint::from(14u8);

        assert!(verify(&prime, &generator, &public_key, &commitment, &challenge, &response));

        // Test with incorrect response
        let incorrect_response = BigUint::from(15u8);
        assert!(!verify(&prime, &generator, &public_key, &commitment, &challenge, &incorrect_response));
    }
}
