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

impl IPRegistryState {
    pub fn get_platform_fee(&self) -> u64 {
        self.fee
    }

    pub fn increment_total_ips(&mut self) {
        self.total_ips += 1;
    }
}
