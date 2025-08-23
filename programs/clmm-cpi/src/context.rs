use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::memo::Memo;
use anchor_spl::metadata::Metadata;
use anchor_spl::token;
use anchor_spl::token::Token;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount, TokenInterface};
#[derive(Accounts)]
pub struct CreatePool<'info> {
    /// Address paying to create the pool. Can be anyone
    #[account(mut)]
    pub pool_creator: Signer<'info>,

    /// Which config the pool belongs to.
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// Initialize an account to store the pool state
    #[account(
        init,
        seeds = [
            POOL_SEED.as_bytes(),
            amm_config.key().as_ref(),
            token_mint_0.key().as_ref(),
            token_mint_1.key().as_ref(),
        ],
        bump,
        payer = pool_creator,
        space = PoolState::LEN
    )]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// Token_0 mint, the key must smaller then token_1 mint.
    #[account(
        constraint = token_mint_0.key() < token_mint_1.key(),
        mint::token_program = token_program_0
    )]
    pub token_mint_0: Box<InterfaceAccount<'info, Mint>>,

    /// Token_1 mint
    #[account(
        mint::token_program = token_program_1
    )]
    pub token_mint_1: Box<InterfaceAccount<'info, Mint>>,

    /// Token_0 vault for the pool
    /// CHECK: The account is created internally by the contract, and removing `init` is just to bypass the stack overflow check (due to anchor; they will fix this issue later).
    #[account(
        // init,
        mut,
        seeds =[
            POOL_VAULT_SEED.as_bytes(),
            pool_state.key().as_ref(),
            token_mint_0.key().as_ref(),
        ],
        bump,
        // payer = pool_creator,
        // token::mint = token_mint_0,
        // token::authority = pool_state,
        // token::token_program = token_program_0,
    )]
    // pub token_vault_0: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_vault_0: UncheckedAccount<'info>,

    /// Token_1 vault for the pool
    /// CHECK:The account is created internally by the contract, and removing `init` is just to bypass the stack overflow check (due to anchor; they will fix this issue later).
    #[account(
        // init,
        mut,
        seeds =[
            POOL_VAULT_SEED.as_bytes(),
            pool_state.key().as_ref(),
            token_mint_1.key().as_ref(),
        ],
        bump,
        // payer = pool_creator,
        // token::mint = token_mint_1,
        // token::authority = pool_state,
        // token::token_program = token_program_1,
    )]
    // pub token_vault_1: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_vault_1: UncheckedAccount<'info>,

    /// Initialize an account to store oracle observations
    #[account(
        init,
        seeds = [
            OBSERVATION_SEED.as_bytes(),
            pool_state.key().as_ref(),
        ],
        bump,
        payer = pool_creator,
        space = ObservationState::LEN
    )]
    pub observation_state: AccountLoader<'info, ObservationState>,

    /// Initialize an account to store if a tick array is initialized.
    #[account(
        init,
        seeds = [
            POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(),
            pool_state.key().as_ref(),
        ],
        bump,
        payer = pool_creator,
        space = TickArrayBitmapExtension::LEN
    )]
    pub tick_array_bitmap: AccountLoader<'info, TickArrayBitmapExtension>,

    /// Spl token program or token program 2022
    pub token_program_0: Interface<'info, TokenInterface>,
    /// Spl token program or token program 2022
    pub token_program_1: Interface<'info, TokenInterface>,
    /// To create a new program account
    pub system_program: Program<'info, System>,
    /// Sysvar for program account
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(tick_lower_index: i32, tick_upper_index: i32,tick_array_lower_start_index:i32,tick_array_upper_start_index:i32)]
pub struct OpenPosition<'info> {
    /// Pays to mint the position
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Receives the position NFT
    pub position_nft_owner: UncheckedAccount<'info>,

    /// Unique token mint address
    /// CHECK:The account is created internally by the contract, and removing `init` is just to bypass the stack overflow check (due to anchor; they will fix this issue later).
    #[account(
        mut,
        // init,
        // mint::decimals = 0,
        // mint::authority = pool_state.key(),
        // payer = payer,
        // mint::token_program = token_program,
    )]
    // pub position_nft_mint: Box<InterfaceAccount<'info, Mint>>,
    pub position_nft_mint: Signer<'info>,

    /// Token account where position NFT will be minted
    #[account(
        init,
        associated_token::mint = position_nft_mint,
        associated_token::authority = position_nft_owner,
        payer = payer,
        token::token_program = token_program,
    )]
    pub position_nft_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// To store metaplex metadata
    /// CHECK: Safety check performed inside function body
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,

    /// Add liquidity for this pool
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// CHECK: Deprecated: protocol_position is deprecated and kept for compatibility.
    pub protocol_position: UncheckedAccount<'info>,

    /// CHECK: Account to store data for the position's lower tick
    #[account(
        mut,
        seeds = [
            TICK_ARRAY_SEED.as_bytes(),
            pool_state.key().as_ref(),
            &tick_array_lower_start_index.to_be_bytes(),
        ],
        bump,
    )]
    pub tick_array_lower: UncheckedAccount<'info>,

    /// CHECK: Account to store data for the position's upper tick
    #[account(
        mut,
        seeds = [
            TICK_ARRAY_SEED.as_bytes(),
            pool_state.key().as_ref(),
            &tick_array_upper_start_index.to_be_bytes(),
        ],
        bump,
    )]
    pub tick_array_upper: UncheckedAccount<'info>,

    /// personal position state
    #[account(
        init,
        seeds = [POSITION_SEED.as_bytes(), position_nft_mint.key().as_ref()],
        bump,
        payer = payer,
        space = PersonalPositionState::LEN
    )]
    pub personal_position: Box<Account<'info, PersonalPositionState>>,

    /// The user token_0 account deposit token to the pool
    #[account(
        mut,
        token::mint = token_vault_0.mint
    )]
    pub token_account_0: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The user token_1 account deposit token to the pool
    #[account(
        mut,
        token::mint = token_vault_1.mint
    )]
    pub token_account_1: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_0
    #[account(
        mut,
        constraint = token_vault_0.key() == pool_state.load()?.token_vault_0
    )]
    pub token_vault_0: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_1
    #[account(
        mut,
        constraint = token_vault_1.key() == pool_state.load()?.token_vault_1
    )]
    pub token_vault_1: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Sysvar for token mint and ATA creation
    pub rent: Sysvar<'info, Rent>,

    /// Program to create the position manager state account
    pub system_program: Program<'info, System>,

    /// Program to create mint account and mint tokens
    pub token_program: Program<'info, Token>,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// Program to create NFT metadata
    /// CHECK: Metadata program address constraint applied
    pub metadata_program: Program<'info, Metadata>,
    // remaining account
    // #[account(
    //     seeds = [
    //         POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(),
    //         pool_state.key().as_ref(),
    //     ],
    //     bump
    // )]
    // pub tick_array_bitmap: AccountLoader<'info, TickArrayBitmapExtension>,
}

