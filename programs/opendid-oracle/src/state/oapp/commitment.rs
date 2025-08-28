use crate::*;

#[account]
pub struct Commitment {
    pub job_id: [u8; 32],
    pub callback_addr: Pubkey,
    pub callback_pda: Pubkey,
    pub callback_function_id: [u8; 8],
    pub amount: u64,
    pub expiration: i64,
    pub requester: Pubkey,
    pub ovns: Vec<Pubkey>,
    pub generate_claim: bool,
    pub fulfill_count: u64,
    pub pda_seed: [u8; 32],
    pub bump: u8, 
}

