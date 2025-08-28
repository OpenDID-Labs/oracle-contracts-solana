use crate::*;

#[derive(Accounts)]
pub struct SetExpiryTime<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [ORACLE_SEED],
        bump = oracle_pda.bump,
    )]
    pub oracle_pda: Account<'info, OracleSettings>,
}

impl SetExpiryTime<'_> {
    pub fn apply(ctx: &mut Context<SetExpiryTime>, expiry_time: u64) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle_pda;
        require!(
            oracle.fee_setters.contains(ctx.accounts.signer.key),
            OracleError::NonAuthorizedFeeSetter
        );
        require!(expiry_time > 0, OracleError::NonZero);
        let before_expiry_time = oracle.expiry_time;
        oracle.expiry_time = expiry_time;

        emit!(ExpirytimeChanged {
            before: before_expiry_time,
            current: expiry_time
        });
        Ok(())
    }
}
