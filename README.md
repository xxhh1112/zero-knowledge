# Zero-Knowledge Implementations: The future of Proofs


![Logo](https://github.com/UniversalDot/documents/blob/master/logo/universaldot-logo/rsz_jpg-02.jpg)

## Description 

Zero-knowledge proofs are cryptographic protocols that enable one party, called the prover, to convince another party, called the verifier, that a certain statement is true, without revealing any additional information beyond the truth of the statement itself. These proofs are designed to provide strong evidence of knowledge or possession without disclosing sensitive data.

## Process 

- Prover Setup:

The prover generates a secret value, which could be a private key or any other sensitive information.
The prover computes the corresponding public value, usually derived from the secret value using mathematical operations on elliptic curves.

- Commitment:

The prover generates a random value, called a commitment, which serves as a commitment to some underlying information.
The commitment is calculated by multiplying the random value with a base point on the elliptic curve.

- Challenge Generation:

The verifier generates a random challenge value, which will be used to validate the proof.
This challenge is typically generated using a random number generator.

- Response Calculation:

The prover computes a response value based on the challenge and the secret value.
This calculation involves combining the secret value and the challenge in a way that preserves the relationship between them without revealing the secret itself.

- Proof Generation:

The verifier generates its own challenge and response values.
The prover then generates a proof commitment and proof response based on the verifier's challenge and response.
The proof commitment is computed by multiplying the verifier's challenge with a base point on the elliptic curve.
The proof response is calculated by adding the verifier's response to the product of the verifier's challenge and the prover's secret value.

- Proof Verification:

The verifier verifies the validity of the proof by comparing the left-hand side and the right-hand side of an equation.
The left-hand side is typically the product of the verifier's response and a base point on the elliptic curve.
The right-hand side is computed by combining the prover's public value, the verifier's challenge, and the proof commitment.
If the two sides are equal, the proof is considered valid, indicating that the prover possesses the knowledge or information claimed, without revealing the underlying secret.