use crate::*;

#[event]
pub struct AdminTransferred {
    pub previous_admin: Pubkey,
    pub new_admin: Pubkey,
}

#[event]
pub struct FeeSetterChanged {
    pub setter: Pubkey,
    pub authorized: bool,
}

#[event]
pub struct OperatorChanged {
    pub operator: Pubkey,
    pub authorized: bool,
}

#[event]
pub struct ExpirytimeChanged {
    pub before: u64,
    pub current: u64,
}

#[event]
pub struct MessagingFeesChanged {
    pub sender: Pubkey,
    pub fees: Vec<MessagingFee>,
}

#[event]
pub struct ClaimFeeChanged {
    pub sender: Pubkey,
    pub before: ClaimFee,
    pub current: ClaimFee,
}

#[event]
pub struct JobOvnMappingChanged {
    pub sender: Pubkey,
    pub job_id: [u8; 32],
    pub before: Vec<Pubkey>,
    pub current: Vec<Pubkey>,
}
#[event]
pub struct OracleRequested {
    pub job_id: [u8; 32],
    pub request_id: [u8; 32],
    pub requester: Pubkey,
    pub callback_address: Pubkey,
    pub callback_pda: Pubkey,
    pub ovns: Vec<Pubkey>,
    pub generate_claim: bool,
    pub amount: u64,
    pub data: String,
    pub pda_seed: [u8; 32],
}
#[event]
pub struct OracleRequestCanceled {
    pub job_id: [u8; 32],
    pub request_id: [u8; 32],
    pub callback_address: Pubkey,
    pub callback_pda: Pubkey,
    pub amount: u64,
    pub refund_address: Pubkey,
}
#[event]
pub struct FulfillOracleRequested {
    pub request_id: [u8; 32],
    pub ovn: Pubkey,
}

#[event]
pub struct ClaimCommitted {
    pub claim_id: [u8; 32],
    pub operator: Pubkey,
}
#[event]
pub struct Withdrawn {
    pub sender: Pubkey, 
    pub to: Pubkey,    
    pub amount: u64,   
}