#[derive(Accounts)]
#[instruction(tick_lower_index: i32, tick_upper_index: i32,tick_array_lower_start_index:i32,tick_array_upper_start_index:i32)]
pub struct OpenPositionV2<'info> {
    /// Pays to mint the position
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Receives the position NFT
    pub position_nft_owner: UncheckedAccount<'info>,

    /// Unique token mint address
    /// CHECK:The account is created internally by the contract, and removing `init` is just to bypass the stack overflow check (due to anchor; they will fix this issue later).
    #[account(
        mut,
        // init,
        // mint::decimals = 0,
        // mint::authority = pool_state.key(),
        // payer = payer,
        // mint::token_program = token_program,
    )]
    // pub position_nft_mint: Box<InterfaceAccount<'info, Mint>>,
    pub position_nft_mint: Signer<'info>,

    /// Token account where position NFT will be minted
    /// CHECK:The account is created internally by the contract, and removing `init` is just to bypass the stack overflow check (due to anchor; they will fix this issue later).
    #[account(
        mut,
        // init,
        // associated_token::mint = position_nft_mint,
        // associated_token::authority = position_nft_owner,
        // payer = payer,
        // token::token_program = token_program,
    )]
    // pub position_nft_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub position_nft_account: UncheckedAccount<'info>,

    /// To store metaplex metadata
    /// CHECK: Safety check performed inside function body
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,

    /// Add liquidity for this pool
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// CHECK: Deprecated: protocol_position is deprecated and kept for compatibility.
    pub protocol_position: UncheckedAccount<'info>,

    /// CHECK: Account to store data for the position's lower tick
    #[account(
        mut,
        seeds = [
            TICK_ARRAY_SEED.as_bytes(),
            pool_state.key().as_ref(),
            &tick_array_lower_start_index.to_be_bytes(),
        ],
        bump,
    )]
    pub tick_array_lower: UncheckedAccount<'info>,

    /// CHECK: Account to store data for the position's upper tick
    #[account(
        mut,
        seeds = [
            TICK_ARRAY_SEED.as_bytes(),
            pool_state.key().as_ref(),
            &tick_array_upper_start_index.to_be_bytes(),
        ],
        bump,
    )]
    pub tick_array_upper: UncheckedAccount<'info>,

    /// personal position state
    #[account(
        init,
        seeds = [POSITION_SEED.as_bytes(), position_nft_mint.key().as_ref()],
        bump,
        payer = payer,
        space = PersonalPositionState::LEN
    )]
    pub personal_position: Box<Account<'info, PersonalPositionState>>,

    /// The user token_0 account deposit token to the pool
    #[account(
        mut,
        token::mint = token_vault_0.mint
    )]
    pub token_account_0: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The user token_1 account deposit token to the pool
    #[account(
        mut,
        token::mint = token_vault_1.mint
    )]
    pub token_account_1: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_0
    #[account(
        mut,
        constraint = token_vault_0.key() == pool_state.load()?.token_vault_0
    )]
    pub token_vault_0: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_1
    #[account(
        mut,
        constraint = token_vault_1.key() == pool_state.load()?.token_vault_1
    )]
    pub token_vault_1: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Sysvar for token mint and ATA creation
    pub rent: Sysvar<'info, Rent>,

    /// Program to create the position manager state account
    pub system_program: Program<'info, System>,

    /// Program to create mint account and mint tokens
    pub token_program: Program<'info, Token>,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// Program to create NFT metadata
    /// CHECK: Metadata program address constraint applied
    pub metadata_program: Program<'info, Metadata>,
    /// Program to create mint account and mint tokens
    pub token_program_2022: Program<'info, Token2022>,
    /// The mint of token vault 0
    #[account(
        address = token_vault_0.mint
    )]
    pub vault_0_mint: Box<InterfaceAccount<'info, Mint>>,
    /// The mint of token vault 1
    #[account(
        address = token_vault_1.mint
    )]
    pub vault_1_mint: Box<InterfaceAccount<'info, Mint>>,
    // remaining account
    // #[account(
    //     seeds = [
    //         POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(),
    //         pool_state.key().as_ref(),
    //     ],
    //     bump
    // )]
    // pub tick_array_bitmap: AccountLoader<'info, TickArrayBitmapExtension>,
}

