use anchor_lang::prelude::*;

use crate::errors::IPRChainErrorCode;

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
            return Err(IPRChainErrorCode::InvalidLicenseFee.into());
        }
        if self.expiration <= Clock::get()?.unix_timestamp {
            return Err(IPRChainErrorCode::InvalidExpiration.into());
        }
        if self.royalty_percent > 100 {
            return Err(IPRChainErrorCode::InvalidRoyaltyPercent.into());
        }
        Ok(())
    }
}
