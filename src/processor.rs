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
    state::{EventStatus, InitEvent},
};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = Instruction::unpack(instruction_data)?;

        match instruction {
            Instruction::InitializeEvent {
                payee_pubkey, // reveed the money.
                event_creator,
                event_id,
                fix_deposit_amount_per_person,
                total_partcipator
            } => Self::initialize_event(
                accounts,
                program_id,
                payee_pubkey,
                event_creator,
                event_id,
                fix_deposit_amount_per_person,
                total_partcipator
            ),
            Instruction::CancelEvent {
                event_creator,
                event_id
            } => Self::cancel_event(
                accounts,
                program_id,
                event_creator
                event_id,
            ),
            Instruction::StartEvent {                
                event_id,
                event_creator
            } => Self::start_event(
                accounts,
                program_id,
                event_id,
                event_creator
            ),
            Instruction::EndEvent {                
                event_id,
                event_creator
            } => Self::end_event(
                accounts,
                program_id,
                event_id,
                event_creator
            ),
            Instruction::ParticipateInEvent {
                event_id,
                particpate_amount,
                event_creator
            } => Self::particpate_in_event(
                accounts,
                program_id,
                event_id,
                particpate_amount,
                event_creator
            ),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn initialize_event(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
        payee_pubkey: Pubkey, // the party receiving the payment is known as the payee.
        event_creator:Pubkey,
        event_id: u64,
        fix_deposit_amount_per_person: u64,
        total_partcipator: u64,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let solana_data_account = next_account_info(accounts_iter)?;
        if solana_data_account.owner != program_id {
            msg!(" Event account not owned by this program");
            return Err(ProgramError::IncorrectProgramId);
        }

        
        // Initialize the Rent Agreement Account with the initial data
        // Note: the structure of the data state must match the `space` reserved when account created
        let solana_data_account_data =
            InitEvent::try_from_slice(&solana_data_account.data.borrow());

        if solana_data_account_data.is_err() {
            msg!(
                "[RentShare] Rent agreement account data size incorrect: {}",
                solana_data_account.try_data_len()?
            );
            return Err(ProgramError::InvalidAccountData);
        }

        let mut data_of_solana_ac = solana_data_account_data.unwrap();
        if data_of_solana_ac.is_initialized() {
            msg!("Event is already initialized");
            return Err(ProgramError::AccountAlreadyInitialized);
        }


        data_of_solana_ac.status = EventStatus::Active as u8;
        data_of_solana_ac.payee_pubkey = payee_pubkey;
        data_of_solana_ac.event_creator = event_creator;
        data_of_solana_ac.event_id = event_id;
        data_of_solana_ac.fix_deposit_amount_per_person = fix_deposit_amount_per_person;
        data_of_solana_ac.total_partcipator = total_partcipator;
        data_of_solana_ac.serialize(&mut &mut solana_data_account.data.borrow_mut()[..])?;

        msg!(
            "Initialized event account: {:?}",
            data_of_solana_ac
        );

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

    //     let solana_data_account = next_account_info(accounts_iter)?;
    //     if solana_data_account.owner != program_id {
    //         msg!("[RentShare] Rent agreement account is not owned by this program");
    //         return Err(ProgramError::IncorrectProgramId);
    //     }

    //     // let payee_account: &AccountInfo = next_account_info(accounts_iter)?;
    //     // let payer_account = next_account_info(accounts_iter)?;
    //     let system_program_account = next_account_info(accounts_iter)?;

    //     if !event_creator.is_signer {
    //         return Err(ProgramError::MissingRequiredSignature);
    //     }
    //     let solana_data_account_data =
    //     InitEvent::try_from_slice(&solana_data_account.data.borrow());

    // if solana_data_account_data.is_err() {
    //     msg!(
    //         "[RentShare] Rent agreement account data size incorrect: {}",
    //         solana_data_account.try_data_len()?
    //     );
    //     return Err(ProgramError::InvalidAccountData);
    // }

    // let mut data_of_solana_ac = solana_data_account_data.unwrap();
    // // EventStatus::Active
    // //************************************************************88
    // if data_of_solana_ac.is_initialized() {
    //     msg!("[RentShare] Rent agreement already initialized");
    //     return Err(ProgramError::AccountAlreadyInitialized);
    // }
    // //*************************************************************

    // if data_of_solana_ac.event_creator != event_creator {
    //     msg!(
    //         "[RentShare] Rent amount does not match agreement amount: {} vs {}",
    //         data_of_solana_ac.rent_amount,
    //         rent_amount
    //     );
    //     return Err(RentShareError::RentPaymentAmountMismatch.into());
    // }

    // if data_of_solana_ac.event_id != event_id {
    //     msg!(
    //         "[RentShare] Rent amount does not match agreement amount: {} vs {}",
    //         data_of_solana_ac.rent_amount,
    //         rent_amount
    //     );
    //     return Err(RentShareError::RentPaymentAmountMismatch.into());
    // }
    // data_of_solana_ac.status = EventStatus::Completed as u8;
    // data_of_solana_ac.status = EventStatus::Active as u8;
    // data_of_solana_ac.payee_pubkey = payee_pubkey;
    // // data_of_solana_ac.payer_pubkey = payer_pubkey;
    // // data_of_solana_ac.rent_amount = rent_amount;
    // data_of_solana_ac.event_creator = event_creator;
    // data_of_solana_ac.event_id = event_id;
    // data_of_solana_ac.fix_deposit_amount_per_person = fix_deposit_amount_per_person;
    // data_of_solana_ac.total_partcipator = total_partcipator;
    // // data_of_solana_ac.remaining_payments = duration;
    // // data_of_solana_ac.serialize(&mut &mut solana_data_account.data.borrow_mut()[..])?;

    // msg!(
    //     "[RentShare] Initialized rent agreement account: {:?}",
    //     data_of_solana_ac
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

        let solana_data_account = next_account_info(accounts_iter)?;
        if solana_data_account.owner != program_id {
            msg!("Account is not owned by this program");
            return Err(ProgramError::IncorrectProgramId);
        }

     
        if !event_creator.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

     
        // Note: the structure of the data state must match the `space` the client used to create the account
        let solana_data_account_data =
            StartEvent::try_from_slice(&solana_data_account.data.borrow());

        if solana_data_account_data.is_err() {
            msg!(
                "account data size incorrect: {}",
                solana_data_account.try_data_len()?
            );
            return Err(ProgramError::InvalidAccountData);
        }

        let mut data_of_solana_ac = solana_data_account_data.unwrap();
       //****************doubt it will run or not **********
        if !data_of_solana_ac.is_initialized() {
            msg!("[RentShare] Rent agreement account not initialized");
            return Err(ProgramError::UninitializedAccount);
        }
//**********************************************************
        // Make sure we pay the same account used during the agreement initialization
        if data_of_solana_ac.event_creator != *event_creator.key {
            msg!("This is not the event creator.");
            return Err(ProgramError::InvalidAccountData);
        }
        if data_of_solana_ac.event_creator != event_creator {
            msg!(
                "This is not the event creator.",
                data_of_solana_ac.event_creator,
                event_creator
            );
            return Err(RentShareError::RentPaymentAmountMismatch.into());
        }
        data_of_solana_ac.event_id = event_id;
        data_of_solana_ac.particpate_amount = particpate_amount;
        data_of_solana_ac.event_creator = event_creator;
        
        data_of_solana_ac.serialize(&mut &mut solana_data_account.data.borrow_mut()[..])?;

        Ok(())

    }
    fn pay_rent(accounts: &[AccountInfo], program_id: &Pubkey, rent_amount: u64) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let solana_data_account = next_account_info(accounts_iter)?;
        if solana_data_account.owner != program_id {
            msg!("[RentShare] Rent agreement account is not owned by this program");
            return Err(ProgramError::IncorrectProgramId);
        }

        let payee_account: &AccountInfo = next_account_info(accounts_iter)?;
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
        let solana_data_account_data =
            InitEvent::try_from_slice(&solana_data_account.data.borrow());

        if solana_data_account_data.is_err() {
            msg!(
                "[RentShare] Rent agreement account data size incorrect: {}",
                solana_data_account.try_data_len()?
            );
            return Err(ProgramError::InvalidAccountData);
        }

        let mut data_of_solana_ac = solana_data_account_data.unwrap();
        if !data_of_solana_ac.is_initialized() {
            msg!("[RentShare] Rent agreement account not initialized");
            return Err(ProgramError::UninitializedAccount);
        }

        // Make sure we pay the same account used during the agreement initialization
        if data_of_solana_ac.payee_pubkey != *payee_account.key {
            msg!("[RentShare] Payee must match payee key used during agreement initialization");
            return Err(ProgramError::InvalidAccountData);
        }

        msg!(
            "[RentShare] Transfer {} lamports from payer with balance: {}",
            rent_amount,
            payer_account.lamports()
        );

        if data_of_solana_ac.is_complete() {
            msg!("[RentShare] Rent already paid in full");
            return Err(RentShareError::RentAlreadyPaidInFull.into());
        }

        if data_of_solana_ac.is_terminated() {
            msg!("[RentShare] Rent agreement already terminated");
            return Err(RentShareError::RentAgreementTerminated.into());
        }

        if data_of_solana_ac.rent_amount != rent_amount {
            msg!(
                "[RentShare] Rent amount does not match agreement amount: {} vs {}",
                data_of_solana_ac.rent_amount,
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
        data_of_solana_ac.remaining_payments -= 1;
        if data_of_solana_ac.remaining_payments == 0 {
            data_of_solana_ac.status = EventStatus::Completed as u8;
        }
        data_of_solana_ac.serialize(&mut &mut solana_data_account.data.borrow_mut()[..])?;

        Ok(())
    }

    // fn terminate_early(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    //     let accounts_iter = &mut accounts.iter();

    //     let solana_data_account = next_account_info(accounts_iter)?;
    //     if solana_data_account.owner != program_id {
    //         msg!("[RentShare] Rent agreement account is not owned by this program");
    //         return Err(ProgramError::IncorrectProgramId);
    //     }

    //     let solana_data_account_data =
    //         InitEvent::try_from_slice(&solana_data_account.data.borrow());

    //     if solana_data_account_data.is_err() {
    //         msg!(
    //             "[RentShare] Rent agreement account data size incorrect: {}",
    //             solana_data_account.try_data_len()?
    //         );
    //         return Err(ProgramError::InvalidAccountData);
    //     }

    //     let mut data_of_solana_ac = solana_data_account_data.unwrap();
    //     if !data_of_solana_ac.is_initialized() {
    //         msg!("[RentShare] Rent agreement account not initialized");
    //         return Err(ProgramError::UninitializedAccount);
    //     }

    //     if data_of_solana_ac.is_complete() {
    //         msg!("[RentShare] Rent already paid in full");
    //         return Err(RentShareError::RentAlreadyPaidInFull.into());
    //     }

    //     if data_of_solana_ac.is_terminated() {
    //         msg!("[RentShare] Rent agreement already terminated");
    //         return Err(RentShareError::RentAgreementTerminated.into());
    //     }

    //     data_of_solana_ac.remaining_payments = 0;
    //     data_of_solana_ac.status = EventStatus::Terminated as u8;
    //     data_of_solana_ac.serialize(&mut &mut solana_data_account.data.borrow_mut()[..])?;

    //     Ok(())
    // }
}
