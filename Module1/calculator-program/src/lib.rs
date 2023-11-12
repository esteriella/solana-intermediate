// Import necessary external crates and Solana program dependencies
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Define a struct for the calculator account, which can be serialized and deserialized using Borsh
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CalculatorAccount {
    pub operand1: u32,
    pub operand2: u32,
    pub result: u32,
}

// Define the program's entry point function, which is called when the program is invoked
entrypoint!(process_instruction);

// Implement the program's entry point function
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Create an iterator to iterate through the provided accounts
    let accounts_iter = &mut accounts.iter();
    // Get the next account from the iterator
    let account = next_account_info(accounts_iter)?;

    // Check if the owner of the account matches the program's ID
    if account.owner != program_id {
        msg!("Calculator account does not have the correct program ID");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize the calculator account from the provided account data
    let mut calculator_account = CalculatorAccount::try_from_slice(&account.data.borrow())?;

    // Match the first byte of instruction_data to determine the operation to perform
    match instruction_data[0] {
        0 => {
            // Addition operation
            calculator_account.result = calculator_account.operand1 + calculator_account.operand2;
        }
        1 => {
            // Subtraction operation with saturation
            calculator_account.result = calculator_account.operand1.saturating_sub(calculator_account.operand2);
        }
        _ => {
            // Invalid instruction
            msg!("Invalid instruction");
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    // Serialize the updated calculator account back into the provided account data
    calculator_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    // Display the calculated result
    msg!("Calculated result: {}", calculator_account.result);

    Ok(())
}

// Test module for unit testing the program
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    // Unit test to check the program's functionality
    #[test]
    fn test_sanity() {
        // Create test data for the calculator account
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<CalculatorAccount>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        // 0 for addition
        let instruction_data: Vec<u8> = vec![0];

        // Create a vector of accounts for testing
        let accounts = vec![account];

        // Deserialize the calculator account from the test account data
        let mut calculator_account = CalculatorAccount::try_from_slice(&accounts[0].data.borrow()).unwrap();
        calculator_account.operand1 = 10;
        calculator_account.operand2 = 5;
        // Store the operands in the account
        calculator_account.serialize(&mut &mut accounts[0].data.borrow_mut()[..]).unwrap(); 

        // Invoke the program's entry point function for testing
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        let calculator_account = CalculatorAccount::try_from_slice(&accounts[0].data.borrow()).unwrap();

        // Assert that the result has been calculated correctly
        assert_eq!(calculator_account.result, 15);
    }
}
