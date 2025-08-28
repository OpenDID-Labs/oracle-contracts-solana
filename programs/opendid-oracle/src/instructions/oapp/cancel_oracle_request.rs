use crate::*;

#[event_cpi]
#[derive(Accounts)]
pub struct CancelOracleRequest<'info> {
    #[account(
        mut,
        seeds = [ORACLE_SEED],
        bump = oracle_pda.bump,
    )]
    pub oracle_pda: Account<'info, OracleSettings>,
    #[account(
        mut,
        seeds = [COMMITMENT_SEED, commitment_pda.requester.as_ref(), &commitment_pda.pda_seed],
        bump = commitment_pda.bump,
        close = refunder, 
    )]
    pub commitment_pda: Account<'info, Commitment>,
    #[account(mut)]
    pub requester: Signer<'info>,
    /// CHECK: Not required to be SystemAccount
    #[account(mut)]
    pub refunder: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl CancelOracleRequest<'_> {
    pub fn apply(ctx: &mut Context<CancelOracleRequest>, request_id: [u8; 32]) -> Result<()> {
        let commitment = &mut ctx.accounts.commitment_pda;
        require!(
            commitment.key().to_bytes() == request_id,
            OracleError::InvalidRequestId
        );
        require!(
            commitment.job_id != [0u8; 32],
            OracleError::InvalidOperation
        );

        let current_time = Clock::get()?.unix_timestamp;
        require!(commitment.expiration < current_time, OracleError::NotYetDue);

        require!(
            commitment.requester == ctx.accounts.requester.key(),
            OracleError::MismatchedRequester
        );

        require!(
            commitment.fulfill_count == 0,
            OracleError::FulfillmentRecordsExist
        );

        let amount = commitment.amount;
        if amount > 0 {
            let required_lamports =
                Rent::get()?.minimum_balance(ANCHOR_DISCRIMINATOR + OracleSettings::INIT_SPACE);
            let surplus_lamports = ctx.accounts.oracle_pda.get_lamports() - required_lamports;
            require!(surplus_lamports >= amount, OracleError::InsufficientBalance);

            ctx.accounts.oracle_pda.sub_lamports(amount)?;
            ctx.accounts.refunder.add_lamports(amount)?;
        }

        emit_cpi!(OracleRequestCanceled {
            job_id: commitment.job_id,
            request_id,
            callback_address: commitment.callback_addr,
            callback_pda: commitment.callback_pda,
            amount: commitment.amount,
            refund_address: ctx.accounts.refunder.key(),
        });
        Ok(())
    }
}
