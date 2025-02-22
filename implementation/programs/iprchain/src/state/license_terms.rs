use anchor_lang::prelude::*;

use crate::errors::IPRChainErrorCode;

#[account]
#[derive(InitSpace)]
pub struct LicenseTerms {
    pub starts_at: i64,      // Unix timestamp
    pub expires_at: i64,     // Unix timestamp
    pub fee: u64,            // License fee in lamports
    pub royalty_percent: u8, // Percentage (0-100)
}

impl LicenseTerms {
    pub fn validate(&self) -> Result<()> {
        if self.starts_at < Clock::get()?.unix_timestamp {
            return Err(IPRChainErrorCode::InvalidStartDate.into());
        }
        if self.expires_at <= Clock::get()?.unix_timestamp {
            return Err(IPRChainErrorCode::InvalidExpiration.into());
        }
        if self.royalty_percent > 100 {
            return Err(IPRChainErrorCode::InvalidRoyaltyPercent.into());
        }
        Ok(())
    }
}
