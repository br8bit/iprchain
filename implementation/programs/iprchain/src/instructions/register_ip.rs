use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
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

    #[account(
        mut,
        seeds = [IP_REGISTRY_SEED, ip_registry.admin.key().as_ref()],
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

    #[account( 
        mut,
        seeds = [TREASURY_SEED, ip_registry.key().as_ref()],
        bump = ip_registry.treasury_bump,
    )]
    pub treasury: SystemAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

impl RegisterIp<'_> {
    pub fn register(&mut self, args: RegisterIpArgs, bumps: &RegisterIpBumps) -> Result<()> {
        let RegisterIpArgs {
            ip_hash,
            metadata_uri,
        } = &args;

        require!(!ip_hash.is_empty(), ErrorCode::InvalidIpHash);
        require!(self.ip_account.ip_hash != *ip_hash, ErrorCode::DuplicateIpHash);
        require!(!metadata_uri.is_empty() && metadata_uri.len() <= MAX_URI_LENGTH, ErrorCode::InvalidMetadataUriLength);

        self.pay_fees()?;
        self.init(args, bumps)?;
        self.ip_registry.increment_total_ips();

        Ok(())
    }

    fn init(&mut self, args: RegisterIpArgs, bumps: &RegisterIpBumps) -> Result<()> {
        self.ip_account.set_inner(IPAccount {
            creator: self.creator.key(),
            core_asset: None,
            ip_hash: args.ip_hash,
            bump: bumps.ip_account,
            created_at: Clock::get()?.unix_timestamp,
            metadata_uri: args.metadata_uri,
        });

        Ok(())
    }

    fn pay_fees(&mut self) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.creator.to_account_info(),
                to: self.treasury.to_account_info(),
            }
        );
        transfer(cpi_ctx, self.ip_registry.fee)?;

        Ok(())
    }
}
