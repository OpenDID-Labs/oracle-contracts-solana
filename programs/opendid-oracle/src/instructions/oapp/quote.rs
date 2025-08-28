use crate::*;

#[derive(Accounts)]
pub struct Quote<'info> {
    #[account(
        seeds = [ORACLE_SEED],
        bump = oracle_pda.bump,
    )]
    pub oracle_pda: Account<'info, OracleSettings>,
}

impl Quote<'_> {
    pub fn apply(ctx: &Context<Quote>, job_id: [u8; 32], generate_claim: bool) -> Result<u64> {
        let oracle = &ctx.accounts.oracle_pda;
        oracle.quote(&job_id, generate_claim)
    }
}