#[derive(Accounts)]
#[instruction(tick_lower_index: i32, tick_upper_index: i32,tick_array_lower_start_index:i32,tick_array_upper_start_index:i32)]
pub struct OpenPositionWithToken22Nft<'info> {
    /// Pays to mint the position
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Receives the position NFT
    pub position_nft_owner: UncheckedAccount<'info>,

    /// Unique token mint address, initialize in constract
    #[account(mut)]
    pub position_nft_mint: Signer<'info>,

    /// CHECK: ATA address where position NFT will be minted, initialize in constract
    #[account(mut)]
    pub position_nft_account: UncheckedAccount<'info>,

    /// Add liquidity for this pool
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// CHECK: Deprecated: protocol_position is deprecated and kept for compatibility.
    pub protocol_position: UncheckedAccount<'info>,

    /// CHECK:  Account to store data for the position's lower tick
    #[account(
        mut,
        seeds = [
            TICK_ARRAY_SEED.as_bytes(),
            pool_state.key().as_ref(),
            &tick_array_lower_start_index.to_be_bytes(),
        ],
        bump,
    )]
    pub tick_array_lower: UncheckedAccount<'info>,

    /// CHECK: Account to store data for the position's upper tick
    #[account(
        mut,
        seeds = [
            TICK_ARRAY_SEED.as_bytes(),
            pool_state.key().as_ref(),
            &tick_array_upper_start_index.to_be_bytes(),
        ],
        bump,
    )]
    pub tick_array_upper: UncheckedAccount<'info>,

    /// personal position state
    #[account(
        init,
        seeds = [POSITION_SEED.as_bytes(), position_nft_mint.key().as_ref()],
        bump,
        payer = payer,
        space = PersonalPositionState::LEN
    )]
    pub personal_position: Box<Account<'info, PersonalPositionState>>,

    /// The token_0 account deposit token to the pool
    #[account(
        mut,
        token::mint = token_vault_0.mint
    )]
    pub token_account_0: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The token_1 account deposit token to the pool
    #[account(
        mut,
        token::mint = token_vault_1.mint
    )]
    pub token_account_1: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_0
    #[account(
        mut,
        constraint = token_vault_0.key() == pool_state.load()?.token_vault_0
    )]
    pub token_vault_0: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_1
    #[account(
        mut,
        constraint = token_vault_1.key() == pool_state.load()?.token_vault_1
    )]
    pub token_vault_1: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Sysvar for token mint and ATA creation
    pub rent: Sysvar<'info, Rent>,

    /// Program to create the position manager state account
    pub system_program: Program<'info, System>,

    /// Program to transfer for token account
    pub token_program: Program<'info, Token>,

    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// Program to create NFT mint/token account and transfer for token22 account
    pub token_program_2022: Program<'info, Token2022>,

    /// The mint of token vault 0
    #[account(
        address = token_vault_0.mint
    )]
    pub vault_0_mint: Box<InterfaceAccount<'info, Mint>>,

    /// The mint of token vault 1
    #[account(
        address = token_vault_1.mint
    )]
    pub vault_1_mint: Box<InterfaceAccount<'info, Mint>>,
    // remaining account
    // #[account(
    //     seeds = [
    //         POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(),
    //         pool_state.key().as_ref(),
    //     ],
    //     bump
    // )]
    // pub tick_array_bitmap: AccountLoader<'info, TickArrayBitmapExtension>,
}

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    /// The position nft owner
    #[account(mut)]
    pub nft_owner: Signer<'info>,

    /// Unique token mint address
    #[account(
      mut,
      address = personal_position.nft_mint,
      mint::token_program = token_program,
    )]
    pub position_nft_mint: Box<InterfaceAccount<'info, Mint>>,

    /// Token account where position NFT will be minted
    #[account(
        mut,
        associated_token::mint = position_nft_mint,
        associated_token::authority = nft_owner,
        constraint = position_nft_account.amount == 1,
        token::token_program = token_program,
    )]
    pub position_nft_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// To store metaplex metadata
    /// CHECK: Safety check performed inside function body
    // #[account(mut)]
    // pub metadata_account: UncheckedAccount<'info>,

    /// Metadata for the tokenized position
    #[account(
        mut, 
        seeds = [POSITION_SEED.as_bytes(), position_nft_mint.key().as_ref()],
        bump,
        close = nft_owner
    )]
    pub personal_position: Box<Account<'info, PersonalPositionState>>,

    /// Program to create the position manager state account
    pub system_program: Program<'info, System>,
    /// Program to create mint account and mint tokens
    pub token_program: Program<'info, Token>,
    // /// Reserved for upgrade
    // pub token_program_2022: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct IncreaseLiquidity<'info> {
    /// Pays to mint the position
    pub nft_owner: Signer<'info>,

    /// The token account for nft
    #[account(
        constraint = nft_account.mint == personal_position.nft_mint
    )]
    pub nft_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// CHECK: Deprecated: protocol_position is deprecated and kept for compatibility.
    pub protocol_position: UncheckedAccount<'info>,

    /// Increase liquidity for this position
    #[account(mut, constraint = personal_position.pool_id == pool_state.key())]
    pub personal_position: Box<Account<'info, PersonalPositionState>>,

    /// Stores init state for the lower tick
    #[account(mut, constraint = tick_array_lower.load()?.pool_id == pool_state.key())]
    pub tick_array_lower: AccountLoader<'info, TickArrayState>,

    /// Stores init state for the upper tick
    #[account(mut, constraint = tick_array_upper.load()?.pool_id == pool_state.key())]
    pub tick_array_upper: AccountLoader<'info, TickArrayState>,

    /// The payer's token account for token_0
    #[account(
        mut,
        token::mint = token_vault_0.mint
    )]
    pub token_account_0: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The token account spending token_1 to mint the position
    #[account(
        mut,
        token::mint = token_vault_1.mint
    )]
    pub token_account_1: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_0
    #[account(
        mut,
        constraint = token_vault_0.key() == pool_state.load()?.token_vault_0
    )]
    pub token_vault_0: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_1
    #[account(
        mut,
        constraint = token_vault_1.key() == pool_state.load()?.token_vault_1
    )]
    pub token_vault_1: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Program to create mint account and mint tokens
    pub token_program: Program<'info, Token>,
    // remaining account
    // #[account(
    //     seeds = [
    //         POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(),
    //         pool_state.key().as_ref(),
    //     ],
    //     bump
    // )]
    // pub tick_array_bitmap: AccountLoader<'info, TickArrayBitmapExtension>,
}

