use crate::*;

#[derive(Accounts)]
pub struct GetClaimFee<'info> {
    #[account(
        seeds = [ORACLE_SEED],
        bump = oracle_pda.bump,
    )]
    pub oracle_pda: Account<'info, OracleSettings>,
}

impl GetClaimFee<'_> {
    pub fn apply(ctx: &Context<GetClaimFee>) -> Result<ClaimFee> {
        let oracle = &ctx.accounts.oracle_pda;
        Ok(oracle.get_claim_fee().clone())
    }
}
