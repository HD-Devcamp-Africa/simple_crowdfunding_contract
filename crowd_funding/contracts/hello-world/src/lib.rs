#![no_std]

// Class work 

// Simple Crowdfunding Contract


// Features: Campaign creation, contribution tracking, deadline management

// Learning outcomes: Basic storage, time-based conditions, payment handling



use core::iter::Map;

use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, String, Symbol, Vec};

#[contract]
pub struct CrowdFundContract;

pub struct Contributor {
    pub title: String,
    pub balance: i32,
    pub time_of_contribution: u32
}

pub struct Contributors {
    contributor: Vec<Contributor>
}

#[contractimpl]
impl CrowdFundContract {
   
    const CONTRIBUTORS_KEY: Symbol = symbol_short!("CONTRIBS");

    // initialize contributor's account.
    pub fn create_contributor(env: Env, title: String){

        // let mut contributors: Map<String, Contributor> = env.storage()
        //     .persistent()
        //     .get(&"contributors".into())
        //     .unwrap_or_else(|| Map::new(&env));
        todo!()

    }

    pub fn deposit(env: &Self , amount: i32, recipient: Contributor) -> Self {
        todo!()
    }






}

mod test;
