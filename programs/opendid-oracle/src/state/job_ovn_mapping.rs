use crate::*;

#[account]
pub struct JobOvnMapping {
    pub admin: Pubkey, 
    pub bump: u8,      
    pub job_id: [u8; 32],
    pub ovns: Vec<Pubkey>,
}

impl JobOvnMapping {
    pub fn get_job_ovns(&self, job_id: [u8; 32]) -> Result<Vec<Pubkey>> {
        require!(self.job_id == job_id, OracleError::InvalidJobId);
        Ok(self.ovns.clone())
    }
}
