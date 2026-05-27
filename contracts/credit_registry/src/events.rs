use soroban_sdk::{Env, Address, BytesN, Symbol, String};

/// Event topics use Symbol::new for consistent formatting across all contracts.
/// This ensures off-chain indexers can reliably parse event schemas.

pub fn credit_submitted(env: &Env, issuer: Address, project_id: String, tonnes: i128) {
    let topics = (Symbol::new(env, "credit_submitted"), issuer);
    env.events().publish(topics, (project_id, tonnes));
}

pub fn credit_minted(env: &Env, verifier: Address, id: BytesN<32>) {
    let topics = (Symbol::new(env, "credit_minted"), verifier);
    env.events().publish(topics, id);
}

pub fn credit_flagged(env: &Env, id: BytesN<32>, reason: String) {
    let topics = (Symbol::new(env, "credit_flagged"),);
    env.events().publish(topics, (id, reason));
}

pub fn verifier_added(env: &Env, admin: Address, verifier: Address) {
    let topics = (Symbol::new(env, "verifier_added"), admin);
    env.events().publish(topics, verifier);
}

pub fn verifier_removed(env: &Env, admin: Address, verifier: Address) {
    let topics = (Symbol::new(env, "verifier_removed"), admin);
    env.events().publish(topics, verifier);
}

pub fn contract_paused(env: &Env, admin: Address) {
    env.events().publish((Symbol::new(env, "contract_paused"),), admin);
}

pub fn contract_unpaused(env: &Env, admin: Address) {
    env.events().publish((Symbol::new(env, "contract_unpaused"),), admin);
}
