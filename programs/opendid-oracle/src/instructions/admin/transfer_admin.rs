use crate::*;

#[derive(Accounts)]
pub struct TransferAdmin<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [ORACLE_SEED],
        bump = oracle_pda.bump,
    )]
    pub oracle_pda: Account<'info, OracleSettings>,
}

impl TransferAdmin<'_> {
    pub fn apply(ctx: &mut Context<TransferAdmin>, params: &TransferAdminParams) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle_pda;
        require!(
            oracle.is_admin(ctx.accounts.signer.key),
            OracleError::Unauthorized
        );
        oracle.admin = params.admin;
        emit!(AdminTransferred {
            previous_admin: *ctx.accounts.signer.key,
            new_admin: params.admin,
        });

        Ok(())
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TransferAdminParams {
    pub admin: Pubkey,
}
