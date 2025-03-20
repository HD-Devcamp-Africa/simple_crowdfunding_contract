#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Env, String};

#[test]
fn test() {
    let env: Env = Env::default();
    let contract_id: soroban_sdk::Address = env.register(CrowdFundContract, ()); // registers the contract you want to test.

    let client: CrowdFundContractClient<'_> = CrowdFundContractClient::new(&env, &contract_id);

    
}
