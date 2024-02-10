use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{pubkey, sysvar};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token::{Mint, Token},
};
use mpl_token_metadata::{
    instructions::{
        CreateV1CpiBuilder, MintV1CpiBuilder, UpdateV1CpiBuilder, VerifyCollectionV1CpiBuilder,
    },
    types,
};

declare_id!("powCFRgLT5dRUdMXm4cBoajxM3S9gAcc54uvPrEwTcs");

const NFT_UPDATE_AUTH: Pubkey = pubkey!("PUPTaF8u37PFBK4d5cRi5vnjfuL6fMZ4y6Xofen8NWD");
const ROYALTIES: Pubkey = pubkey!("PRLdyE6EAVhj2KUJ9ScttLoRvuwgFqWDxTv3qjZhmjJ");
const COLLECTION: Pubkey = pubkey!("PcoL2azniJHzRGjGMpj8PhxSwuFtb7QqxDVHC5xs7uL");

const CREATOR_SEED: &[u8] = b"CREATOR";
const REGISTER_SEED: &[u8] = b"REGISTER";

const METAPLEX_DEFAULT_RULES: Pubkey = pubkey!("eBJLFYPxJmMGKuFwpDWkzxZeUrad92kZRC5BJLpzyT9");

#[program]
pub mod pow {
    use super::*;

