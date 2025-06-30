use risc0_zkvm::guest::env;

/// Reads a `Proof` from the environment, verifies it, and commits the verification result.
///
/// The function reads a `lib::Proof` object, checks its validity using the `verify` method, and commits a boolean indicating whether the verification succeeded.
///
/// # Examples
///
/// ```
/// // In a zkVM guest context:
/// main(); // Commits true if the provided proof is valid, false otherwise.
/// ```
fn main() {
    // read the input
    let input: lib::Proof = env::read();

    let pass = input.verify().is_ok();

    env::commit(&pass);
}
