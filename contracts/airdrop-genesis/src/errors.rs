use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Already claimed")]
    AlreadyClaimed {},

    #[error("Do tasks to claim more")]
    DoMoreTasks {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Pending owner missing")]
    PendingOwnerMissing {},

    #[error("Cannot convert address to canonical")]
    CannotConvertAddressToCanonical {},

    #[error("Cannot convert address to human")]
    CannotConvertAddressToHuman {},

    #[error("Balance is empty")]
    BalanceIsEmpty {},

    #[error("UST native token balance sent to low")]
    UstBalanceSentToLow {},
}
