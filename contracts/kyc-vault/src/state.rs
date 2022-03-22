use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, StdResult, Storage};
use cosmwasm_storage::{singleton, singleton_read, Bucket, ReadonlyBucket, Singleton};

static KEY_CONFIG: &[u8] = b"config";
static KEY_PENDING_OWNER: &[u8] = b"pending_owner";

static PREFIX_KEY_KYC_ADDRESS: &[u8] = b"kyc_address";
static PREFIX_KEY_TOU_ADDRESS: &[u8] = b"tou_address";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: CanonicalAddr,
    pub kyc_provider_address: CanonicalAddr,
}

pub fn store_config(storage: &mut dyn Storage, config: &Config) -> StdResult<()> {
    singleton(storage, KEY_CONFIG).save(config)
}

pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
    singleton_read(storage, KEY_CONFIG).load()
}

pub fn store_kyc_address(
    storage: &mut dyn Storage,
    address: &CanonicalAddr,
    is_registering: bool,
) -> StdResult<()> {
    Ok(Bucket::<bool>::new(storage, PREFIX_KEY_KYC_ADDRESS).save(address, &is_registering)?)
}

pub fn read_kyc_address(storage: &dyn Storage, address: &CanonicalAddr) -> StdResult<bool> {
    Ok(ReadonlyBucket::new(storage, PREFIX_KEY_KYC_ADDRESS).load(address)?)
}

pub fn store_tou_address(
    storage: &mut dyn Storage,
    address: &CanonicalAddr,
    is_accepted: bool,
) -> StdResult<()> {
    Ok(Bucket::<bool>::new(storage, PREFIX_KEY_TOU_ADDRESS).save(address, &is_accepted)?)
}

pub fn read_tou_address(storage: &dyn Storage, address: &CanonicalAddr) -> StdResult<bool> {
    match ReadonlyBucket::new(storage, PREFIX_KEY_TOU_ADDRESS).may_load(address)? {
        Some(found) => Ok(found),
        None => Ok(false),
    }
}

pub fn store_pending_owner(storage: &mut dyn Storage, new_owner: &CanonicalAddr) -> StdResult<()> {
    singleton(storage, KEY_PENDING_OWNER).save(new_owner)
}

pub fn read_pending_owner(storage: &dyn Storage) -> Option<CanonicalAddr> {
    singleton_read(storage, KEY_PENDING_OWNER)
        .may_load()
        .unwrap()
}

pub fn remove_pending_owner(storage: &mut dyn Storage) {
    Singleton::<CanonicalAddr>::new(storage, KEY_PENDING_OWNER).remove();
}
