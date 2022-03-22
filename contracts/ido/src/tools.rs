use crate::errors::ContractError;
use crate::state::read_config;
use cosmwasm_std::{Deps, MessageInfo, Response};

pub fn assert_owner_privilege(deps: Deps, info: MessageInfo) -> Result<Response, ContractError> {
    if read_config(deps.storage)?.owner != deps.api.addr_canonicalize(&info.sender.as_str())? {
        return Err(ContractError::Unauthorized {});
    }

    Ok(Response::default())
}
