use crate::*;

#[derive(Accounts)]
pub struct ManageFeeSetter<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [ORACLE_SEED],
        bump = oracle_pda.bump,
    )]
    pub oracle_pda: Account<'info, OracleSettings>,
}

impl ManageFeeSetter<'_> {
    pub fn apply(
        ctx: &mut Context<ManageFeeSetter>,
        fee_setter: Pubkey,
        authorized: bool,
    ) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle_pda;
        require!(
            oracle.is_admin(ctx.accounts.signer.key),
            OracleError::Unauthorized
        );

        if authorized {
            require!(
                !oracle.fee_setters.contains(&fee_setter),
                OracleError::AlreadyExists
            );
            oracle.fee_setters.push(fee_setter);
        } else {
            require!(
                oracle.fee_setters.contains(&fee_setter),
                OracleError::NotFound
            );

            if let Some(pos) = oracle.fee_setters.iter().position(|&p| p == fee_setter) {
                oracle.fee_setters.remove(pos);
            }
        }

        emit!(FeeSetterChanged {
            setter: fee_setter,
            authorized: authorized,
        });

        Ok(())
    }
}
