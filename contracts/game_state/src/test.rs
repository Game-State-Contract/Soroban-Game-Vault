#[cfg(test)]
mod tests {
    use soroban_sdk::{testutils::Address as _, Address, Bytes, BytesN, Env};
    use crate::{GameStateContract, GameStateContractClient, Condition, ConditionOp};

    fn setup() -> (Env, Address, GameStateContractClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, GameStateContract);
        let client = GameStateContractClient::new(&env, &contract_id);
        let player = Address::generate(&env);
        (env, player, client)
    }

    #[test]
    fn test_store_and_retrieve_state() {
        let (env, player, client) = setup();

        let blob = Bytes::from_slice(&env, b"encrypted_inventory_data");
        let hash = BytesN::from_array(&env, &[1u8; 32]);

        client.store_state(&player, &blob, &hash);

        let entry = client.get_state(&player).unwrap();
        assert_eq!(entry.encrypted_blob, blob);
        assert_eq!(entry.state_hash, hash);
    }

    #[test]
    fn test_get_state_returns_none_when_empty() {
        let (_, player, client) = setup();
        assert!(client.get_state(&player).is_none());
    }

    #[test]
    fn test_clear_state() {
        let (env, player, client) = setup();

        let blob = Bytes::from_slice(&env, b"some_state");
        let hash = BytesN::from_array(&env, &[2u8; 32]);

        client.store_state(&player, &blob, &hash);
        client.clear_state(&player);

        assert!(client.get_state(&player).is_none());
    }

    #[test]
    fn test_commit_hash_updates_hash() {
        let (env, player, client) = setup();

        let blob = Bytes::from_slice(&env, b"state");
        let hash_v1 = BytesN::from_array(&env, &[1u8; 32]);
        let hash_v2 = BytesN::from_array(&env, &[2u8; 32]);

        client.store_state(&player, &blob, &hash_v1);
        client.commit_hash(&player, &hash_v2);

        let entry = client.get_state(&player).unwrap();
        assert_eq!(entry.state_hash, hash_v2);
        // blob should remain unchanged
        assert_eq!(entry.encrypted_blob, blob);
    }
}
