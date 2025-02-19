use anchor_lang::prelude::*;
use anchor_spl::metadata::mpl_token_metadata::MAX_URI_LENGTH;

#[account]
pub struct IPAccount {
    pub creator: Pubkey,    // Creator wallet
    pub core_asset: Pubkey, // MPL Core NFT address
    pub ip_hash: [u8; 32],  // SHA-256 of IP content
    pub bump: u8,
    pub created_at: i64,
    pub metadata_uri: String, // Arweave/IPFS URI
}

impl Space for IPAccount {
    const INIT_SPACE: usize = 8 + 32 + 32 + 32 + 1 + 8 + (4 + MAX_URI_LENGTH);
}
