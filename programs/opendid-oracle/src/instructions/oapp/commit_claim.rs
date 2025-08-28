use crate::*;

#[derive(Accounts)]
#[instruction(claim_id: [u8; 32],claim: String)]
pub struct CommitClaim<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        seeds = [ORACLE_SEED],
        bump = oracle_pda.bump,
    )]
    pub oracle_pda: Account<'info, OracleSettings>,
    #[account(
        init,
        seeds = [&claim_id],
        bump,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + 32 + 1 + 32 + 4 + claim.len(),
    )]
    pub claim_pda: Account<'info, Claim>,
    pub system_program: Program<'info, System>,
}

impl CommitClaim<'_> {
    pub fn apply(ctx: &mut Context<CommitClaim>, claim_id: [u8; 32], claim: String) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle_pda;

        require!(
            oracle.is_aurhorized_operator(ctx.accounts.signer.key),
            OracleError::UnauthorizedOperator
        );
        require!(claim_id != [0; 32], OracleError::InvalidClaimId);
        require!(!claim.is_empty(), OracleError::EmptyClaim);

        let new_claim = &mut ctx.accounts.claim_pda;
        new_claim.operator = ctx.accounts.signer.key();
        new_claim.bump = ctx.bumps.claim_pda;
        new_claim.claim_id = claim_id;
        new_claim.claim_data = claim;

        emit!(ClaimCommitted {
            claim_id,
            operator: ctx.accounts.signer.key().clone(),
        });
        Ok(())
    }
}
