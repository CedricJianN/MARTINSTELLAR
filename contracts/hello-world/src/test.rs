#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::{AgriGrantWallet, AgriGrantWalletClient};

mod tests {

    use super::*;

    #[test]
    fn test_happy_path_claim() {
        let env = Env::default();

        let contract_id = env.register(AgriGrantWallet, ());
        let client = AgriGrantWalletClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let farmer = Address::generate(&env);

        client.init(&admin);
        client.approve_farmer(&farmer);
        client.claim(&farmer);

        assert!(client.has_claimed(&farmer));
    }

    #[test]
    #[should_panic(expected = "Farmer not approved")]
    fn test_unapproved_farmer() {
        let env = Env::default();

        let contract_id = env.register(AgriGrantWallet, ());
        let client = AgriGrantWalletClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let farmer = Address::generate(&env);

        client.init(&admin);
        client.claim(&farmer);
    }

    #[test]
    fn test_storage_state() {
        let env = Env::default();

        let contract_id = env.register(AgriGrantWallet, ());
        let client = AgriGrantWalletClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let farmer = Address::generate(&env);

        client.init(&admin);
        client.approve_farmer(&farmer);
        client.claim(&farmer);

        assert_eq!(client.has_claimed(&farmer), true);
    }

    #[test]
    #[should_panic(expected = "Already claimed")]
    fn test_duplicate_claim() {
        let env = Env::default();

        let contract_id = env.register(AgriGrantWallet, ());
        let client = AgriGrantWalletClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let farmer = Address::generate(&env);

        client.init(&admin);
        client.approve_farmer(&farmer);

        client.claim(&farmer);
        client.claim(&farmer);
    }

    #[test]
    fn test_multiple_farmers() {
        let env = Env::default();

        let contract_id = env.register(AgriGrantWallet, ());
        let client = AgriGrantWalletClient::new(&env, &contract_id);

        let admin = Address::generate(&env);

        let farmer1 = Address::generate(&env);
        let farmer2 = Address::generate(&env);

        client.init(&admin);

        client.approve_farmer(&farmer1);
        client.approve_farmer(&farmer2);

        client.claim(&farmer1);

        assert!(client.has_claimed(&farmer1));
        assert!(!client.has_claimed(&farmer2));
    }
}
