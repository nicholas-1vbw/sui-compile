use risc0_zkvm::guest::env;

fn main() {
    // read the input
    let input: lib::Proof = env::read();

    let pass = input.verify().is_ok();

    env::commit(&pass);
}
