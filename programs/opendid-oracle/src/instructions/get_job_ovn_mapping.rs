use crate::*;

#[derive(Accounts)]
#[instruction(job_id: [u8; 32])]
pub struct GetJobOvnMapping<'info> {
    #[account(
        seeds = [&job_id],
        bump = job_ovn_mapping_pda.bump,
    )]
    pub job_ovn_mapping_pda: Account<'info, JobOvnMapping>,
}

impl GetJobOvnMapping<'_> {
    pub fn apply(ctx: &Context<GetJobOvnMapping>, job_id: [u8; 32]) -> Result<Vec<Pubkey>> {
        let job_ovn_mapping = &ctx.accounts.job_ovn_mapping_pda;
        job_ovn_mapping.get_job_ovns(job_id)
    }
}
