pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
pub use errors::*;
pub use events::*;
pub use instructions::*;
pub use state::*;

declare_id!("DKwV8oEV9J4uP8eHePtxWnF48pntHukgjSkWQxqvutHR");

pub const ORACLE_SEED: &[u8] = b"OracleDev";
pub const ORACLE_JOB_OVN_SEED: &[u8] = b"OracleJobOvnSettings";

pub const COMMITMENT_SEED: &[u8] = b"commitment";

pub const FEESETTER_MAX_LEN: usize = 5; 
pub const OPERATOR_MAX_LEN: usize = 20; 
pub const JOB_MAX_LEN: usize = 50; 
/// Anchor discriminator size (8 bytes)
pub const ANCHOR_DISCRIMINATOR: usize = 8;
#[program]
pub mod opendid_oracle {
    use super::*;
    // ------------------ Admin Instructions ------------------
    // Initialize oracle PDA
    pub fn init_oracle(mut ctx: Context<InitOracle>) -> Result<()> {
        InitOracle::apply(&mut ctx)
    }

    //withdraw job fee
    pub fn withdraw_fee(mut ctx: Context<WithdrawFee>, amount: u64) -> Result<()> {
        WithdrawFee::apply(&mut ctx, amount)
    }

    // pub fn transfer_admin(
    //     mut ctx: Context<TransferAdmin>,
    //     params: TransferAdminParams,
    // ) -> Result<()> {
    //     TransferAdmin::apply(&mut ctx, &params)
    // }

    // Manage FeeSetter (add or delete)
    pub fn set_fee_setter(
        mut ctx: Context<ManageFeeSetter>,
        fee_setter: Pubkey,
        authorized: bool,
    ) -> Result<()> {
        ManageFeeSetter::apply(&mut ctx, fee_setter, authorized)
    }

    // Manage Operator (add or delete)
    pub fn set_operator(
        mut ctx: Context<ManageOperator>,
        operator: Pubkey,
        authorized: bool,
    ) -> Result<()> {
        ManageOperator::apply(&mut ctx, operator, authorized)
    }

    // ------------------ Business Instructions ------------------
    // Set expiration time 
    pub fn set_expiry_time(mut ctx: Context<SetExpiryTime>, expiry_time: u64) -> Result<()> {
        SetExpiryTime::apply(&mut ctx, expiry_time)
    }

    // Set job fee 
    pub fn set_messaging_fees(
        mut ctx: Context<SetMessagingFee>,
        fees: Vec<MessagingFee>,
    ) -> Result<()> {
        SetMessagingFee::apply(&mut ctx, fees)
    }

    // Query job fees
    pub fn get_messaging_fee(
        ctx: Context<GetMessagingFee>,
        job_id: [u8; 32],
    ) -> Result<MessagingFee> {
        GetMessagingFee::apply(&ctx, job_id)
    }

    // Set claim fee
    pub fn set_claim_fee(mut ctx: Context<SetClaimFee>, fee: ClaimFee) -> Result<()> {
        SetClaimFee::apply(&mut ctx, fee)
    }

    // Query claim fee
    pub fn get_claim_fee(mut ctx: Context<GetClaimFee>) -> Result<ClaimFee> {
        GetClaimFee::apply(&mut ctx)
    }

    // Query the total cost of the job fee
    pub fn quote(ctx: Context<Quote>, job_id: [u8; 32], generate_claim: bool) -> Result<u64> {
        Quote::apply(&ctx, job_id, generate_claim)
    }

    // Create the mapping relationship between the job and ovn
    pub fn set_job_ovns(
        mut ctx: Context<CreateJobOvnMapping>,
        job_id: [u8; 32],
        ovns: Vec<Pubkey>,
    ) -> Result<()> {
        CreateJobOvnMapping::apply(&mut ctx, job_id, ovns)
    }

    // Update the mapping relationship between the job and ovn
    pub fn update_job_ovns(
        mut ctx: Context<UpdateJobOvnMapping>,
        job_id: [u8; 32],
        ovns: Vec<Pubkey>,
    ) -> Result<()> {
        UpdateJobOvnMapping::apply(&mut ctx, job_id, ovns)
    }

    // Query the mapping relationship between job and ovn
    pub fn get_job_ovns(ctx: Context<GetJobOvnMapping>, job_id: [u8; 32]) -> Result<Vec<Pubkey>> {
        GetJobOvnMapping::apply(&ctx, job_id)
    }

    // Initiate an Oracle request
    pub fn oracle_request(
        mut ctx: Context<OracleRequest>,
        pda_seed: [u8; 32],
        job_id: [u8; 32],
        ovns: Vec<Pubkey>,
        callback_address: Pubkey,
        callback_pda: Pubkey,
        generate_claim: bool,
        data: String,
        amount: u64,
    ) -> Result<[u8; 32]> {
        OracleRequest::apply(
            &mut ctx,
            pda_seed,
            job_id,
            ovns,
            callback_address,
            callback_pda,
            generate_claim,
            data,
            amount,
        )
    }

    // Cancel an Oracle request
    pub fn cancel_oracle_request(
        mut ctx: Context<CancelOracleRequest>,
        request_id: [u8; 32],
    ) -> Result<()> {
        CancelOracleRequest::apply(&mut ctx, request_id)
    }

    // fulfill an Oracle request
    pub fn fulfill_oracle_request(
        mut ctx: Context<FulfillOracleRequest>,
        request_id: [u8; 32],
        data: String,
    ) -> Result<()> {
        FulfillOracleRequest::apply(&mut ctx, request_id, data)
    }

    // commit claim
    pub fn commit_claim(
        mut ctx: Context<CommitClaim>,
        claim_id: [u8; 32],
        claim: String,
    ) -> Result<()> {
        CommitClaim::apply(&mut ctx, claim_id, claim)
    }
}
