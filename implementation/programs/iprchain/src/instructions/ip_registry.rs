use anchor_lang::prelude::*;

use crate::{constants::{MAX_PLATFORM_FEE, MIN_PLATFORM_FEE}, errors::IPRChainErrorCode, state::IPRegistryState};

#[derive(Accounts)]
pub struct IPRegistry<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + IPRegistryState::INIT_SPACE,
        seeds = [b"iprchain", admin.key().as_ref()],
        bump,
    )]
    pub ip_registry: Account<'info, IPRegistryState>,
    #[account( 
        seeds = [b"treasury", ip_registry.key().as_ref()],
        bump,
    )]
    pub treasury: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl IPRegistry<'_> {
    pub fn init(&mut self, fee: u64, bumps: &IPRegistryBumps) -> Result<()> {
        require!((MIN_PLATFORM_FEE..=MAX_PLATFORM_FEE).contains(&fee), IPRChainErrorCode::InvalidPlatformFee);
        self.ip_registry.set_inner(IPRegistryState {
            admin: self.admin.key(),
            fee,
            bump: bumps.ip_registry,
            treasury_bump: bumps.treasury,
            total_ips: 0,
        });
        Ok(())
    }
}
