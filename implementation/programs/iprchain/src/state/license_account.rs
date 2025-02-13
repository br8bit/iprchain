use anchor_lang::prelude::*;

use super::{LicenseStatus, LicenseTerms};

#[account]
pub struct LicenseAccount {
    pub ip_account: Pubkey,
    pub licensee: Pubkey,
    pub terms: LicenseTerms,
    pub status: LicenseStatus,
}
