use anchor_lang::prelude::*;

use super::{LicenseStatus, LicenseTerms};

#[account]
#[derive(InitSpace)]
pub struct LicenseAccount {
    pub creator: Pubkey,
    pub ip_account: Pubkey,
    pub bump: u8,
    pub licensee: Option<Pubkey>,
    pub terms: LicenseTerms,
    pub status: LicenseStatus,
    pub total_royalties_paid: u64,
    pub last_payment_date: Option<i64>,
}
