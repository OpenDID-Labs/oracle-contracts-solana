use crate::*;

#[account]
pub struct Claim {
    pub operator: Pubkey,
    pub bump: u8,
    pub claim_id: [u8; 32],
    pub claim_data: String,
}
