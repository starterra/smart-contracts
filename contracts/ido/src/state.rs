use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, StdResult, Storage, Uint128};
use cosmwasm_storage::{singleton, singleton_read, Bucket, ReadonlyBucket};
use starterra_token::common::OrderBy;
use starterra_token::ido::{ParticipantInfoResponse, ParticipantResponse};

static KEY_CONFIG: &[u8] = b"config";
static KEY_STATE: &[u8] = b"state";

static PREFIX_KEY_IDO_PARTICIPANT: &[u8] = b"ido_participant";
static PREFIX_KEY_PARTICIPANT: &[u8] = b"participant";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: CanonicalAddr,
    pub pending_owner: Option<CanonicalAddr>,
    pub prefund_address: CanonicalAddr,
    pub kyc_terms_vault_address: CanonicalAddr,
    pub ido_token: CanonicalAddr,
    pub ido_token_price: Uint128,
    pub end_date: u64,
    pub paused: bool,
    pub snapshot_time: Option<u64>,
    pub minimum_prefund: Uint128,
}

pub fn store_config(storage: &mut dyn Storage, config: &Config) -> StdResult<()> {
    singleton(storage, KEY_CONFIG).save(config)
}

pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
    singleton_read(storage, KEY_CONFIG).load()
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub number_of_participants: u64,
}

pub fn store_state(storage: &mut dyn Storage, state: &State) -> StdResult<()> {
    singleton(storage, KEY_STATE).save(state)
}

pub fn read_state(storage: &dyn Storage) -> StdResult<State> {
    singleton_read(storage, KEY_STATE).load()
}

pub fn store_ido_participant(
    storage: &mut dyn Storage,
    address: &CanonicalAddr,
    participant_info: &ParticipantInfoResponse,
) -> StdResult<()> {
    Ok(Bucket::new(storage, PREFIX_KEY_IDO_PARTICIPANT).save(address, participant_info)?)
}

pub fn remove_ido_participant(storage: &mut dyn Storage, address: &CanonicalAddr) {
    Bucket::<ParticipantInfoResponse>::new(storage, PREFIX_KEY_IDO_PARTICIPANT).remove(address)
}

pub fn read_ido_participant(
    storage: &dyn Storage,
    address: &CanonicalAddr,
) -> StdResult<ParticipantInfoResponse> {
    Ok(ReadonlyBucket::new(storage, PREFIX_KEY_IDO_PARTICIPANT).load(address)?)
}

pub fn store_participant(
    storage: &mut dyn Storage,
    address: &CanonicalAddr,
    participant_info: &ParticipantResponse,
) -> StdResult<()> {
    Ok(
        Bucket::<ParticipantResponse>::new(storage, PREFIX_KEY_PARTICIPANT)
            .save(address, participant_info)?,
    )
}

pub fn read_participant(
    storage: &dyn Storage,
    address: &CanonicalAddr,
) -> StdResult<ParticipantResponse> {
    match ReadonlyBucket::new(storage, PREFIX_KEY_PARTICIPANT).may_load(address)? {
        Some(found) => Ok(found),
        None => Ok(ParticipantResponse { is_joined: false }),
    }
}

const DEFAULT_LIMIT: u32 = 1024;
pub fn read_participants(
    storage: &dyn Storage,
    start_after: Option<CanonicalAddr>,
    limit: Option<u32>,
    order_by: Option<OrderBy>,
) -> StdResult<Vec<CanonicalAddr>> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT) as usize;
    let (start, end, order_by) = match order_by {
        Some(OrderBy::Asc) => (calc_range_start_addr(start_after), None, OrderBy::Asc),
        _ => (None, calc_range_end_addr(start_after), OrderBy::Desc),
    };

    let participants: ReadonlyBucket<ParticipantResponse> =
        ReadonlyBucket::new(storage, PREFIX_KEY_PARTICIPANT);

    return participants
        .range(start.as_deref(), end.as_deref(), order_by.into())
        .take(limit)
        .map(|item| {
            let (k, _) = item?;
            Ok(CanonicalAddr::from(k))
        })
        .collect();
}

// this will set the first key after the provided key, by appending a 1 byte
fn calc_range_start_addr(start_after: Option<CanonicalAddr>) -> Option<Vec<u8>> {
    start_after.map(|addr| {
        let mut v = addr.as_slice().to_vec();
        v.push(1);
        v
    })
}

// this will set the first key after the provided key, by appending a 1 byte
fn calc_range_end_addr(start_after: Option<CanonicalAddr>) -> Option<Vec<u8>> {
    start_after.map(|addr| addr.as_slice().to_vec())
}
