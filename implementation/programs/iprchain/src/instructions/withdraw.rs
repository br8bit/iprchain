use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{
    constants::{IP_REGISTRY_SEED, TREASURY_SEED},
    errors::IPRChainErrorCode,
    state::IPRegistryState,
};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    admin: Signer<'info>,

    #[account(
        seeds = [IP_REGISTRY_SEED, ip_registry.admin.key().as_ref()],
        bump=ip_registry.bump,
    )]
    ip_registry: Account<'info, IPRegistryState>,

    #[account(
        mut,
        seeds = [TREASURY_SEED, ip_registry.key().as_ref()],
        bump = ip_registry.treasury_bump
    )]
    treasury: SystemAccount<'info>,

    system_program: Program<'info, System>,
}

impl Withdraw<'_> {
    pub fn withdraw(&mut self) -> Result<()> {
        require!(
            self.admin.key() == self.ip_registry.admin,
            IPRChainErrorCode::Unauthorized
        );
        require!(
            self.treasury.lamports() > 0,
            IPRChainErrorCode::InvalidWithdrawalAmount
        );

        let seeds = &[
            TREASURY_SEED,
            &self.ip_registry.key().to_bytes()[..],
            &[self.ip_registry.treasury_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.treasury.to_account_info(),
                to: self.admin.to_account_info(),
            },
        )
        .with_signer(signer_seeds);

        transfer(cpi_ctx, self.treasury.lamports())?;

        Ok(())
    }
}
