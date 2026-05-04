#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, BytesN};

#[contract]
pub struct Marketplace;

#[contractimpl]
impl Marketplace {
    pub fn create_offer(_env: Env, _seller: Address, _credit_id: BytesN<32>, _price_xlm: i128) {
        // TODO: Implementation
    }
}
