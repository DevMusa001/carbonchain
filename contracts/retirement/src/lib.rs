pub mod types;

use crate::types::RetirementRecord;

#[contract]
pub struct Retirement;

#[contractimpl]
impl Retirement {
    pub fn retire(env: Env, buyer: Address, credit_id: BytesN<32>, tonnes: i128, reason: String) -> BytesN<32> {
        buyer.require_auth();

        // TODO: Invoke CCR token contract burn()
        
        let retirement_id = env.crypto().sha256(&reason.to_xdr(&env));
        let record = RetirementRecord {
            credit_id: credit_id.clone(),
            buyer: buyer.clone(),
            tonnes_retired: tonnes,
            reason: reason.clone(),
            retired_at: env.ledger().timestamp(),
        };

        env.storage().persistent().set(&crate::types::DataKey::Retirement(retirement_id.clone()), &record);

        retirement_id
    }
}
