use borsh::{BorshDeserialize, BorshSerialize};


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



/// For an example, let us create human struct.
#[derive(Debug)]
struct Human {
    /// we can add all the fields in our struct here.
    /// we also have to specify the type of each variable.
    /// Like the [eyes_color] here is a `String` type object.
    pub eyes_color: String,
    pub name: String,
    pub height: i32,
}


#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct CampaignDetails {
    pub admin: Pubkey,
    pub name: String,
    pub description: String,
    pub image_link: String,
    /// we will be using this to know the total amount 
    /// donated to a campaign.
    pub amount_donated: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct WithdrawRequest {
    pub amount: u64,
}


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
    let accounts_iter = &mut accounts.iter();

     /// Writing account or we can call it program account.
    /// This is an account we will create in our front-end.
    /// This account should br owned by the solana program.
    let writing_account = next_account_info(accounts_iter)?;

    /// Account of the person creating the campaign.
    let creator_account = next_account_info(accounts_iter)?;

    // Now to allow transactions we want the creator account to sign the transaction.
    if !creator_account.is_signer {
        msg!("creator_account should be signer");
        return Err(ProgramError::IncorrectProgramId);
    }
    /// We want to write in this account so we want its owner by the program.
    if writing_account.owner != program_id {
        msg!("writing_account isn't owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut input_data = CampaignDetails::try_from_slice(&instruction_data)
    .expect("Instruction data serialization didn't worked");

    // Now I want that for a campaign created the only admin should be the one who created it.
    // You can add additional logical here to check things like
    // The image url should not be null
    // The name shouldn't be smaller than some specific length...
    if input_data.admin != *creator_account.key {
    msg!("Invaild instruction data");
    return Err(ProgramError::InvalidInstructionData);
    }
    /// get the minimum balance we need in our program account.
    let rent_exemption = Rent::get()?.minimum_balance(writing_account.data_len());
    /// And we make sure our program account (`writing_account`) has that much lamports(balance).
    if **writing_account.lamports.borrow() < rent_exemption {
        msg!("The balance of writing_account should be more then rent_exemption");
        return Err(ProgramError::InsufficientFunds);
    }
    // Then we can set the initial amount donate to be zero.
    input_data.amount_donated=0;
    input_data.serialize(&mut &mut writing_account.data.borrow_mut()[..])?;
    Ok(())
}

fn withdraw(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let writing_account = next_account_info(accounts_iter)?;
    let admin_account = next_account_info(accounts_iter)?;
    
    // We check if the writing account is owned by program.
    if writing_account.owner != program_id {
        msg!("writing_account isn't owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }
    // Admin account should be the signer in this trasaction.
    if !admin_account.is_signer {
        msg!("admin should be signer");
        return Err(ProgramError::IncorrectProgramId);
    }
    // Just like we used the try_from_slice for 
    // instruction_data we will use it for the 
    // writing_account's data.
    let campaign_data = CampaignDetails::try_from_slice(*writing_account.data.borrow())
        .expect("Error deserializing data");

    // Then we check if the admin_account's public key is equal to
    // the public key we have stored in our campaign_data.
    if campaign_data.admin != *admin_account.key {
        msg!("Only the account admin can withdraw");
        return Err(ProgramError::InvalidAccountData);
    }


    // Here we make use of the struct we created.
    // We will get the amount of lamports admin wants to withdraw
    let input_data = WithdrawRequest::try_from_slice(&instruction_data)
        .expect("Instruction data serialization didn't worked");
        let rent_exemption = Rent::get()?.minimum_balance(writing_account.data_len());

    /// We check if we have enough funds
    if **writing_account.lamports.borrow() - rent_exemption < input_data.amount {
        msg!("Insufficent balance");
        return Err(ProgramError::InsufficientFunds);
    }

    // Transfer balance
    // We will decrease the balance of the program account, and increase the admin_account balance.
    **writing_account.try_borrow_mut_lamports()? -= input_data.amount;
    **admin_account.try_borrow_mut_lamports()? += input_data.amount;
    Ok(())
}

fn donate(
    program_id: &Pubkey, 
    accounts: &[AccountInfo], 
    _instruction_data: &[u8]
) -> ProgramResult {
    Ok(())
}