use cosmwasm_std::{ Response, MessageInfo, Deps};
use crate::errors::ContractError;
use crate::state::read_config;

pub fn assert_kyc_provider_privilege(
    deps: Deps,
    info: MessageInfo
) -> Result<Response, ContractError> {
    if read_config(deps.storage)?.kyc_provider_address != deps.api.addr_canonicalize(info.sender.as_str())? {
        return Err(ContractError::Unauthorized {});
    }

    Ok(Response::default())
}
