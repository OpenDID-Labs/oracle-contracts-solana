use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    bpf_loader, bpf_loader_upgradeable, instruction::Instruction, program::invoke_signed,
    system_program,
};

use crate::*;
#[derive(Accounts)]
pub struct FulfillOracleRequest<'info> {
    #[account(mut)]
    pub oracle_requester: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [COMMITMENT_SEED, commitment_pda.requester.as_ref(), &commitment_pda.pda_seed],
        bump = commitment_pda.bump,
    )]
    pub commitment_pda: Account<'info, Commitment>,
    #[account(mut)]
    pub ovn: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl FulfillOracleRequest<'_> {
    pub fn apply(
        ctx: &mut Context<FulfillOracleRequest>,
        request_id: [u8; 32],
        data: String,
    ) -> Result<()> {
        let (target_program, store_pda) = {
            require!(
                ctx.remaining_accounts.len() >= 2,
                OracleError::InsufficientAccounts
            );
            (&ctx.remaining_accounts[0], &ctx.remaining_accounts[1])
        };

        validate_request_id(&ctx.accounts.commitment_pda.key(), request_id)?;

        let (_, index) = validate_ovn(&ctx.accounts.commitment_pda.ovns, &ctx.accounts.ovn.key())?;

        validate_callback_accounts(
            &ctx.accounts.commitment_pda,
            target_program.key,
            store_pda.key,
        )?;

        update_commitment_state(
            &mut ctx.accounts.commitment_pda,
            index,
            ctx.accounts.oracle_requester.to_account_info(),
        )?;

        emit_fulfill_event(request_id, &ctx.accounts.ovn.key())?;

        // CPI call
        if is_valid_program(target_program.owner) {
            execute_cpi_call(
                target_program,
                &ctx.remaining_accounts,
                ctx.accounts.commitment_pda.callback_function_id,
                request_id,
                &data,
            )?;
        }
        Ok(())
    }
}
/// Verify that the request ID matches the commitment pda
fn validate_request_id(commitment_pda_key: &Pubkey, request_id: [u8; 32]) -> Result<()> {
    require!(
        commitment_pda_key.to_bytes() == request_id,
        OracleError::InvalidRequestId
    );
    Ok(())
}

fn validate_ovn(ovns: &[Pubkey], ovn_key: &Pubkey) -> Result<(bool, usize)> {
    let result = _verify_ovn_specified(ovns, ovn_key)?;
    require!(result.0, OracleError::NonSpecifiedOvn);
    Ok(result)
}

fn _verify_ovn_specified(ovns: &[Pubkey], ovn_key: &Pubkey) -> Result<(bool, usize)> {
    ovns.iter()
        .enumerate()
        .find(|(_, ovn)| *ovn == ovn_key)
        .map(|(i, _)| Ok((true, i)))
        .unwrap_or_else(|| Ok((false, 0)))
}

fn validate_callback_accounts(
    commitment: &Commitment,
    target_program_key: &Pubkey,
    store_pda_key: &Pubkey,
) -> Result<()> {
    require!(
        commitment.callback_addr == *target_program_key,
        OracleError::MismatchedTargetProgram
    );

    require!(
        commitment.callback_pda == *store_pda_key,
        OracleError::MismatchedStorePda
    );

    Ok(())
}

fn update_commitment_state(
    commitment: &mut Account<Commitment>,
    current_ovn_index: usize,
    oracle_requester: AccountInfo,
) -> Result<()> {
    let ovn_count = commitment.ovns.len();
    commitment.fulfill_count += 1;
    if ovn_count == 1 {
        // Single OVN scenario: Clear job_id
        commitment.job_id = [0u8; 32];
        commitment.ovns.pop();
        // Close the PDA account and refund to the oracle_requester account
        require!(
            commitment.requester == oracle_requester.key(),
            OracleError::MismatchedRequester
        );
        close_pda(commitment.to_account_info(), oracle_requester)?;
    } else {
        // Multiple OVN scenarios: Remove the current OVN 
        commitment.ovns.swap(current_ovn_index, ovn_count - 1);
        commitment.ovns.pop();
    }

    Ok(())
}
fn close_pda(info: AccountInfo, sol_destination: AccountInfo) -> Result<()> {
    // Transfer tokens from the account to the sol_destination.
    let dest_starting_lamports = sol_destination.lamports();
    **sol_destination.lamports.borrow_mut() =
        dest_starting_lamports.checked_add(info.lamports()).unwrap();
    **info.lamports.borrow_mut() = 0;

    info.assign(&system_program::ID);
    info.resize(0).map_err(Into::into)
}
fn emit_fulfill_event(request_id: [u8; 32], ovn_key: &Pubkey) -> Result<()> {
    emit!(FulfillOracleRequested {
        request_id,
        ovn: *ovn_key,
    });
    Ok(())
}

fn is_valid_program(owner: &Pubkey) -> bool {
    *owner == bpf_loader::id() || *owner == bpf_loader_upgradeable::id()
}

fn execute_cpi_call(
    target_program: &AccountInfo,
    remaining_accounts: &[AccountInfo],
    callback_function_id: [u8; 8],
    request_id: [u8; 32],
    data: &str,
) -> Result<()> {
    let instruction_data = build_cpi_data(callback_function_id, request_id, data)?;

    let cpi_accounts = remaining_accounts
        .iter()
        .skip(1)
        .map(|acc| acc.to_account_metas(None)[0].clone())
        .collect::<Vec<_>>();

    let ix = Instruction {
        program_id: *target_program.key,
        accounts: cpi_accounts,
        data: instruction_data,
    };

    invoke_signed(&ix, remaining_accounts, &[]).map_err(|e| {
        msg!("CPI call failed: {:?}", e);
        OracleError::CpiFailed
    })?;

    Ok(())
}

/// Construct the serialized data for the CPI call
fn build_cpi_data(
    callback_function_id: [u8; 8],
    request_id: [u8; 32],
    data: &str,
) -> Result<Vec<u8>> {
    let mut instruction_data = callback_function_id.to_vec();
    instruction_data.extend_from_slice(&request_id);

    // Append strings with length prefixes
    let data_bytes = data.as_bytes();
    instruction_data.extend_from_slice(&(data_bytes.len() as u32).to_le_bytes());
    instruction_data.extend_from_slice(data_bytes);

    Ok(instruction_data)
}
