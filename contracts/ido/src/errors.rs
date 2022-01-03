use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Pending owner missing")]
    PendingOwnerMissing {},

    #[error("You are not eligible to join IDO")]
    NotEligibleToJoinIdo {},

    #[error("You already joined to the IDO")]
    AlreadyJoined {},

    #[error("IDO is paused")]
    IdoPaused {},

    #[error("IDO is closed")]
    IdoClosed {},

    #[error("You have to deposit more on prefund contract")]
    NotEnoughDeposit {},

    #[error("You have to be kyc verified")]
    KycFailed {},

    #[error("You have to accept terms of use")]
    TouFailed {},

    #[error("User already joined and cant be edited")]
    UserJoinedAndCantBeEdited {},

    #[error("Snapshot time can not be in the past")]
    SnapshotTimeFromPast {},

    #[error("End date can not be in the past")]
    EndDateInThePast {},
}
