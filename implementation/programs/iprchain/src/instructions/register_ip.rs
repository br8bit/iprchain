use anchor_lang::prelude::*;
use anchor_spl::metadata::mpl_token_metadata::MAX_URI_LENGTH;
    
use crate::{
    constants::{IP_ACCOUNT_SEED, IP_REGISTRY_SEED, TREASURY_SEED},
    errors::IPRChainErrorCode as ErrorCode,
    state::{IPAccount, IPRegistryState},
};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct RegisterIpArgs {
    pub ip_hash: [u8; 32],
    pub metadata_uri: String,
}

#[derive(Accounts)]
#[instruction(ip_hash: [u8; 32], metadata_uri: String)]
pub struct RegisterIp<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    pub admin: Signer<'info>,
    
    #[account(
        mut,
        seeds = [IP_REGISTRY_SEED, admin.key().as_ref()],
        bump = ip_registry.bump,
    )]
    pub ip_registry: Account<'info, IPRegistryState>,

    #[account(
        init_if_needed,
        payer = creator,
        space = IPAccount::INIT_SPACE,
        seeds = [IP_ACCOUNT_SEED, ip_hash.as_ref()], 
        bump,
    )]
    pub ip_account: Account<'info, IPAccount>,
    
    pub system_program: Program<'info, System>,
}

impl RegisterIp<'_> {
    pub fn register(&mut self, args: RegisterIpArgs, bumps: &RegisterIpBumps) -> Result<()> {
        let RegisterIpArgs {
            ip_hash,
            metadata_uri,
        } = args;

        require!(!ip_hash.is_empty(), ErrorCode::InvalidIpHash);
        require!(self.ip_account.ip_hash != ip_hash, ErrorCode::DuplicateIpHash);
        require!(!metadata_uri.is_empty() && metadata_uri.len() <= MAX_URI_LENGTH, ErrorCode::InvalidMetadataUriLength);

        self.ip_account.set_inner(IPAccount {
            creator: self.creator.key(),
            core_asset: None,
            ip_hash,
            bump: bumps.ip_account,
            created_at: Clock::get()?.unix_timestamp,
            metadata_uri,
        });

        self.ip_registry.total_ips += 1;

        Ok(())
    }
}
