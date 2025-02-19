use anchor_lang::prelude::*;

declare_id!("BAbpq4BbvfHmgwHGDY4zxWBUBc2osE9f3vKvr6uGnKN");

mod constants;
mod errors;
mod instructions;
mod state;

use instructions::*;

#[program]
pub mod iprchain {
    use super::*;

    pub fn initialize(ctx: Context<IPRegistry>) -> Result<()> {
        ctx.accounts.init(0, &ctx.bumps)?;
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