#[derive(Accounts)]
pub struct IncreaseLiquidityV2<'info> {
    /// Pays to mint the position
    pub nft_owner: Signer<'info>,

    /// The token account for nft
    #[account(
        constraint = nft_account.mint == personal_position.nft_mint,
        token::token_program = token_program,
    )]
    pub nft_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// CHECK: Deprecated: protocol_position is deprecated and kept for compatibility.
    pub protocol_position: UncheckedAccount<'info>,

    /// Increase liquidity for this position
    #[account(mut, constraint = personal_position.pool_id == pool_state.key())]
    pub personal_position: Box<Account<'info, PersonalPositionState>>,

    /// Stores init state for the lower tick
    #[account(mut, constraint = tick_array_lower.load()?.pool_id == pool_state.key())]
    pub tick_array_lower: AccountLoader<'info, TickArrayState>,

    /// Stores init state for the upper tick
    #[account(mut, constraint = tick_array_upper.load()?.pool_id == pool_state.key())]
    pub tick_array_upper: AccountLoader<'info, TickArrayState>,

    /// The payer's token account for token_0
    #[account(
        mut,
        token::mint = token_vault_0.mint
    )]
    pub token_account_0: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The token account spending token_1 to mint the position
    #[account(
        mut,
        token::mint = token_vault_1.mint
    )]
    pub token_account_1: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_0
    #[account(
        mut,
        constraint = token_vault_0.key() == pool_state.load()?.token_vault_0
    )]
    pub token_vault_0: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_1
    #[account(
        mut,
        constraint = token_vault_1.key() == pool_state.load()?.token_vault_1
    )]
    pub token_vault_1: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Program to create mint account and mint tokens
    pub token_program: Program<'info, Token>,

    /// Token program 2022
    pub token_program_2022: Program<'info, Token2022>,

    /// The mint of token vault 0
    #[account(
            address = token_vault_0.mint
    )]
    pub vault_0_mint: Box<InterfaceAccount<'info, Mint>>,

    /// The mint of token vault 1
    #[account(
            address = token_vault_1.mint
    )]
    pub vault_1_mint: Box<InterfaceAccount<'info, Mint>>,
    // remaining account
    // #[account(
    //     seeds = [
    //         POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(),
    //         pool_state.key().as_ref(),
    //     ],
    //     bump
    // )]
    // pub tick_array_bitmap: AccountLoader<'info, TickArrayBitmapExtension>,
}

