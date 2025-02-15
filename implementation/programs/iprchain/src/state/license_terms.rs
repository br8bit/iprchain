use anchor_lang::prelude::*;

use crate::errors::ErrorCode;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LicenseTerms {
    pub fee: u64,            // License fee in lamports
    pub expiration: i64,     // Unix timestamp
    pub royalty_percent: u8, // Percentage (0-100)
}

impl LicenseTerms {
    pub const LEN: usize = 8 + 8 + 1;

    pub fn validate(&self) -> Result<()> {
        if self.fee == 0 {
            return Err(ErrorCode::InvalidFee.into());
        }
        if self.expiration <= Clock::get()?.unix_timestamp {
            return Err(ErrorCode::InvalidExpiration.into());
        }
        if self.royalty_percent > 100 {
            return Err(ErrorCode::InvalidRoyaltyPercent.into());
        }
        Ok(())
    }
}
