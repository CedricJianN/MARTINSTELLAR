#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Symbol,
};

#[contract]
pub struct AgriGrantWallet;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Approved(Address),
    Claimed(Address),
}

#[contractimpl]
impl AgriGrantWallet {

    // Initialize admin
    pub fn init(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    // Approve farmer eligibility
    pub fn approve_farmer(env: Env, farmer: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        env.storage().instance().set(&DataKey::Approved(farmer), &true);
    }

    // Farmer claims subsidy
    pub fn claim(env: Env, farmer: Address) {
        farmer.require_auth();

        let approved: bool = env
            .storage()
            .instance()
            .get(&DataKey::Approved(farmer.clone()))
            .unwrap_or(false);

        if !approved {
            panic!("Farmer not approved");
        }

        let already_claimed: bool = env
            .storage()
            .instance()
            .get(&DataKey::Claimed(farmer.clone()))
            .unwrap_or(false);

        if already_claimed {
            panic!("Already claimed");
        }

        env.storage().instance().set(&DataKey::Claimed(farmer), &true);
    }

    // Check claim status
    pub fn has_claimed(env: Env, farmer: Address) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::Claimed(farmer))
            .unwrap_or(false)
    }
}
