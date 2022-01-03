use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, StdResult, Storage};
use cosmwasm_storage::{singleton, singleton_read, Singleton};

static KEY_CONFIG: &[u8] = b"config";
static KEY_PENDING_OWNER: &[u8] = b"pending_owner";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: CanonicalAddr,
    pub staking_contracts: Vec<CanonicalAddr>,
}

pub fn store_config(storage: &mut dyn Storage, config: &Config) -> StdResult<()> {
    singleton(storage, KEY_CONFIG).save(config)
}

pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
    singleton_read(storage, KEY_CONFIG).load()
}

pub fn store_pending_owner(storage: &mut dyn Storage, new_owner: &CanonicalAddr) -> StdResult<()> {
    singleton(storage, KEY_PENDING_OWNER).save(new_owner)
}

pub fn read_pending_owner(storage: &dyn Storage) -> Option<CanonicalAddr> {
    singleton_read(storage, KEY_PENDING_OWNER).may_load().unwrap()
}

pub fn remove_pending_owner(storage: &mut dyn Storage) {
    Singleton::<CanonicalAddr>::new(storage, KEY_PENDING_OWNER).remove();
}
