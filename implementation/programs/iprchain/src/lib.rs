use anchor_lang::prelude::*;

declare_id!("BAbpq4BbvfHmgwHGDY4zxWBUBc2osE9f3vKvr6uGnKN");

#[program]
pub mod iprchain {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
