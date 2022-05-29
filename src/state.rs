use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_pack::{IsInitialized, Sealed},
    pubkey::Pubkey,
};
// #[derive(BorshSerialize, BorshDeserialize, Debug)]
// pub struct InitlializeEvent{
//     pub event_id:u64,
//     pub event_struct:RentShareAccount
// }
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct StartEvent{
    pub event_id: u8,
    pub particpate_amount: u64,
    pub event_creator:pubkey

}

/// Rent Share Account state stored in the Agreement Account
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct RentShareAccount {
    pub status: u8,
    pub payee_pubkey: Pubkey,
    // pub payer_pubkey: Pubkey,
    pub event_id: u64,
    pub event_creator:Pubkey,

    pub fix_deposit_amount_per_person: u64,
    pub total_partcipator: u64,
    // pub duration_unit: u8,
    // pub remaining_payments: u64,
}

impl Sealed for RentShareAccount {}


impl IsInitialized for RentShareAccount {
    fn is_initialized(&self) -> bool {
        self.status != AgreementStatus::Uninitialized as u8
    }
}

impl RentShareAccount {
    pub fn is_complete(&self) -> bool {
        self.status == AgreementStatus::Completed as u8
    }

    pub fn is_terminated(&self) -> bool {
        self.status == AgreementStatus::Terminated as u8
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Duration {
    Months = 0,
}

#[derive(Copy, Clone)]
pub enum AgreementStatus {
    Uninitialized = 0,
    Active,
    Completed,
    Terminated,
}
