// First we include what we are going to need in our program. 
// This  is the Rust style of importing things.
// Remember we added the dependencies in cargo.toml
// And from the `solana_program` crate we are including  all the required things.
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};


fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // We check if We have a instruction_data len greater than 0 if it is not we do not want to procced.
    // So we return Error with InvalidInstructionData message.
     if instruction_data.len() == 0 {
        return Err(ProgramError::InvalidInstructionData);
    }
    /// Now we just check and call the function for each of them.
    // I have choosen 0 for create_campaign,
    // 1 for withdraw
    // 2 for donate.
    if instruction_data[0] == 0 {
        return create_campaign(
            program_id,
            accounts,
            /// Notice we pass program_id and accounts as they where 
            // but we pass a reference to slice of [instruction_data]. 
            /// we do not want the first element in any of our functions.
            &instruction_data[1..instruction_data.len()],
        );
    } else if instruction_data[0] == 1 {
        return withdraw(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    } else if instruction_data[0] == 2 {
        return donate(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    }

    /// If instruction_data doesn't match we give an error.
    // Note I have used msg!() macro and passed a string here. 
    // It is good to do this as this would 
    // also get printed in the console window
    // if a program fails.
    msg!("Didn't find the entrypoint required");
    Err(ProgramError::InvalidInstructionData)
}

entrypoint!(process_instruction);


/// Here, I have created the function for every action we want to do in our program.
/// They take in the same parameters as process_intruction and also have the same return type.

fn create_campaign(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}

fn withdraw(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}

fn donate(
    program_id: &Pubkey, 
    accounts: &[AccountInfo], 
    _instruction_data: &[u8]
) -> ProgramResult {
    Ok(())
}