#[derive(Accounts)]
pub struct DecreaseLiquidity<'info> {
    /// The position owner or delegated authority
    pub nft_owner: Signer<'info>,

    /// The token account for the tokenized position
    #[account(
        constraint = nft_account.mint == personal_position.nft_mint,
        token::token_program = token_program,
    )]
    pub nft_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Decrease liquidity for this position
    #[account(mut, constraint = personal_position.pool_id == pool_state.key())]
    pub personal_position: Box<Account<'info, PersonalPositionState>>,

    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// CHECK: Deprecated: protocol_position is deprecated and kept for compatibility.
    pub protocol_position: UncheckedAccount<'info>,

    /// Token_0 vault
    #[account(
        mut,
        constraint = token_vault_0.key() == pool_state.load()?.token_vault_0
    )]
    pub token_vault_0: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Token_1 vault
    #[account(
        mut,
        constraint = token_vault_1.key() == pool_state.load()?.token_vault_1
    )]
    pub token_vault_1: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Stores init state for the lower tick
    #[account(mut, constraint = tick_array_lower.load()?.pool_id == pool_state.key())]
    pub tick_array_lower: AccountLoader<'info, TickArrayState>,

    /// Stores init state for the upper tick
    #[account(mut, constraint = tick_array_upper.load()?.pool_id == pool_state.key())]
    pub tick_array_upper: AccountLoader<'info, TickArrayState>,

    /// The destination token account for receive amount_0
    #[account(
        mut,
        token::mint = token_vault_0.mint
    )]
    pub recipient_token_account_0: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The destination token account for receive amount_1
    #[account(
        mut,
        token::mint = token_vault_1.mint
    )]
    pub recipient_token_account_1: Box<InterfaceAccount<'info, TokenAccount>>,

    /// SPL program to transfer out tokens
    pub token_program: Program<'info, Token>,
    // remaining account
    // #[account(
    //     seeds = [
    //         POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(),
    //         pool_state.key().as_ref(),
    //     ],
    //     bump
    // )]
    // pub tick_array_bitmap: AccountLoader<'info, TickArrayBitmapExtension>,
}

