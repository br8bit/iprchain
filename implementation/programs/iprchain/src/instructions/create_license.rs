use anchor_lang::prelude::*;

use crate::{
    constants::{IP_ACCOUNT_SEED, LICENSE_ACCOUNT_SEED},
    state::{IPAccount, LicenseAccount, LicenseStatus, LicenseTerms}
};


#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateLicenseArgs {
    pub fee: u64,
    pub starts_at: i64,
    pub expires_at: i64,
    pub royalty_percent: u8,
}

#[derive(Accounts)]
pub struct CreateLicense<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        seeds = [IP_ACCOUNT_SEED, ip_account.ip_hash.as_ref()],
        bump = ip_account.bump,
    )]
    pub ip_account: Account<'info, IPAccount>,

    #[account(
        init,
        payer = creator,
        space = 8 + LicenseAccount::INIT_SPACE,
        seeds = [LICENSE_ACCOUNT_SEED, creator.key.as_ref(), ip_account.ip_hash.as_ref()], 
        bump,
    )]
    pub license_account: Account<'info, LicenseAccount>,

    pub system_program: Program<'info, System>,
}

impl CreateLicense<'_> {
    pub fn create(
        &mut self,
        args: CreateLicenseArgs,
        bumps: &CreateLicenseBumps
    ) -> Result<()> {
        let CreateLicenseArgs {
            fee,
            starts_at,
            expires_at,
            royalty_percent,
        } = args;
        
        let terms  = LicenseTerms {
            fee,
            starts_at,
            expires_at,
            royalty_percent,
        };
        
        terms.validate()?;

        self.license_account.set_inner(LicenseAccount {
            creator: self.creator.key(),
            ip_account: self.ip_account.key(),
            bump: bumps.license_account,
            licensee: None,
            terms,
            status: LicenseStatus::Active,
            total_royalties_paid: 0,
            last_payment_date: None,
        });
        Ok(())
    }
}