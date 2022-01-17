use near_sdk::{
    env,
    borsh::{
        self,
        BorshDeserialize,
        BorshSerialize,
    },
};

pub type AccountId = String;


#[derive(BorshDeserialize, BorshSerialize)]
pub struct Design{
    pub owner: String,
    pub instructions: String,
}

impl Design{
    pub fn new(instructions: String) -> Self {
        let owner = env::signer_account_id();
        Design{
            owner,
            instructions,
        }
    }
}
