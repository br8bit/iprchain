use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LicenseTerms {
    pub fee: u64,            // License fee in lamports
    pub expiration: i64,     // Unix timestamp
    pub royalty_percent: u8, // Percentage (0-100)
}
