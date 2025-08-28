use crate::*;

#[derive(Accounts)]
#[instruction(job_id: [u8; 32], ovns: Vec<Pubkey>)]
pub struct CreateJobOvnMapping<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + 32 + 1 + 32 + 4 + ovns.len() * 32,
        seeds = [&job_id],
        bump
      )]
    pub job_ovn_mapping_pda: Account<'info, JobOvnMapping>,

    #[account(
        seeds = [ORACLE_SEED],
        bump = oracle_pda.bump,
    )]
    pub oracle_pda: Account<'info, OracleSettings>,
    pub system_program: Program<'info, System>,
}

impl CreateJobOvnMapping<'_> {
    pub fn apply(
        ctx: &mut Context<CreateJobOvnMapping>,
        job_id: [u8; 32],
        ovns: Vec<Pubkey>,
    ) -> Result<()> {
        let oracle = &ctx.accounts.oracle_pda;

        require!(
            oracle.is_authorized_fee_setter(ctx.accounts.signer.key),
            OracleError::NonAuthorizedFeeSetter
        );

        require!(job_id != [0; 32], OracleError::InvalidJobId);

        require!(ovns.len() > 0, OracleError::InvalidOvns);
        for ovn in &ovns {
            require!(oracle.is_aurhorized_operator(ovn), OracleError::InvalidOvns);
        }
        let job_ovn_mapping = &mut ctx.accounts.job_ovn_mapping_pda;
        job_ovn_mapping.admin = ctx.accounts.signer.key();
        job_ovn_mapping.bump = ctx.bumps.job_ovn_mapping_pda;
        job_ovn_mapping.job_id = job_id;
        job_ovn_mapping.ovns = ovns.clone();
        emit!(JobOvnMappingChanged {
            sender: ctx.accounts.signer.key(),
            job_id: job_id,
            before: vec![],
            current: ovns,
        });
        Ok(())
    }
}
