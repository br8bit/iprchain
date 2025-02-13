use anchor_lang::prelude::*;

use super::{LicenseStatus, LicenseTerms};

#[account]
pub struct LicenseAccount {
    pub ip_account: Pubkey,
    pub licensee: Pubkey,
    pub terms: LicenseTerms,
    pub status: LicenseStatus,
    pub total_royalties_paid: u64,
    pub last_payment_date: i64,
}
