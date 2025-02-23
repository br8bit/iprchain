use anchor_lang::prelude::*;

use crate::{
    constants::{IP_ACCOUNT_SEED, LICENSE_ACCOUNT_SEED},
    errors::IPRChainErrorCode,
    state::{IPAccount, LicenseAccount, LicenseStatus},
};

#[derive(Accounts)]
pub struct AssignLicensee<'info> {
    #[account(
        mut,
        seeds = [LICENSE_ACCOUNT_SEED, license_account.creator.as_ref(), ip_account.ip_hash.as_ref()],
        bump = license_account.bump,
    )]
    pub license_account: Account<'info, LicenseAccount>,

    #[account(
        seeds = [IP_ACCOUNT_SEED, ip_account.ip_hash.as_ref()],
        bump = ip_account.bump,
    )]
    pub ip_account: Account<'info, IPAccount>,

    #[account(mut)]
    pub licensee: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl AssignLicensee<'_> {
    pub fn assign(&mut self) -> Result<()> {
        require!(
            self.license_account.licensee.is_none(),
            IPRChainErrorCode::LicenseeAlreadyAssigned,
        );
        require!(
            self.license_account.status == LicenseStatus::Active,
            IPRChainErrorCode::LicenseNotActive
        );

        self.license_account.licensee = Some(self.licensee.key());
        Ok(())
    }
}
