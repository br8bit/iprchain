use anchor_lang::prelude::*;

use crate::{errors::IPRChainErrorCode, state::{LicenseAccount, LicenseStatus}};

#[derive(Accounts)]
pub struct RevokeLicense<'info> {
    #[account(
        mut, 
        has_one = creator @ IPRChainErrorCode::Unauthorized
    )]
    pub license_account: Account<'info, LicenseAccount>,

    pub creator: Signer<'info>,
}

impl RevokeLicense<'_> {
    pub fn revoke(&mut self) -> Result<()> {
        self.license_account.status = LicenseStatus::Revoked;
        Ok(())
    }
}
