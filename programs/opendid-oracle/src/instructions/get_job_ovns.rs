use crate::*;

#[derive(Accounts)]
pub struct GetJobOvns<'info> {
    pub oracle_job_ovn: Account<'info, OracleJobOvnSettings>,
}

impl GetJobOvns<'_> {
    pub fn apply(ctx: &mut Context<GetJobOvns>, job_id: [u8; 32]) -> Result<Vec<Pubkey>> {
        let oracle_job_ovn = &mut ctx.accounts.oracle_job_ovn;

        let mapping = oracle_job_ovn
            .get_job_ovns(&job_id)
            .ok_or(OracleError::NotFound)?;

        Ok(mapping.ovns.clone())
    }
}