#[derive(Accounts)]
pub struct DecreaseLiquidityV2<'info> {
    /// The position owner or delegated authority
    pub nft_owner: Signer<'info>,

    /// The token account for the tokenized position
    #[account(
        constraint = nft_account.mint == personal_position.nft_mint,
        token::token_program = token_program,
    )]
    pub nft_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Decrease liquidity for this position
    #[account(mut, constraint = personal_position.pool_id == pool_state.key())]
    pub personal_position: Box<Account<'info, PersonalPositionState>>,

    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// CHECK: Deprecated: protocol_position is deprecated and kept for compatibility.
    pub protocol_position: UncheckedAccount<'info>,

    /// Token_0 vault
    #[account(
        mut,
        constraint = token_vault_0.key() == pool_state.load()?.token_vault_0
    )]
    pub token_vault_0: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Token_1 vault
    #[account(
        mut,
        constraint = token_vault_1.key() == pool_state.load()?.token_vault_1
    )]
    pub token_vault_1: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Stores init state for the lower tick
    #[account(mut, constraint = tick_array_lower.load()?.pool_id == pool_state.key())]
    pub tick_array_lower: AccountLoader<'info, TickArrayState>,

    /// Stores init state for the upper tick
    #[account(mut, constraint = tick_array_upper.load()?.pool_id == pool_state.key())]
    pub tick_array_upper: AccountLoader<'info, TickArrayState>,

    /// The destination token account for receive amount_0
    #[account(
        mut,
        token::mint = token_vault_0.mint
    )]
    pub recipient_token_account_0: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The destination token account for receive amount_1
    #[account(
        mut,
        token::mint = token_vault_1.mint
    )]
    pub recipient_token_account_1: Box<InterfaceAccount<'info, TokenAccount>>,

    /// SPL program to transfer out tokens
    pub token_program: Program<'info, Token>,
    /// Token program 2022
    pub token_program_2022: Program<'info, Token2022>,

    /// memo program
    pub memo_program: Program<'info, Memo>,

    /// The mint of token vault 0
    #[account(
        address = token_vault_0.mint
    )]
    pub vault_0_mint: Box<InterfaceAccount<'info, Mint>>,

    /// The mint of token vault 1
    #[account(
        address = token_vault_1.mint
    )]
    pub vault_1_mint: Box<InterfaceAccount<'info, Mint>>,
    // remaining account
    // #[account(
    //     seeds = [
    //         POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(),
    //         pool_state.key().as_ref(),
    //     ],
    //     bump
    // )]
    // pub tick_array_bitmap: AccountLoader<'info, TickArrayBitmapExtension>,
}