    pub fn mint(ctx: Context<Initialize>) -> Result<()> {
        let id = extract_mint_id(&ctx.accounts.mint.key()).unwrap();
        let tier = id.to_string().len() as u8;

        let register = &mut ctx.accounts.register;
        register.id = id;
        register.mint = ctx.accounts.mint.key();
        register.tier = tier;

        if tier == 5 || tier == 6 || tier == 7 || tier == 8 {
            panic!();
        }

        let metadata = match tier {
            1 => "VE87I01Vw5Fw-R-OkB7DQexJEfBXGDp9GiX-OltfQwo",
            2 => "Wskg0eK--pzYK4P9V5e3zSXTBrG3huN0adFYgn_xY5M",
            3 => "PMwLnBzGaZE0GCbQ1v5wxgbo1dyWBGPYMe4zY6ANWQI",
            4 => "-XJ76ysuhGVXly3nULQseKUIrRHT3XY_lbc6YNtUvKs",
            5 => "XRe-xit7mUPK-33yifZNDgKP8VgRe_gLAmk61f648kE",
            6 => "-kGVweWLthx4h2kXOr38E8x2k2xqQ_m4xEpzaXOP5hE",
            _ => "mMgT9ZGv30A9TgK9CaAnneX3CssfLrpp3zNoe89icig",
        };

        CreateV1CpiBuilder::new(&ctx.accounts.token_metadata_program)
            .mint(&ctx.accounts.mint, true)
            .name(format!("POW #{}", id))
            .uri(format!("https://arweave.net/{}", metadata))
            .symbol("POW".to_string())
            .collection(mpl_token_metadata::types::Collection {
                key: ctx.accounts.collection_mint.key(),
                verified: false,
            })
            .creators(vec![
                mpl_token_metadata::types::Creator {
                    address: ROYALTIES,
                    verified: false,
                    share: 100,
                },
                mpl_token_metadata::types::Creator {
                    address: ctx.accounts.mint_authority.key(),
                    verified: true,
                    share: 0,
                },
            ])
            .metadata(&ctx.accounts.mint_metadata)
            .master_edition(Some(&ctx.accounts.mint_master_edition))
            .payer(&ctx.accounts.signer)
            .update_authority(&ctx.accounts.mint_authority, true)
            .authority(&ctx.accounts.mint_authority)
            .token_standard(types::TokenStandard::ProgrammableNonFungible)
            .rule_set(ctx.accounts.rule_set.key())
            .print_supply(mpl_token_metadata::types::PrintSupply::Zero)
            .is_mutable(true)
            .system_program(&ctx.accounts.system_program)
            .sysvar_instructions(&ctx.accounts.sysvar_ixs)
            .spl_token_program(Some(&ctx.accounts.token_program))
            .seller_fee_basis_points(500)
            .decimals(0)
            .invoke_signed(&[&[CREATOR_SEED, &[ctx.bumps.mint_authority]]])?;

        MintV1CpiBuilder::new(&ctx.accounts.token_metadata_program)
            .token(&ctx.accounts.mint_assoc)
            .token_owner(Some(&ctx.accounts.signer))
            .metadata(&ctx.accounts.mint_metadata)
            .master_edition(Some(&ctx.accounts.mint_master_edition))
            .mint(&ctx.accounts.mint)
            .payer(&ctx.accounts.signer)
            .authority(&ctx.accounts.mint_authority)
            .token_record(Some(&ctx.accounts.token_record))
            .system_program(&ctx.accounts.system_program)
            .sysvar_instructions(&ctx.accounts.sysvar_ixs)
            .spl_token_program(&ctx.accounts.token_program)
            .spl_ata_program(&ctx.accounts.spl_ata_program)
            .amount(1)
            .invoke_signed(&[&[CREATOR_SEED, &[ctx.bumps.mint_authority]]])?;

        VerifyCollectionV1CpiBuilder::new(&ctx.accounts.token_metadata_program)
            .authority(&ctx.accounts.mint_authority)
            .metadata(&ctx.accounts.mint_metadata)
            .collection_mint(&ctx.accounts.collection_mint.to_account_info())
            .collection_metadata(Some(&ctx.accounts.collection_metadata.to_account_info()))
            .collection_master_edition(Some(
                &ctx.accounts.collection_master_edition.to_account_info(),
            ))
            .system_program(&ctx.accounts.system_program)
            .sysvar_instructions(&ctx.accounts.sysvar_ixs)
            .invoke_signed(&[&[CREATOR_SEED, &[ctx.bumps.mint_authority]]])?;

        UpdateV1CpiBuilder::new(&ctx.accounts.token_metadata_program)
            .payer(&ctx.accounts.signer)
            .authority(&ctx.accounts.mint_authority)
            .mint(&ctx.accounts.mint)
            .metadata(&ctx.accounts.mint_metadata)
            .token(Some(&ctx.accounts.mint_assoc))
            .authorization_rules(Some(&ctx.accounts.rule_set))
            .authorization_rules_program(Some(&ctx.accounts.authorization_rules_program))
            .system_program(&ctx.accounts.system_program)
            .sysvar_instructions(&ctx.accounts.sysvar_ixs)
            .primary_sale_happened(true)
            .new_update_authority(NFT_UPDATE_AUTH)
            .invoke_signed(&[&[CREATOR_SEED, &[ctx.bumps.mint_authority]]])?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [CREATOR_SEED],
        bump,
    )]
    /// CHECK: PDA signer
    pub mint_authority: UncheckedAccount<'info>,

    #[account(
        init,
        seeds = [
            REGISTER_SEED,
            &extract_mint_id(&mint.key()).unwrap().to_le_bytes()
        ],
        payer = signer,
        space = 8 + std::mem::size_of::<Register>(),
        bump,
    )]
    pub register: Account<'info, Register>,

    // EMPTY ACCOUNTS
    #[account(mut, signer)]
    /// CHECK: Created during mint
    pub mint: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Created during mint
    pub mint_metadata: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Created during mint
    pub mint_master_edition: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Created during mint
    pub mint_assoc: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Created during mint
    pub token_record: UncheckedAccount<'info>,

    // COLLECTION ACCOUNTS
    #[account(address = COLLECTION)]
    pub collection_mint: Account<'info, Mint>,
    #[account(mut)]
    pub collection_metadata: Account<'info, MetadataAccount>,
    pub collection_master_edition: Account<'info, MasterEditionAccount>,

    #[account(address = METAPLEX_DEFAULT_RULES)]
    /// CHECK: Address check
    pub rule_set: UncheckedAccount<'info>,

    #[account(address = sysvar::instructions::ID)]
    /// CHECK: Address check
    pub sysvar_ixs: UncheckedAccount<'info>,

    #[account(address = mpl_token_auth_rules::ID)]
    /// CHECK: Address check
    pub authorization_rules_program: UncheckedAccount<'info>,

    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub spl_ata_program: Program<'info, AssociatedToken>,
}

fn extract_mint_id(input: &Pubkey) -> Option<u32> {
    input.to_string().strip_prefix("pow").and_then(|s| {
        s.chars()
            // Gathers 1-9 numbers
            // 0 is not found in pubkeys
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .ok()
    })
}

#[account]
pub struct Register {
    id: u32,
    tier: u8,
    mint: Pubkey,
}
