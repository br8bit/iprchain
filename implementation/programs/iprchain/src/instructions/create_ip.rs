use anchor_lang::prelude::*;
use anchor_spl::metadata::mpl_token_metadata::MAX_URI_LENGTH;
use mpl_core::{accounts::BaseCollectionV1, instructions::CreateV2CpiBuilder, ID as MPL_CORE_ID};
    
use crate::{
    constants::{IP_ACCOUNT_SEED, IP_REGISTRY_SEED, TREASURY_SEED},
    errors::IPRChainErrorCode as ErrorCode,
    state::{IPAccount, IPRegistryState},
};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct IPCertificateArgs {
    pub name: String,
    pub uri: String,
}

#[derive(Accounts)]
#[instruction(ip_hash: [u8; 32], metadata_uri: String)]
pub struct CreateIp<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    pub admin: Signer<'info>,

    #[account(mut)]
    pub asset: Signer<'info>,
    
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

    // #[account(
    //     mut,
    //     seeds = [TREASURY_SEED, ip_registry.key().as_ref()],
    //     bump = ip_registry.treasury_bump,
    // )]
    // pub treasury: SystemAccount<'info>,
    
    #[account(mut)]
    pub collection: Option<Account<'info, BaseCollectionV1>>,

    pub authority: Option<Signer<'info>>,

    /// CHECK: this account will be checked by the mpl_core program
    pub update_authority: Option<UncheckedAccount<'info>>,

    #[account(address = MPL_CORE_ID)]
    /// CHECK: this account is checked by the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

impl CreateIp<'_> {
    pub fn create(&mut self, ip_hash: [u8; 32], metadata_uri: String, bumps: &CreateIpBumps) -> Result<()> {
        require!(self.ip_account.ip_hash != ip_hash, ErrorCode::DuplicateIpHash);
        require!(!metadata_uri.is_empty() && metadata_uri.len() <= MAX_URI_LENGTH, ErrorCode::InvalidMetadataUriLength);

        let _ = self.create_certificate(IPCertificateArgs {
            name: "IPCertificate".to_string(),
            uri: metadata_uri.clone(),
        });

        self.ip_account.set_inner(IPAccount {
            creator: self.creator.key(),
            core_asset: self.asset.key(),
            ip_hash,
            bump: bumps.ip_account,
            created_at: Clock::get()?.unix_timestamp,
            metadata_uri,
        });

        self.ip_registry.total_ips += 1;

        Ok(())
    }

    fn create_certificate(&mut self, args: IPCertificateArgs) -> Result<()> {
        let collection = &self.collection.as_ref().map(|c| c.to_account_info()); 

        let authority = &self.authority.as_ref().map(|a| a.to_account_info());

        let update_authority = &self.update_authority.as_ref().map(|ua| ua.to_account_info());

        CreateV2CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(collection.as_ref())
            .authority(authority.as_ref())
            .payer(&self.creator.to_account_info())
            .owner(Some(self.creator.as_ref()))
            .update_authority(update_authority.as_ref())
            .system_program(&self.system_program.to_account_info())
            .name(args.name)
            .uri(args.uri)
            .invoke()?;

        Ok(())
    }
}
