use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Provided vesting address already registered")]
    AddressAlreadyRegistered {},

    #[error("Provided address is not registered")]
    AddressNotRegistered {},

    #[error("Pending owner missing")]
    PendingOwnerMissing {},

    #[error("You can not add next vesting address (max {max:})")]
    CannotAddMoreVestingAddresses { max: u64 },

    #[error("Too many vesting addresses (max {max:})")]
    CannotHaveMoreVestingAddresses { max: u64 },
}
