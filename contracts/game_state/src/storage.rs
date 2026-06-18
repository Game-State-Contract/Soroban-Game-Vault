use soroban_sdk::{Address, Env};
use crate::StateEntry;

const STATE_KEY: &str = "state";

pub fn save_state(env: &Env, player: &Address, entry: &StateEntry) {
    env.storage().persistent().set(&(STATE_KEY, player), entry);
}

pub fn load_state(env: &Env, player: &Address) -> Option<StateEntry> {
    env.storage().persistent().get(&(STATE_KEY, player))
}

pub fn delete_state(env: &Env, player: &Address) {
    env.storage().persistent().remove(&(STATE_KEY, player));
}
