use crate::*;

#[derive(InitSpace, Clone, AnchorSerialize, AnchorDeserialize,Default)]
pub struct ClaimFee {
    pub free: bool,
    pub gas_amount: u64,
}
