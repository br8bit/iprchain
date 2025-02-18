use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct IPRegistryState {
    pub admin: Pubkey,
    pub fee: u64,
    pub bump: u8,
    pub treasury_bump: u8,
    pub total_ips: u64,
}
