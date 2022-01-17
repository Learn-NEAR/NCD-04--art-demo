mod generate;
mod models;


#[allow(unused_imports)]
use near_sdk::{
    env,
    near_bindgen,
    borsh::{
        self, 
        BorshDeserialize, 
        BorshSerialize,
    },
    collections::{
        LookupMap,
        LookupSet,
        UnorderedSet,
    },
};

use crate::{
    generate::generate,
    models::{
        AccountId,
        Design,
    },
};

/// This function is not compiled when testing.
/// In other words, this function doesn't exist in tests
#[cfg(not(test))]
pub fn log(message: &str) {
    env::log(message.as_bytes());
}

/// This function is only compiled during tests.
/// In other words, this function doesn't exist outside of tests.
#[cfg(test)]
pub fn log(message: &str) {
    println!("{}", message);
}


near_sdk::setup_alloc!();


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // Contract State is here
    designs: LookupMap<AccountId, Design>,
    owners: UnorderedSet<AccountId>,
}

impl Default for Contract{
    fn default() -> Self{
        let designs: LookupMap<AccountId, Design> = LookupMap::new("d".as_bytes());
        let owners: UnorderedSet<AccountId> = UnorderedSet::new("o".as_bytes());
        
        Contract{
            designs,
            owners,
        }
    }
}

#[near_bindgen]
impl Contract{
    // allow(non_snake_case) Tells the compiler not to complain about the function name
    #[allow(non_snake_case)]
    pub fn claimMyDesign(&mut self, seed: i32) {
        let sender = env::signer_account_id();
        assert!(seed >= 0, "Seed needs to be valid.");
        assert!(!&self.designs.contains_key(&sender), "You can only own one design.");

        let instructions = generate(seed);

        let design = Design::new(instructions.clone());

        log(&format!("\n\n\t> ART / Seed: {} \n\n\t{}\n", seed, instructions.replace("\n", "\n\t")));

        log("\n\n\tClaimed Art");

        self.designs.insert(&sender, &design);

        self.owners.insert(&sender);
    }   

    #[allow(non_snake_case)]
    pub fn viewMyDesign(&self) {
        let sender = env::signer_account_id();
        let design = self.designs.get(&sender);

        let response: String = match design{
            Some(value) => format!("\n\n\t> Your Art \n\n\t{}\n", value.instructions.replace("\n", "\n\t")),
            None => format!("Failed to find any design for {}.", sender),
        };
        
        log(&response);
    }

    #[allow(non_snake_case)]
    pub fn burnMyDesign(&mut self) {
        let sender: String = env::signer_account_id();

        assert!(self.designs.contains_key(&sender), "No design to burn here.");

        self.designs.remove(&sender).unwrap();
        self.owners.remove(&sender);

        log("\n\n\t> Design burned \n\n\t");
    }

    pub fn design(&self, seed: Option<i32>) {
        let seed = seed.unwrap_or(0);
        let instructions = generate(seed);

        let response = &format!("\n\n\t> ART \n\n\t{}\n", instructions.replace("\n", "\n\t"));

        log(response);
    }

    #[allow(non_snake_case)]
    pub fn viewDesigns(&self) {
        // let ownersValues = self.owners.values();

        for value in self.owners.iter(){
            let design = self.designs.get(&value).unwrap();

            let response: &String = &format!("\n\n\t> Owner : {} \n\n\t{}\n", design.owner, design.instructions.replace("\n", "\n\t"));
            log(response);
        }

    }
}

// burn/delete user might want to do it to create new one

#[cfg(test)]
mod tests{
    use super::*;

    use near_sdk::{testing_env, VMContext, MockedBlockchain};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {

        // Had to change block_index size because it is used as seed for random values.
        // If block_index is 0, the input for seed on random is 0. So it cause an error.
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 100000,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    fn set_default_call_context(){
        let context = get_context(Vec::new(), false);

        testing_env!(context);
    }

    #[test]
    pub fn should_design(){
        set_default_call_context();

        let contract: Contract = Contract::default();
        
        // This is the same as giving no args
        contract.design(None);
    }

    /// With this test we can see that a value higher than 144_500_000 will cause an overflow error
    /// Because of that, random function will generate a value between 0 and 1. Then multiply that value with 144_500_000 and convert it into integer.
    #[test]
    pub fn brute_test(){
        set_default_call_context();

        let contract: Contract = Contract::default();


        for i in 0..1000{
            // set_default_call_context();
            contract.design(Some(i*500000));
        }

    }
}