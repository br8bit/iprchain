use anchor_lang::prelude::*;

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum LicenseStatus {
    Pending,
    Active,
    Revoked,
    Expired,
}
