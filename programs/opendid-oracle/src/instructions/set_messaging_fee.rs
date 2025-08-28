use crate::*;

#[derive(Accounts)]
pub struct SetMessagingFee<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [ORACLE_SEED],
        bump = oracle_pda.bump,
    )]
    pub oracle_pda: Account<'info, OracleSettings>,
}

impl SetMessagingFee<'_> {
    pub fn apply(ctx: &mut Context<SetMessagingFee>, fees: Vec<MessagingFee>) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle_pda;
        require!(
            oracle.is_authorized_fee_setter(ctx.accounts.signer.key),
            OracleError::Unauthorized
        );
        for fee in fees.clone() {
            if let Some(pos) = oracle
                .messaging_fees
                .iter()
                .position(|f| f.job_id == fee.job_id)
            {
                oracle.messaging_fees[pos] = fee;
            } else {
                oracle.messaging_fees.push(fee);
            }
        }

        emit!(MessagingFeesChanged {
            sender: ctx.accounts.signer.key(),
            fees: fees,
        });
        Ok(())
    }
}
