use crate::*;

#[derive(Accounts)]
pub struct GetMessagingFee<'info> {
    #[account(
        seeds = [ORACLE_SEED],
        bump = oracle_pda.bump,
    )]
    pub oracle_pda: Account<'info, OracleSettings>,
}

impl GetMessagingFee<'_> {
    pub fn apply(ctx: &Context<GetMessagingFee>, job_id: [u8; 32]) -> Result<MessagingFee> {
        let oracle = &ctx.accounts.oracle_pda;

        let fee = oracle
            .get_messaging_fee(&job_id)
            .ok_or(OracleError::NotFound)?;

        Ok(fee.clone())
    }
}
