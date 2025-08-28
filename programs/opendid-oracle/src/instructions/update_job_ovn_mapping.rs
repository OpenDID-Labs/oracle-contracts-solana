use crate::*;

#[derive(Accounts)]
#[instruction(job_id: [u8; 32], ovns: Vec<Pubkey>)]
pub struct UpdateJobOvnMapping<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
         seeds = [&job_id],
        bump =job_ovn_mapping_pda.bump,
        realloc = ANCHOR_DISCRIMINATOR + 32 + 1 + 32 + 4 + ovns.len() * 32,
        realloc::payer = signer,
        realloc::zero = true,
      )]
    pub job_ovn_mapping_pda: Account<'info, JobOvnMapping>,

    #[account(
        seeds = [ORACLE_SEED],
        bump = oracle_pda.bump,
    )]
    pub oracle_pda: Account<'info, OracleSettings>,
    pub system_program: Program<'info, System>,
}

impl UpdateJobOvnMapping<'_> {
    pub fn apply(
        ctx: &mut Context<UpdateJobOvnMapping>,
        job_id: [u8; 32],
        ovns: Vec<Pubkey>,
    ) -> Result<()> {
        let oracle = &ctx.accounts.oracle_pda;
        let job_ovn_mapping = &mut ctx.accounts.job_ovn_mapping_pda;
        require!(
            oracle.is_authorized_fee_setter(ctx.accounts.signer.key),
            OracleError::NonAuthorizedFeeSetter
        );
        require!(job_id == job_ovn_mapping.job_id, OracleError::InvalidJobId);
        require!(ovns.len() > 0, OracleError::InvalidOvns);
        for ovn in &ovns {
            require!(oracle.is_aurhorized_operator(ovn), OracleError::InvalidOvns);
        }
        let before = job_ovn_mapping.ovns.clone();

        job_ovn_mapping.admin = ctx.accounts.signer.key();
        job_ovn_mapping.ovns = ovns.clone();

        emit!(JobOvnMappingChanged {
            sender: ctx.accounts.signer.key(),
            job_id: job_id,
            before: before,
            current: ovns,
        });
        Ok(())
    }
}
