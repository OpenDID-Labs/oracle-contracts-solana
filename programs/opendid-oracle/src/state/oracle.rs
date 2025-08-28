use crate::*;

#[account]
#[derive(InitSpace, Default)]
pub struct OracleSettings {
    pub admin: Pubkey, 
    pub bump: u8,      
    #[max_len(FEESETTER_MAX_LEN)]
    pub fee_setters: Vec<Pubkey>, 
    #[max_len(OPERATOR_MAX_LEN)]
    pub operators: Vec<Pubkey>, 
    pub expiry_time: u64, 
    #[max_len(JOB_MAX_LEN)]
    pub messaging_fees: Vec<MessagingFee>, 
    pub claim_fee: ClaimFee, 
}

impl OracleSettings {
    pub fn is_admin(&self, account: &Pubkey) -> bool {
        self.admin == *account
    }

    pub fn is_authorized_fee_setter(&self, account: &Pubkey) -> bool {
        self.fee_setters.contains(account)
    }

    pub fn is_aurhorized_operator(&self, account: &Pubkey) -> bool {
        self.operators.contains(account)
    }

    pub fn get_messaging_fee(&self, job_id: &[u8; 32]) -> Option<&MessagingFee> {
        self.messaging_fees.iter().find(|fee| fee.job_id == *job_id)
    }

    pub fn get_claim_fee(&self) -> &ClaimFee {
        &self.claim_fee
    }

    pub fn quote(&self, job_id: &[u8; 32], generate_claim: bool) -> Result<u64> {
        let mut total_fee = 0;
        let messaging_fee = self
            .get_messaging_fee(job_id)
            .ok_or(OracleError::NotFound)?;
        if !messaging_fee.free {
            total_fee += messaging_fee.gas_amount;
        }
        if generate_claim {
            let claim_fee = self.get_claim_fee();
            if !claim_fee.free {
                total_fee += claim_fee.gas_amount;
            }
        }
        Ok(total_fee)
    }
}
