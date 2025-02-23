use anchor_lang::prelude::*;

declare_id!("CoADrt4GSwVVujxn1HuqKMTYVVyrUqqYLG4obi68EpAh");

mod constants;
mod errors;
mod instructions;
mod state;

use instructions::*;

#[program]
pub mod iprchain {
    use super::*;

    pub fn initialize(ctx: Context<IPRegistry>, fee: u64) -> Result<()> {
        ctx.accounts.init(fee, &ctx.bumps)?;
        Ok(())
    }

    pub fn register_ip(
        ctx: Context<CreateIp>,
        ip_hash: [u8; 32],
        metadata_uri: String,
    ) -> Result<()> {
        ctx.accounts.create(ip_hash, metadata_uri, &ctx.bumps)?;
        Ok(())
    }

    pub fn create_license(
        ctx: Context<CreateLicense>,
        fee: u64,
        starts_at: i64,
        expires_at: i64,
        royalty_percent: u8,
    ) -> Result<()> {
        ctx.accounts
            .create(fee, starts_at, expires_at, royalty_percent, &ctx.bumps)?;
        Ok(())
    }

    pub fn assign_licensee(ctx: Context<AssignLicensee>) -> Result<()> {
        ctx.accounts.assign()?;
        Ok(())
    }
}
