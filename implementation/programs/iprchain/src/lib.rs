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

    pub fn register_ip(ctx: Context<RegisterIp>, args: RegisterIpArgs) -> Result<()> {
        ctx.accounts.register(args, &ctx.bumps)?;
        Ok(())
    }

    pub fn create_license(ctx: Context<CreateLicense>, args: CreateLicenseArgs) -> Result<()> {
        ctx.accounts.create(args, &ctx.bumps)?;
        Ok(())
    }

    pub fn accept_license(ctx: Context<AcceptLicense>) -> Result<()> {
        ctx.accounts.accept_license()?;
        Ok(())
    }

    pub fn create_certificate(
        ctx: Context<CreateCertificate>,
        args: CertificateArgs,
    ) -> Result<()> {
        ctx.accounts.create_certificate(args)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        ctx.accounts.withdraw()
    }
}
