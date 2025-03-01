use crate::error::ReviewError;
use crate::instruction::MovieInstruction;
use crate::state::MovieAccountState;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{ Sysvar, rent::Rent},
    program_pack::{IsInitialized},
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MovieInstruction::unpack(instruction_data)?;
    match instruction {
        MovieInstruction::AddMovieReview {
            title,
            rating,
            description,
        } => add_movie_review(program_id, accounts, title, rating, description),
    }
}

pub fn add_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String,
) -> ProgramResult {
    // Logging instruction data that was passed in
    msg!("Adding movie review...");
    msg!("Title: {}", title);
    msg!("Rating: {}", rating);
    msg!("Description: {}", description);

    // Get Account iterator
    let account_info_iter = &mut accounts.iter();

    // Get accounts
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Derive PDA
    let (pda, bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), title.as_bytes().as_ref()],
        program_id,
    );

    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ReviewError::InvalidPDA.into());
    }

    if rating > 5 || rating < 1 {
        msg!("Rating cannot be higher than 5");
        return Err(ReviewError::InvalidRating.into());
    }

    let total_len: usize = 1 + 1 + (4 + title.len()) + (4 + description.len());
    if total_len > 1000 {
        msg!("Data length is larger than 1000 bytes");
        return Err(ReviewError::InvalidDataLength.into());
    }

    // Calculate account size required
    // let account_len: usize = 1 + 1 + (4 + title.len()) + (4 + description.len());
    let account_len = 1000;

    // Calculate rent required
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    // Create the account
    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[
            initializer.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        //@note learn: for PDA info to invoke_signed? how it is used in this function
        &[&[
            initializer.key.as_ref(),
            title.as_bytes().as_ref(),
            &[bump_seed],
        ]],
    )?;

    msg!("PDA created: {}", pda);

    msg!("Unpacking state account");
    let mut account_data = MovieAccountState::try_from_slice(&pda_account.data.borrow())
        .unwrap_or(MovieAccountState::default());
    msg!("Borrowed account data");

    msg!("checking if movie account is already initialized");
    if account_data.is_initialized() {
      msg!("Account already initialized");
      return Err(ProgramError::AccountAlreadyInitialized);
  }

    account_data.title = title;
    account_data.rating = rating;
    account_data.description = description;
    account_data.is_initialized = true;

    // Finally, serialize the updated account_data into the data field of pda_account.
    msg!("Serializing account");
    //@note learn: syntax
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("State account serialized");

    Ok(())
}
