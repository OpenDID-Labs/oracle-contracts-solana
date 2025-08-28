use anchor_lang::prelude::error_code;

#[error_code]
pub enum OracleError {
    #[msg("Max capacity reached")]
    MaxCapacity,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Already exists")]
    AlreadyExists,
    #[msg("Not found")]
    NotFound,
    #[msg("Non authorized fee setter")]
    NonAuthorizedFeeSetter,
    #[msg("Non authorized operator")]
    UnauthorizedOperator,
    #[msg("Non zero")]
    NonZero,
    #[msg("Insufficient payment")]
    InsufficientPayment,
    #[msg("Duplicated requestId")]
    DuplicatedRequestId,
    #[msg("Invalid ovns")]
    InvalidOvns,
    #[msg("Invalid requestId")]
    InvalidRequestId,
    #[msg("Non-specified ovn")]
    NonSpecifiedOvn,
    #[msg("Callback error occurred")]
    CallbackError,
    #[msg("Invalid jobId")]
    InvalidJobId,
    #[msg("Invalid claim ID")]
    InvalidClaimId,
    #[msg("Invalid operation")]
    InvalidOperation,
    #[msg("Claim cannot be empty")]
    EmptyClaim,
    #[msg("Claim already exists")]
    ClaimAlreadyExists,
    #[msg("Not yet due")]
    NotYetDue,
    #[msg("Mismatched requester")]
    MismatchedRequester,
    #[msg("Fulfillment records exist")]
    FulfillmentRecordsExist,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Insufficient accounts,The target program account and store PDA account are required")]
    InsufficientAccounts,
    #[msg("Mismatched target program account")]
    MismatchedTargetProgram,
    #[msg("Mismatched target store pda account")]
    MismatchedStorePda,
    #[msg("CPI call failed")]
    CpiFailed,
    #[msg("Zero addres")]
    ZeroAddress,
    #[msg("Transfer failed")]
    TransferFailed,
    #[msg("Invalid seed")]
    InvalidSeed,
}
