use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid mint decimals - must be 0 for NFTs")]
    InvalidMintDecimals,
    #[msg("Invalid mint supply - must be 1 for NFTs")]
    InvalidMintSupply,
    #[msg("Duplicate IP hash detected")]
    DuplicateIpHash,
    #[msg("Invalid license expiration time")]
    InvalidExpiration,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("IP account owner mismatch")]
    InvalidIpOwner,
    #[msg("License is not active")]
    LicenseNotActive,
    #[msg("Licensee does not match license account")]
    LicenseeMismatch,
    #[msg("Royalty distribution failed")]
    RoyaltyDistributionFailed,
    #[msg("Invalid royalty percent")]
    InvalidRoyaltyPercent,
    #[msg("Invalid fee")]
    InvalidFee,
    #[msg("Invalid license terms")]
    InvalidLicenseTerms,
    #[msg("Invalid license status")]
    InvalidLicenseStatus,
}
