use near_sdk::{AccountId, near_bindgen, borsh::{self, BorshSerialize, BorshDeserialize}, env};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Design {
    owner: AccountId,
    pub instructions: String
}

impl Design {
    pub fn new(instructions: &str) -> Self {
        Design {
            owner: env::signer_account_id(),
            instructions: instructions.to_string()
        }
    }
}
