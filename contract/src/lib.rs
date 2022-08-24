/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::{env, log, near_bindgen, AccountId};

mod design;
mod generate;

use design::Design;
use generate::generate;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    designs: UnorderedMap<AccountId, Design>,
    owners: UnorderedSet<AccountId>,
}

// Define the default, which automatically initializes the co&ntract
impl Default for Contract {
    fn default() -> Self {
        Self {
            designs: UnorderedMap::new(b"d"),
            owners: UnorderedSet::new(b"o"),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    pub fn claim_my_design(&mut self, seed: i32) {
        assert!(seed >= 0, "Seed needs to be valid.");
        assert!(
            !self
                .designs
                .keys()
                .any(|owner| owner == env::signer_account_id()),
            "You can only own one design."
        );

        let instructions = generate(seed);

        let design = Design::new(&instructions);

        log!(
            "\n\n\t> ART / Seed: {seed} \n\n\t {}\n",
            instructions.replace('\n', "\n\t")
        );

        log!("\n\n\tClaimed Art");

        self.designs.insert(&env::signer_account_id(), &design);
        self.owners.insert(&env::signer_account_id());
    }

    pub fn view_my_design(&self) {
        let design = self.designs.get(&env::signer_account_id()).unwrap();

        log!(
            "\n\n\t> Your Art \n\n\t {}",
            design.instructions.replace('\n', "\n\t")
        );
    }

    pub fn burn_my_design(&mut self) {
        assert!(
            self.owners.contains(&env::signer_account_id()),
            "No design to burn here."
        );

        self.designs.remove(&env::signer_account_id());
        self.designs.remove(&env::signer_account_id());

        log!("\n\n\t> Design burned \n\n\t");
    }

    pub fn design(maybe_seed: Option<i32>) {
        let instructions = generate(maybe_seed.unwrap_or(0));

        log!(
            "\n\n\t> ART \n\n\t {}\n",
            instructions.replace('\n', "\n\t")
        );
    }

    pub fn view_designs(&self) {
        let owners = self.owners.to_vec();

        for owner in owners.iter() {
            let design = self.designs.get(owner).unwrap();

            log!(
                "\n\n\t> Owner : {owner} \n\n\t {}\n",
                design.instructions.replace('\n', "\n\t"),
            )
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let contract = Contract::default();

        contract.view_designs();
    }
}
