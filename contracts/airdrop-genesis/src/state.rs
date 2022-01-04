use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, StdResult, Storage, Uint128};
use cosmwasm_storage::{singleton, singleton_read, bucket, bucket_read, Singleton};
use starterra_token::airdrop_genesis::AirdropInfo;

static KEY_CONFIG: &[u8] = b"config";

static PREFIX_KEY_AIRDROP_INFO: &[u8] = b"airdrop_info";
static KEY_PENDING_OWNER: &[u8] = b"pending_owner";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: CanonicalAddr,
    pub starterra_token: CanonicalAddr,
    pub lp_staking_addresses: Vec<CanonicalAddr>,
    pub stt_staking_addresses: Vec<CanonicalAddr>,
    pub ido_addresses: Vec<CanonicalAddr>,
    pub claim_fee: Uint128,
}

pub fn store_config(storage: &mut dyn Storage, config: &Config) -> StdResult<()> {
    singleton(storage, KEY_CONFIG).save(config)
}

pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
    singleton_read(storage, KEY_CONFIG).load()
}

pub fn read_airdrop_info(
    storage: &dyn Storage,
    address: &CanonicalAddr,
) -> StdResult<AirdropInfo> {
    Ok(bucket_read::<AirdropInfo>(storage, PREFIX_KEY_AIRDROP_INFO).load(address.as_slice())?)
}

pub fn store_airdrop_info(
    storage: &mut dyn Storage,
    address: &CanonicalAddr,
    airdrop_info: &AirdropInfo,
) -> StdResult<()> {
    Ok(bucket::<AirdropInfo>(storage, PREFIX_KEY_AIRDROP_INFO)
        .save(address.as_slice(), airdrop_info)?)
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
