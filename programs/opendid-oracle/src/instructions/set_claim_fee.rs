use crate::*;

#[derive(Accounts)]
pub struct SetClaimFee<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [ORACLE_SEED],
        bump = oracle_pda.bump,
    )]
    pub oracle_pda: Account<'info, OracleSettings>,
}

impl SetClaimFee<'_> {
    pub fn apply(ctx: &mut Context<SetClaimFee>, fee: ClaimFee) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle_pda;
        require!(
            oracle.is_authorized_fee_setter(ctx.accounts.signer.key),
            OracleError::Unauthorized
        );
        let before = oracle.claim_fee.clone();
        oracle.claim_fee = fee.clone();
        emit!(ClaimFeeChanged {
            sender: ctx.accounts.signer.key(),
            before,
            current: fee,
        });
        Ok(())
    }
}
