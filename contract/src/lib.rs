/*
 * This is an example of a Rust smart contract with two simple, symmetric functions:
 *
 * 1. set_greeting: accepts a greeting, such as "howdy", and records it for the user (account_id)
 *    who sent the request
 * 2. get_greeting: accepts an account_id and returns the greeting saved for it, defaulting to
 *    "Hello"
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://github.com/near/near-sdk-rs
 *
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, setup_alloc, AccountId, Promise};

setup_alloc!();

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct VenmoClone {
    memo: LookupMap<String, Vec<String>>,
}

impl Default for VenmoClone {
    fn default() -> Self {
        Self {
            memo: LookupMap::new(b"memo".to_vec()),
        }
    }
}

#[near_bindgen]
impl VenmoClone {
    //change method incurs a fee by changing the state of the blockchain

    pub fn add_memo(&mut self, memo_text: String, price: String) {
        //env comes from the near sdk at the top
        //signer_account_id() gives the id for the person that last signed the smart contract
        //aka since the person connected wallet, its the name of the wallet thats currently signed in
        let account_id = env::signer_account_id();
        let contains_user = self.memo.contains_key(&account_id);

        if contains_user {
            //the .get function returns either "Some" or "None". so if its found, it returns Some, if not found, then None.  So below its accounting for each return
            let mut temp_list = match self.memo.get(&account_id) {
                Some(x) => x,
                None => vec![],
            };

            temp_list.push(memo_text + " || " + &price + "NEAR");
            self.memo.insert(&account_id, &temp_list);
        } else {
            let fresh_vec = vec![memo_text + " || " + &price + "NEAR"];
            self.memo.insert(&account_id, &fresh_vec);
        }
    }

    pub fn transfer_money(&mut self, account_id: AccountId, amount: f64) {
        //The line below is one way of transferring money with rust in NEAR
        Promise::new(account_id).transfer(amount as u128);
    }

    //View Methods
    //dont make self mutable in a view method
    pub fn get_memos(self, user: String) -> Vec<String> {
        match self.memo.get(&user) {
            Some(x) => x,   //contains all your memos
            None => vec![], //otherwise returns empty memo
        } //Since theres no semicolon here, this value is returned
    }
}
