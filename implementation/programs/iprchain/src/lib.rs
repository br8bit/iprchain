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
}
