use crate::*;

#[derive(Accounts)]
pub struct InitOracleJobOvn<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + OracleJobOvnSettings::INIT_SPACE,
        seeds = [ORACLE_JOB_OVN_SEED],
        bump
      )]
    pub oracle_job_ovn: Account<'info, OracleJobOvnSettings>,
    pub system_program: Program<'info, System>,
}

impl InitOracleJobOvn<'_> {
    pub fn apply(ctx: &mut Context<InitOracleJobOvn>) -> Result<()> {
        let oracle_job_ovn = &mut ctx.accounts.oracle_job_ovn;
        oracle_job_ovn.admin = ctx.accounts.signer.key();
        oracle_job_ovn.bump = ctx.bumps.oracle_job_ovn;
        Ok(())
    }
}