#[derive(Accounts)]
pub struct SwapRouterBaseIn<'info> {
    /// The user performing the swap
    pub payer: Signer<'info>,

    /// The token account that pays input tokens for the swap
    #[account(mut)]
    pub input_token_account: InterfaceAccount<'info, TokenAccount>,

    /// The mint of input token
    #[account(mut)]
    pub input_token_mint: InterfaceAccount<'info, Mint>,

    /// SPL program for token transfers
    pub token_program: Program<'info, Token>,
    /// SPL program 2022 for token transfers
    pub token_program_2022: Program<'info, Token2022>,

    /// CHECK:
    // #[account(
    //     address = spl_memo::id()
    // )]
    pub memo_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct SwapSingle<'info> {
    /// The user performing the swap
    pub payer: Signer<'info>,

    /// The factory state to read protocol fees
    #[account(address = pool_state.load()?.amm_config)]
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// The program account of the pool in which the swap will be performed
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// The user token account for input token
    #[account(
        mut,
        token::token_program = token_program,
    )]
    pub input_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The user token account for output token
    #[account(
        mut,
        token::token_program = token_program,
    )]
    pub output_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The vault token account for input token
    #[account(
        mut,
        token::token_program = token_program,
    )]
    pub input_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The vault token account for output token
    #[account(
        mut,
        token::token_program = token_program,
    )]
    pub output_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The program account for the most recent oracle observation
    #[account(mut, address = pool_state.load()?.observation_key)]
    pub observation_state: AccountLoader<'info, ObservationState>,

    /// SPL program for token transfers
    pub token_program: Program<'info, Token>,

    #[account(mut, constraint = tick_array.load()?.pool_id == pool_state.key())]
    pub tick_array: AccountLoader<'info, TickArrayState>,
}

#[derive(Accounts)]
pub struct SwapSingleV2<'info> {
    /// The user performing the swap
    pub payer: Signer<'info>,

    /// The factory state to read protocol fees
    #[account(address = pool_state.load()?.amm_config)]
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// The program account of the pool in which the swap will be performed
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// The user token account for input token
    #[account(mut)]
    pub input_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The user token account for output token
    #[account(mut)]
    pub output_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The vault token account for input token
    #[account(mut)]
    pub input_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The vault token account for output token
    #[account(mut)]
    pub output_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The program account for the most recent oracle observation
    #[account(mut, address = pool_state.load()?.observation_key)]
    pub observation_state: AccountLoader<'info, ObservationState>,

    /// SPL program for token transfers
    pub token_program: Program<'info, Token>,

    /// SPL program 2022 for token transfers
    pub token_program_2022: Program<'info, Token2022>,

    /// memo program
    pub memo_program: Program<'info, Memo>,

    /// The mint of token vault 0
    #[account(
        address = input_vault.mint
    )]
    pub input_vault_mint: Box<InterfaceAccount<'info, Mint>>,

    /// The mint of token vault 1
    #[account(
        address = output_vault.mint
    )]
    pub output_vault_mint: Box<InterfaceAccount<'info, Mint>>,
    // remaining accounts
    // tickarray_bitmap_extension: must add account if need regardless the sequence
    // tick_array_account_1
    // tick_array_account_2
    // tick_array_account_...
}

#[derive(Accounts)]
pub struct CollectRemainingRewards<'info> {
    /// The founder who init reward info in berfore
    pub reward_funder: Signer<'info>,
    /// The funder's reward token account
    #[account(mut)]
    pub funder_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    /// Set reward for this pool
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,
    /// Reward vault transfer remaining token to founder token account
    pub reward_token_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    /// The mint of reward token vault
    #[account(
        address = reward_token_vault.mint
    )]
    pub reward_vault_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    /// Token program 2022
    pub token_program_2022: Program<'info, Token2022>,

    /// memo program
    pub memo_program: Program<'info, Memo>,
}

#[derive(Accounts)]
pub struct UpdateRewardInfos<'info> {
    /// The liquidity pool for which reward info to update
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,
}
