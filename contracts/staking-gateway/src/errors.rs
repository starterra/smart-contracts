use cosmwasm_std::{StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Cannot convert address to canonical")]
    CannotConvertAddressToCanonical {},

    #[error("Cannot convert address to human")]
    CannotConvertAddressToHuman {},

    #[error("Maximum number of staking contracts is 5")]
    MaximumNumberOfStaking {},

    #[error("User cannot stake in more than one contract")]
    CannotStakeInMoreThanOneContract {},

    #[error("Pending owner missing")]
    PendingOwnerMissing {},
}
