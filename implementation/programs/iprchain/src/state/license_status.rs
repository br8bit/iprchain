use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum LicenseStatus {
    Active,
    Revoked,
    Expired,
}
