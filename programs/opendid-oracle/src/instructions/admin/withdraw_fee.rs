use crate::*;

#[derive(Accounts)]
pub struct WithdrawFee<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [ORACLE_SEED],
        bump = oracle_pda.bump,
    )]
    pub oracle_pda: Account<'info, OracleSettings>,
    /// CHECK: Not required to be SystemAccount
    #[account(mut)]
    pub to: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl WithdrawFee<'_> {
    pub fn apply(ctx: &mut Context<WithdrawFee>, amount: u64) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle_pda;
        require!(
            oracle.fee_setters.contains(ctx.accounts.signer.key),
            OracleError::NonAuthorizedFeeSetter
        );
        let to = &ctx.accounts.to;
        require!(to.key() != Pubkey::default(), OracleError::ZeroAddress);

        let required_lamports =
            Rent::get()?.minimum_balance(ANCHOR_DISCRIMINATOR + OracleSettings::INIT_SPACE);
        let surplus_lamports = ctx.accounts.oracle_pda.get_lamports() - required_lamports;
        require!(surplus_lamports >= amount, OracleError::InsufficientBalance);

        ctx.accounts.oracle_pda.sub_lamports(amount)?;
        to.add_lamports(amount)?;

        emit!(Withdrawn {
            sender: ctx.accounts.signer.key(),
            to: to.key(),
            amount,
        });

        Ok(())
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TransferAdminParams {
    pub admin: Pubkey,
}
