use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

use crate::{
    error::RentShareError,
    instruction::RentShareInstruction,
    state::{AgreementStatus, RentShareAccount},
};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = RentShareInstruction::unpack(instruction_data)?;

        match instruction {
            RentShareInstruction::InitializeEvent {
                payee_pubkey, // reveed the money.
                event_creator,
                // payer_pubkey, //A payor is a person paying money to someone else
                event_id,
                fix_deposit_amount_per_person,
                total_partcipator
            } => Self::initialize_event(
                accounts,
                program_id,
                payee_pubkey,
                event_creator,

                // payer_pubkey,
                event_id,
                fix_deposit_amount_per_person,
                total_partcipator
                // duration_unit,
            ),
            RentShareInstruction::CancelEvent {
                // payee_pubkey, // reveed the money.
                event_creator,

                // payer_pubkey, //A payor is a person paying money to someone else
                event_id
                // fix_deposit_amount_per_person,
                // total_partcipator,
            } => Self::cancel_event(
                accounts,
                program_id,
                event_creator
                // payee_pubkey,
                // payer_pubkey,
                event_id,
                // fix_deposit_amount_per_person,
                // total_partcipator,
                // duration_unit,
            ),
            RentShareInstruction::EndEvent {
                // payee_pubkey, // reveed the money.
                
                // payer_pubkey, //A payor is a person paying money to someone else
                event_id,
                event_creator
                // start_date,
                // ending_date,
            } => Self::end_event(
                accounts,
                program_id,
                // payee_pubkey,
                // payer_pubkey,
                event_id,
                event_creator
                // start_date,
                // ending_date,
                // duration_unit,
            ),
            RentShareInstruction::ParticipateInEvent {
                // payee_pubkey, // reveed the money.
                // payer_pubkey, //A payor is a person paying money to someone else
                event_id,
                particpate_amount,
                event_creator
            } => Self::start_event(
                accounts,
                program_id,
                // payee_pubkey,
                // payer_pubkey,
                event_id,
                particpate_amount,
                event_creator
                // ending_date,
                // duration_unit,
            ),
            // RentShareInstruction::DistributeAmountToWinner { rent_amount } => {
            //     Self::pay_rent(accounts, program_id, rent_amount)
            // }
            // RentShareInstruction::TerminateEarly {} => Self::terminate_early(accounts, program_id),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn initialize_event(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
        payee_pubkey: Pubkey, // the party receiving the payment is known as the payee.
        event_creator:Pubkey,
        // payer_pubkey: Pubkey,
        event_id: u64,
        fix_deposit_amount_per_person: u64,
        total_partcipator: u64,
        // duration_unit: u8,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let rent_agreement_account = next_account_info(accounts_iter)?;
        if rent_agreement_account.owner != program_id {
            msg!(" Event account not owned by this program");
            return Err(ProgramError::IncorrectProgramId);
        }

        // let solana_rent = &Rent::from_account_info(next_account_info(accounts_iter)?)?;
        // // Make sure this account is rent exemtpt
        // if !solana_rent.is_exempt(
        //     rent_agreement_account.lamports(),
        //     rent_agreement_account.data_len(),
        // ) {
        //     msg!(
        //         "[RentShare] Rent agreement account not rent exempt. Balance: {}",
        //         rent_agreement_account.lamports()
        //     );
        //     return Err(ProgramError::AccountNotRentExempt);
        // }

        // Initialize the Rent Agreement Account with the initial data
        // Note: the structure of the data state must match the `space` reserved when account created
        let rent_agreement_data =
            RentShareAccount::try_from_slice(&rent_agreement_account.data.borrow());

        if rent_agreement_data.is_err() {
            msg!(
                "[RentShare] Rent agreement account data size incorrect: {}",
                rent_agreement_account.try_data_len()?
            );
            return Err(ProgramError::InvalidAccountData);
        }

        let mut rent_data = rent_agreement_data.unwrap();
        if rent_data.is_initialized() {
            msg!("[RentShare] Rent agreement already initialized");
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        // if rent_data.event_id!=0{

        // }

        rent_data.status = AgreementStatus::Active as u8;
        rent_data.payee_pubkey = payee_pubkey;
        // rent_data.payer_pubkey = payer_pubkey;
        // rent_data.rent_amount = rent_amount;
        rent_data.event_creator = event_creator;
        rent_data.event_id = event_id;
        rent_data.fix_deposit_amount_per_person = fix_deposit_amount_per_person;
        rent_data.total_partcipator = total_partcipator;
        // rent_data.remaining_payments = duration;
        // rent_data.serialize(&mut &mut rent_agreement_account.data.borrow_mut()[..])?;

        msg!(
            "[RentShare] Initialized rent agreement account: {:?}",
            rent_data
        );
        rent_data.serialize(&mut &mut rent_agreement_account.data.borrow_mut()[..])?;

        Ok(())
    }


    // cancel event-----------
    // fn cancel_event(
    //     accounts: &[AccountInfo],
    //     program_id: &Pubkey,
    //     event_creator:pubkey,
    //     event_id:u64
    // ){
    //     let accounts_iter = &mut accounts.iter();

    //     let rent_agreement_account = next_account_info(accounts_iter)?;
    //     if rent_agreement_account.owner != program_id {
    //         msg!("[RentShare] Rent agreement account is not owned by this program");
    //         return Err(ProgramError::IncorrectProgramId);
    //     }

    //     // let payee_account: &AccountInfo = next_account_info(accounts_iter)?;
    //     // let payer_account = next_account_info(accounts_iter)?;
    //     let system_program_account = next_account_info(accounts_iter)?;

    //     if !event_creator.is_signer {
    //         return Err(ProgramError::MissingRequiredSignature);
    //     }
    //     let rent_agreement_data =
    //     RentShareAccount::try_from_slice(&rent_agreement_account.data.borrow());

    // if rent_agreement_data.is_err() {
    //     msg!(
    //         "[RentShare] Rent agreement account data size incorrect: {}",
    //         rent_agreement_account.try_data_len()?
    //     );
    //     return Err(ProgramError::InvalidAccountData);
    // }

    // let mut rent_data = rent_agreement_data.unwrap();
    // // AgreementStatus::Active
    // //************************************************************88
    // if rent_data.is_initialized() {
    //     msg!("[RentShare] Rent agreement already initialized");
    //     return Err(ProgramError::AccountAlreadyInitialized);
    // }
    // //*************************************************************

    // if rent_data.event_creator != event_creator {
    //     msg!(
    //         "[RentShare] Rent amount does not match agreement amount: {} vs {}",
    //         rent_data.rent_amount,
    //         rent_amount
    //     );
    //     return Err(RentShareError::RentPaymentAmountMismatch.into());
    // }

    // if rent_data.event_id != event_id {
    //     msg!(
    //         "[RentShare] Rent amount does not match agreement amount: {} vs {}",
    //         rent_data.rent_amount,
    //         rent_amount
    //     );
    //     return Err(RentShareError::RentPaymentAmountMismatch.into());
    // }
    // rent_data.status = AgreementStatus::Completed as u8;
    // rent_data.status = AgreementStatus::Active as u8;
    // rent_data.payee_pubkey = payee_pubkey;
    // // rent_data.payer_pubkey = payer_pubkey;
    // // rent_data.rent_amount = rent_amount;
    // rent_data.event_creator = event_creator;
    // rent_data.event_id = event_id;
    // rent_data.fix_deposit_amount_per_person = fix_deposit_amount_per_person;
    // rent_data.total_partcipator = total_partcipator;
    // // rent_data.remaining_payments = duration;
    // // rent_data.serialize(&mut &mut rent_agreement_account.data.borrow_mut()[..])?;

    // msg!(
    //     "[RentShare] Initialized rent agreement account: {:?}",
    //     rent_data
    // );



    // }
    fn start_event(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
        event_id:u64,
        particpate_amount:u64,
        event_creator:Pubkey
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let rent_agreement_account = next_account_info(accounts_iter)?;
        if rent_agreement_account.owner != program_id {
            msg!("[RentShare] Rent agreement account is not owned by this program");
            return Err(ProgramError::IncorrectProgramId);
        }

        // let payee_account: &AccountInfo = next_account_info(accounts_iter)?;
        // let payer_account = next_account_info(accounts_iter)?;
        // let system_program_account = next_account_info(accounts_iter)?;

        if !event_creator.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // if payer_account.lamports() < rent_amount {
        //     return Err(ProgramError::InsufficientFunds);
        // }

        // Transfer to self - do nothing
        // if payer_account.key == payee_account.key {
        //     return Ok(());
        // }

        // Initialize the Rent Agreement Account with the initial data
        // Note: the structure of the data state must match the `space` the client used to create the account
        let rent_agreement_data =
            RentShareAccount::try_from_slice(&rent_agreement_account.data.borrow());

        if rent_agreement_data.is_err() {
            msg!(
                "[RentShare] Rent agreement account data size incorrect: {}",
                rent_agreement_account.try_data_len()?
            );
            return Err(ProgramError::InvalidAccountData);
        }

        let mut rent_data = rent_agreement_data.unwrap();
        if !rent_data.is_initialized() {
            msg!("[RentShare] Rent agreement account not initialized");
            return Err(ProgramError::UninitializedAccount);
        }

        // Make sure we pay the same account used during the agreement initialization
        if rent_data.event_creator != *event_creator.key {
            msg!("[RentShare] Payee must match payee key used during agreement initialization");
            return Err(ProgramError::InvalidAccountData);
        }
        if rent_data.event_creator != event_creator {
            msg!(
                "[RentShare] Rent amount does not match agreement amount: {} vs {}",
                rent_data.rent_amount,
                rent_amount
            );
            return Err(RentShareError::RentPaymentAmountMismatch.into());
        }
        // rent_data.status = AgreementStatus::Active as u8;
        rent_data.event_id = event_id;
        // rent_data.payer_pubkey = payer_pubkey;
        // rent_data.rent_amount = rent_amount;
        rent_data.particpate_amount = particpate_amount;
        rent_data.event_creator = event_creator;
        // rent_data.fix_deposit_amount_per_person = fix_deposit_amount_per_person;
        // rent_data.total_partcipator = total_partcipator;

        // msg!(
        //     "[RentShare] Transfer {} lamports from payer with balance: {}",
        //     rent_amount,
        //     payer_account.lamports()
        // );

        // if rent_data.is_complete() {
        //     msg!("[RentShare] Rent already paid in full");
        //     return Err(RentShareError::RentAlreadyPaidInFull.into());
        // }

        // if rent_data.is_terminated() {
        //     msg!("[RentShare] Rent agreement already terminated");
        //     return Err(RentShareError::RentAgreementTerminated.into());
        // }

        // if rent_data.rent_amount != rent_amount {
        //     msg!(
        //         "[RentShare] Rent amount does not match agreement amount: {} vs {}",
        //         rent_data.rent_amount,
        //         rent_amount
        //     );
        //     return Err(RentShareError::RentPaymentAmountMismatch.into());
        // }

        // let instruction =
        //     system_instruction::transfer(payer_account.key, payee_account.key, rent_amount);

        // Invoke the system program to transfer funds
        // invoke(
        //     &instruction,
        //     &[
        //         system_program_account.clone(),
        //         payee_account.clone(),
        //         payer_account.clone(),
        //     ],
        // )?;

        // msg!(
        //     "[RentShare] Transfer completed. New payer balance: {}",
        //     payer_account.lamports()
        // );

        // Decrement the number of payment
        // rent_data.remaining_payments -= 1;
        // if rent_data.remaining_payments == 0 {
        //     rent_data.status = AgreementStatus::Completed as u8;
        // }
        rent_data.serialize(&mut &mut rent_agreement_account.data.borrow_mut()[..])?;

        Ok(())

    }
    fn pay_rent(accounts: &[AccountInfo], program_id: &Pubkey, rent_amount: u64) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let rent_agreement_account = next_account_info(accounts_iter)?;
        if rent_agreement_account.owner != program_id {
            msg!("[RentShare] Rent agreement account is not owned by this program");
            return Err(ProgramError::IncorrectProgramId);
        }

        let payee_account: &AccountInfo = next_account_info(accounts_iter)?;
        // let payer_account = next_account_info(accounts_iter)?;
        let system_program_account = next_account_info(accounts_iter)?;

        if !payer_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if payer_account.lamports() < rent_amount {
            return Err(ProgramError::InsufficientFunds);
        }

        // Transfer to self - do nothing
        if payer_account.key == payee_account.key {
            return Ok(());
        }

        // Initialize the Rent Agreement Account with the initial data
        // Note: the structure of the data state must match the `space` the client used to create the account
        let rent_agreement_data =
            RentShareAccount::try_from_slice(&rent_agreement_account.data.borrow());

        if rent_agreement_data.is_err() {
            msg!(
                "[RentShare] Rent agreement account data size incorrect: {}",
                rent_agreement_account.try_data_len()?
            );
            return Err(ProgramError::InvalidAccountData);
        }

        let mut rent_data = rent_agreement_data.unwrap();
        if !rent_data.is_initialized() {
            msg!("[RentShare] Rent agreement account not initialized");
            return Err(ProgramError::UninitializedAccount);
        }

        // Make sure we pay the same account used during the agreement initialization
        if rent_data.payee_pubkey != *payee_account.key {
            msg!("[RentShare] Payee must match payee key used during agreement initialization");
            return Err(ProgramError::InvalidAccountData);
        }

        msg!(
            "[RentShare] Transfer {} lamports from payer with balance: {}",
            rent_amount,
            payer_account.lamports()
        );

        if rent_data.is_complete() {
            msg!("[RentShare] Rent already paid in full");
            return Err(RentShareError::RentAlreadyPaidInFull.into());
        }

        if rent_data.is_terminated() {
            msg!("[RentShare] Rent agreement already terminated");
            return Err(RentShareError::RentAgreementTerminated.into());
        }

        if rent_data.rent_amount != rent_amount {
            msg!(
                "[RentShare] Rent amount does not match agreement amount: {} vs {}",
                rent_data.rent_amount,
                rent_amount
            );
            return Err(RentShareError::RentPaymentAmountMismatch.into());
        }

        let instruction =
            system_instruction::transfer(payer_account.key, payee_account.key, rent_amount);

        // Invoke the system program to transfer funds
        invoke(
            &instruction,
            &[
                system_program_account.clone(),
                payee_account.clone(),
                payer_account.clone(),
            ],
        )?;

        msg!(
            "[RentShare] Transfer completed. New payer balance: {}",
            payer_account.lamports()
        );

        // Decrement the number of payment
        rent_data.remaining_payments -= 1;
        if rent_data.remaining_payments == 0 {
            rent_data.status = AgreementStatus::Completed as u8;
        }
        rent_data.serialize(&mut &mut rent_agreement_account.data.borrow_mut()[..])?;

        Ok(())
    }

    // fn terminate_early(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    //     let accounts_iter = &mut accounts.iter();

    //     let rent_agreement_account = next_account_info(accounts_iter)?;
    //     if rent_agreement_account.owner != program_id {
    //         msg!("[RentShare] Rent agreement account is not owned by this program");
    //         return Err(ProgramError::IncorrectProgramId);
    //     }

    //     let rent_agreement_data =
    //         RentShareAccount::try_from_slice(&rent_agreement_account.data.borrow());

    //     if rent_agreement_data.is_err() {
    //         msg!(
    //             "[RentShare] Rent agreement account data size incorrect: {}",
    //             rent_agreement_account.try_data_len()?
    //         );
    //         return Err(ProgramError::InvalidAccountData);
    //     }

    //     let mut rent_data = rent_agreement_data.unwrap();
    //     if !rent_data.is_initialized() {
    //         msg!("[RentShare] Rent agreement account not initialized");
    //         return Err(ProgramError::UninitializedAccount);
    //     }

    //     if rent_data.is_complete() {
    //         msg!("[RentShare] Rent already paid in full");
    //         return Err(RentShareError::RentAlreadyPaidInFull.into());
    //     }

    //     if rent_data.is_terminated() {
    //         msg!("[RentShare] Rent agreement already terminated");
    //         return Err(RentShareError::RentAgreementTerminated.into());
    //     }

    //     rent_data.remaining_payments = 0;
    //     rent_data.status = AgreementStatus::Terminated as u8;
    //     rent_data.serialize(&mut &mut rent_agreement_account.data.borrow_mut()[..])?;

    //     Ok(())
    // }
}
