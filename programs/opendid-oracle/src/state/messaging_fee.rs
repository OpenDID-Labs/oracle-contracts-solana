use crate::*;

#[derive(InitSpace,Clone, AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Hash,Default)]
pub struct MessagingFee {
    pub job_id: [u8; 32],
    pub free: bool,
    pub gas_amount: u64,
}
