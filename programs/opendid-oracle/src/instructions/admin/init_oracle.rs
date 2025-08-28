use crate::*;

#[derive(Accounts)]
pub struct InitOracle<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + OracleSettings::INIT_SPACE,
        seeds = [ORACLE_SEED],
        bump
      )]
    pub oracle_pda: Account<'info, OracleSettings>,
    pub system_program: Program<'info, System>,
}

impl InitOracle<'_> {
    pub fn apply(ctx: &mut Context<InitOracle>) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle_pda;
        oracle.admin = ctx.accounts.signer.key();
        oracle.bump = ctx.bumps.oracle_pda;

        Ok(())
    }
}
