use anchor_lang::prelude::*;

#[account]
pub struct IpAccount {
    pub owner: Pubkey,     // Current owner (NFT holder)
    pub mint: Pubkey,      // NFT mint address
    pub ip_hash: [u8; 32], // SHA-256 of IP content
    pub created_at: i64,
    pub metadata_uri: String, // Arweave/IPFS URI
}
