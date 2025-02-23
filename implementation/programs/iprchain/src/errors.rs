use anchor_lang::error_code;

#[error_code]
pub enum IPRChainErrorCode {
    #[msg("Invalid mint decimals - must be 0 for NFTs")]
    InvalidMintDecimals,
    #[msg("Invalid mint supply - must be 1 for NFTs")]
    InvalidMintSupply,
    #[msg("Duplicate IP hash detected")]
    DuplicateIpHash,
    #[msg("Invalid start time")]
    InvalidStartDate,
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
    #[msg("Licensee already assigned")]
    LicenseeAlreadyAssigned,
    #[msg("Licensee not assigned")]
    LicenseeNotAssigned,
    #[msg("Licensee not found")]
    LicenseeNotFound,
    #[msg("Licensee not authorized")]
    LicenseeNotAuthorized,
    #[msg("Royalty distribution failed")]
    RoyaltyDistributionFailed,
    #[msg("Invalid royalty percent")]
    InvalidRoyaltyPercent,
    #[msg("Platform fee should be within the range of 1% to 2%")]
    InvalidPlatformFee,
    #[msg("Invalid license terms")]
    InvalidLicenseTerms,
    #[msg("Invalid license status")]
    InvalidLicenseStatus,
    #[msg("Invalid license type")]
    InvalidLicenseType,
    #[msg("Invalid license account")]
    InvalidLicenseAccount,
    #[msg("Invalid license fee")]
    InvalidLicenseFee,
    #[msg("Invalid collection mint")]
    InvalidCollectionMint,
    #[msg("Invalid metadata uri length")]
    InvalidMetadataUriLength,
}
