use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    sol_pub_unit8::sol_log_pubkey_as_unit8,
};
use solana_system_interface::instruction as system_instruction;

pub fn transfer_sol_with_cpi(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let recipient = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    msg!("payer in base58: {}", payer.key);
    msg!("recipient in base58: {}", recipient.key);

    msg!("Payer in Unit8 array:- ",);
    sol_log_pubkey_as_unit8(payer.key);
    msg!("Recipient in Unit8 array:- ",);
    sol_log_pubkey_as_unit8(recipient.key);

    invoke(
        &system_instruction::transfer(payer.key, recipient.key, amount),
        &[payer.clone(), recipient.clone(), system_program.clone()],
    )?;

    Ok(())
}
