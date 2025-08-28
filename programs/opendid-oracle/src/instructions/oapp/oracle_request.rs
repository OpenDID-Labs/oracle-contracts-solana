use anchor_lang::solana_program::{program, system_instruction};
use crate::*;
#[event_cpi]
#[derive(Accounts)]
#[instruction(pda_seed: [u8; 32],job_id: [u8; 32], ovns: Vec<Pubkey>)]
pub struct OracleRequest<'info> {
    #[account(
        mut,
        seeds = [ORACLE_SEED],
        bump = oracle_pda.bump,
    )]
    pub oracle_pda: Account<'info, OracleSettings>,
    #[account(
        seeds = [&job_id],
        bump = job_ovn_mapping_pda.bump,
    )]
    pub job_ovn_mapping_pda: Account<'info, JobOvnMapping>,
    #[account(
        init,
        seeds = [COMMITMENT_SEED, requester.key().as_ref(), &pda_seed],
        bump,
        payer = requester,
        space =  ANCHOR_DISCRIMINATOR        // Discriminator
        + 32      // job_id ([u8; 32])
        + 32      // callback_addr 
        + 32      // callback_pda 
        + 8      // callback_function_id 
        + 8       // amount (u64)
        + 8       // expiration (i64)
        + 32      // requester (Pubkey)
        + 4 + (32 * ovns.len())          // ovns 
        + 1       // generate_claim (bool)
        + 8       // fulfill_count (u64)
        + 32       // pda_seed
        + 1,      //bump
    )]
    pub commitment_pda: Account<'info, Commitment>,
    #[account(mut)]
    pub requester: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// "oracle_response(request_id: [u8; 32], data: String)"
pub const CALLBACK_FUNCTION_ID: [u8; 8] = [238, 231, 190, 148, 216, 135, 40, 26];

impl OracleRequest<'_> {
    pub fn apply(
        ctx: &mut Context<OracleRequest>,
        pda_seed: [u8; 32],
        job_id: [u8; 32],
        ovns: Vec<Pubkey>,
        callback_address: Pubkey,
        callback_pda: Pubkey,
        generate_claim: bool,
        data: String,
        amount: u64,
    ) -> Result<[u8; 32]> {
        let oracle_pda_info = ctx.accounts.oracle_pda.to_account_info();
        let oracle = &ctx.accounts.oracle_pda;
        let job_ovn_mapping_pda = &ctx.accounts.job_ovn_mapping_pda;

        let commitment = &mut ctx.accounts.commitment_pda;
        let requester_account_info = ctx.accounts.requester.to_account_info();

        let requester = &mut ctx.accounts.requester;
        require!(pda_seed != [0; 32], OracleError::InvalidSeed);

        let configured_ovns = job_ovn_mapping_pda.get_job_ovns(job_id)?;
        for ovn in &ovns {
            require!(configured_ovns.contains(ovn), OracleError::InvalidOvns)
        }
        let job_fee = oracle.quote(&job_id, generate_claim)?;
        let total_fee = job_fee * ovns.len() as u64;
        if total_fee > 0 {
            require!(amount >= total_fee, OracleError::InsufficientPayment);
            program::invoke(
                &system_instruction::transfer(
                    &requester_account_info.key(), 
                    &oracle_pda_info.key(),
                    amount, // tip：not "total_fee"
                ),
                &[
                    requester_account_info.clone(),
                    oracle_pda_info.clone(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;
        }
        let current_time = Clock::get()?.unix_timestamp;
        commitment.job_id = job_id;
        commitment.callback_addr = callback_address.clone();
        commitment.callback_pda = callback_pda.clone();
        commitment.callback_function_id = CALLBACK_FUNCTION_ID;
        commitment.amount = amount;
        commitment.expiration = current_time + oracle.expiry_time as i64;
        commitment.requester = requester.key().clone();
        commitment.ovns = ovns;
        commitment.generate_claim = generate_claim;
        commitment.fulfill_count = 0;
        commitment.pda_seed = pda_seed;
        commitment.bump = ctx.bumps.commitment_pda;

        emit_cpi!(OracleRequested {
            job_id,
            request_id: commitment.key().to_bytes(),
            requester: requester.key(),
            callback_address: callback_address,
            callback_pda: callback_pda,
            ovns: commitment.ovns.clone(),
            generate_claim,
            amount,
            data,
            pda_seed: pda_seed,
        });

        Ok(commitment.key().to_bytes())
    }
}
