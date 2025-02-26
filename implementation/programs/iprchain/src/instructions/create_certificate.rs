use anchor_lang::prelude::*;
use mpl_core::{accounts::BaseCollectionV1, instructions::CreateV2CpiBuilder, ID as MPL_CORE_ID};

use crate::{constants::IP_ACCOUNT_SEED, state::IPAccount};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CertificateArgs {
    pub name: String,
    pub uri: String,
}

#[derive(Accounts)]
pub struct CreateCertificate<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(mut)]
    pub asset: Signer<'info>,

    #[account(
        mut,
        seeds = [IP_ACCOUNT_SEED, ip_account.ip_hash.as_ref()],
        bump = ip_account.bump,
    )]
    pub ip_account: Account<'info, IPAccount>,

    #[account(mut)]
    pub collection: Option<Account<'info, BaseCollectionV1>>,

    /// CHECK: this account will be checked by the mpl_core program
    pub owner: Option<UncheckedAccount<'info>>,

    pub authority: Option<Signer<'info>>,

    /// CHECK: this account will be checked by the mpl_core program
    pub update_authority: Option<UncheckedAccount<'info>>,

    #[account(address = MPL_CORE_ID)]
    /// CHECK: this account is checked by the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateCertificate<'info> {
    pub fn create_certificate(&mut self, args: CertificateArgs) -> Result<()> {
        let collection = &self.collection.as_ref().map(|c| c.to_account_info());

        // let authority = &self.authority.as_ref().map(|a| a.to_account_info());

        let update_authority = &self
            .update_authority
            .as_ref()
            .map(|ua| ua.to_account_info());

        CreateV2CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(collection.as_ref())
            .payer(&self.creator.to_account_info())
            .authority(Some(&self.creator.to_account_info()))
            .owner(Some(&self.creator.to_account_info()))
            .update_authority(update_authority.as_ref())
            .system_program(&self.system_program.to_account_info())
            .name(args.name)
            .uri(args.uri)
            .invoke()?;

        self.ip_account.core_asset = Some(self.asset.key());

        Ok(())
    }
}
