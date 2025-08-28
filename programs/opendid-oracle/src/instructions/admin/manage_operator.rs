use crate::*;

#[derive(Accounts)]
pub struct ManageOperator<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [ORACLE_SEED],
        bump = oracle_pda.bump,
    )]
    pub oracle_pda: Account<'info, OracleSettings>,
}

impl ManageOperator<'_> {
    pub fn apply(
        ctx: &mut Context<ManageOperator>,
        operator: Pubkey,
        authorized: bool,
    ) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle_pda;
        require!(
            oracle.is_admin(ctx.accounts.signer.key),
            OracleError::Unauthorized
        );

        if authorized {
            require!(
                !oracle.operators.contains(&operator),
                OracleError::AlreadyExists
            );
            oracle.operators.push(operator);
        } else {
            require!(oracle.operators.contains(&operator), OracleError::NotFound);
            if let Some(pos) = oracle.operators.iter().position(|&p| p == operator) {
                oracle.operators.remove(pos);
            }
        }

        emit!(OperatorChanged {
            operator,
            authorized,
        });

        Ok(())
    }
}
