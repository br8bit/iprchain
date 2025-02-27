use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;

pub const IP_ACCOUNT_SEED: &[u8] = b"ip_account";
pub const IP_REGISTRY_SEED: &[u8] = b"iprchain";
pub const LICENSE_ACCOUNT_SEED: &[u8] = b"license";
pub const TREASURY_SEED: &[u8] = b"treasury";

pub const MIN_PLATFORM_FEE: u64 = LAMPORTS_PER_SOL / 100 * 3; // 0.03 SOL
