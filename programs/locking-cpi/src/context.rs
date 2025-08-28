use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    memo::Memo,
    metadata::Metadata,
    token::Token,
    token_2022::Token2022,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use raydium_clmm_cpi::{
    program::RaydiumClmm,
    states::{
        PersonalPositionState, PoolState, ProtocolPositionState, TickArrayState, POSITION_SEED,
    },
};

use raydium_cpmm_cpi::program::RaydiumCpmm;

#[derive(Accounts)]
pub struct LockClmmPosition<'info> {
    /// CHECK: the authority of position nft account that clmm is locked
    #[account(
         seeds = [
            crate::LOCK_CLMM_AUTH_SEED.as_bytes(),
        ],
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

    /// Pay to create account lamports
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The position NFT owner
    pub position_nft_owner: Signer<'info>,

    /// CHECK: locked position NFT allows who to collect fee
    pub fee_nft_owner: UncheckedAccount<'info>,

    /// The token account for the tokenized position
    #[account(
        mut,
        constraint = position_nft_account.mint == personal_position.nft_mint && position_nft_account.amount == 1,
        token::authority = position_nft_owner,
        token::token_program = locked_nft_token_program,
    )]
    pub position_nft_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Lock personal position associated with position_nft_account's mint
    #[account()]
    pub personal_position: Box<Account<'info, PersonalPositionState>>,

    /// Position NFT mint to create a ATA `locked_nft_account` belongs to program
    #[account(
        address = position_nft_account.mint
    )]
    pub position_nft_mint: Box<InterfaceAccount<'info, Mint>>,

    /// CHECK: Program's token account where `position_nft_account` will transfer to, init by locking program
    #[account(
        mut
        // init,
        // associated_token::mint = position_nft_mint,
        // associated_token::authority = authority,
        // associated_token::token_program = locked_nft_token_program,
        // payer = payer,
    )]
    pub locked_nft_account: UncheckedAccount<'info>,

    ///  Store the locked information of the personal position
    #[account(
        init,
        seeds = [
            LOCKED_POSITION_SEED.as_bytes(),
            fee_nft_mint.key().as_ref(),
        ],
        bump,
        payer = payer,
        space = LockedClmmPositionState::LEN
    )]
    pub locked_position: Box<Account<'info, LockedClmmPositionState>>,

    /// Create a unique fee nft mint
    #[account(
        init,
        mint::decimals = 0,
        mint::authority = authority,
        payer = payer,
        mint::token_program = fee_nft_token_program,
    )]
    pub fee_nft_mint: Box<InterfaceAccount<'info, Mint>>,

    /// Token account where fee nft will be minted to
    #[account(
        init,
        associated_token::mint = fee_nft_mint,
        associated_token::authority = fee_nft_owner,
        associated_token::token_program = fee_nft_token_program,
        payer = payer,
    )]
    pub fee_nft_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// To store metaplex metadata
    /// CHECK: Safety check performed inside function body
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,

    /// Program to create NFT metadata account
    /// CHECK: Metadata program address constraint applied
    pub metadata_program: Program<'info, Metadata>,

    /// Program to create an ATA for receiving fee NFT
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// Sysvar for token mint and ATA creation
    pub rent: Sysvar<'info, Rent>,

    /// Token program fee NFT mint/account belongs to
    pub fee_nft_token_program: Program<'info, Token>,

    /// Token program position NFT mint/account belongs to
    pub locked_nft_token_program: Interface<'info, TokenInterface>,

    /// Program to create new account
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CollectClmmFeeAndReward<'info> {
    /// CHECK: the authority of position nft account that clmm is locked
    #[account(
        seeds = [
            crate::LOCK_CLMM_AUTH_SEED.as_bytes(),
        ],
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

    /// The owner who has fee nft account
    pub fee_nft_owner: Signer<'info>,

    /// Fee token account
    #[account(
        token::mint = locked_position.fee_nft_mint,
        token::authority = fee_nft_owner,
        constraint = fee_nft_account.amount == 1
    )]
    pub fee_nft_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Store the locked the information of position
    #[account(
        seeds = [
            LOCKED_POSITION_SEED.as_bytes(),
            fee_nft_account.mint.as_ref(),
        ],
        bump,
    )]
    pub locked_position: Box<Account<'info, LockedClmmPositionState>>,

    /// clmm program
    pub clmm_program: Program<'info, RaydiumClmm>,

    /// Program ATA locked NFT account or user ATA position NFT account
    #[account(
        mut,
        constraint = locked_nft_account.mint == personal_position.nft_mint && locked_nft_account.amount == 1,
        address = locked_position.locked_nft_account,
        token::authority = authority,
    )]
    pub locked_nft_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Decrease liquidity for this position
    #[account(
        mut, 
        address = locked_position.position_id,
        constraint = personal_position.pool_id == pool_state.key()
    )]
    pub personal_position: Box<Account<'info, PersonalPositionState>>,

    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// CHECK: Deprecated: protocol_position is deprecated and kept for compatibility.
    pub protocol_position: UncheckedAccount<'info>,

    /// The address that holds pool tokens for token_0
    #[account(
        mut,
        address = pool_state.load()?.token_vault_0
    )]
    pub token_0_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_1
    #[account(
        mut,
        address = pool_state.load()?.token_vault_1
    )]
    pub token_1_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Stores init state for the lower tick
    #[account(mut, constraint = tick_array_lower.load()?.pool_id == pool_state.key())]
    pub tick_array_lower: AccountLoader<'info, TickArrayState>,

    /// Stores init state for the upper tick
    #[account(mut, constraint = tick_array_upper.load()?.pool_id == pool_state.key())]
    pub tick_array_upper: AccountLoader<'info, TickArrayState>,

    /// The destination token account for receive amount_0
    #[account(
        mut,
        token::mint = token_0_vault.mint
    )]
    pub recipient_token_0_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The destination token account for receive amount_1
    #[account(
        mut,
        token::mint = token_1_vault.mint
    )]
    pub recipient_token_1_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// SPL program to transfer out tokens
    pub token_program: Program<'info, Token>,

    /// Token program 2022
    pub token_program_2022: Program<'info, Token2022>,

    /// memo program
    pub memo_program: Program<'info, Memo>,

    /// The mint of token vault 0
    #[account(
        address = token_0_vault.mint
    )]
    pub vault_0_mint: Box<InterfaceAccount<'info, Mint>>,

    /// The mint of token vault 1
    #[account(
        address = token_1_vault.mint
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
pub struct LockCpLiquidity<'info> {
    /// CHECK: the authority of token vault that cp is locked
    #[account(
        seeds = [
            crate::LOCK_CP_AUTH_SEED.as_bytes(),
        ],
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

    /// Pay to create account lamports
    #[account(mut)]
    pub payer: Signer<'info>,

    /// who want to lock liquidity
    pub liquidity_owner: Signer<'info>,

    /// CHECK: locked liquidity allow who to collect fee
    pub fee_nft_owner: UncheckedAccount<'info>,

    /// Create a unique fee nft mint
    #[account(
        init,
        mint::decimals = 0,
        mint::authority = authority,
        payer = payer,
        mint::token_program = token_program,
    )]
    pub fee_nft_mint: Box<InterfaceAccount<'info, Mint>>,

    /// CHECK: Token account where fee nft will be minted to, init by locking program
    #[account(
        mut,
        // init,
        // associated_token::mint = fee_nft_mint,
        // associated_token::authority = fee_nft_owner,
        // token::token_program = token_program,
    )]
    pub fee_nft_account: UncheckedAccount<'info>,

    /// CHECK: Indicates which pool the locked liquidity belong to
    #[account()]
    pub pool_state: UncheckedAccount<'info>,

    /// Store the locked information of liquidity
    #[account(
        init,
        seeds = [
            LOCKED_LIQUIDITY_SEED.as_bytes(),
            fee_nft_mint.key().as_ref(),
        ],
        bump,
        payer = payer,
        space = LockedCpLiquidityState::LEN
    )]
    pub locked_liquidity: Box<Account<'info, LockedCpLiquidityState>>,

    /// The mint of liquidity token
    /// address = pool_state.lp_mint
    #[account(mut)]
    pub lp_mint: Box<InterfaceAccount<'info, Mint>>,

    /// liquidity owner lp token account
    #[account(
        mut,
        token::mint = lp_mint,
        token::authority = liquidity_owner,
    )]
    pub liquidity_owner_lp: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Locked lp token deposit to
    #[account(
        init_if_needed,
        associated_token::mint = lp_mint,
        associated_token::authority = authority,
        payer = payer,
        token::token_program = token_program,
    )]
    pub locked_lp_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_0
    /// address = pool_state.token_0_vault
    #[account(mut)]
    pub token_0_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_1
    /// address = pool_state.token_1_vault
    #[account(mut)]
    pub token_1_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// To store metaplex metadata
    /// CHECK: Safety check performed inside function body
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,

    /// Sysvar for token mint and ATA creation
    pub rent: Sysvar<'info, Rent>,

    /// Program to create the new account
    pub system_program: Program<'info, System>,

    /// Program to create/transfer mint/token account
    pub token_program: Program<'info, Token>,

    /// Program to create an ATA for receiving fee NFT
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// Program to create NFT metadata accunt
    /// CHECK: Metadata program address constraint applied
    pub metadata_program: Program<'info, Metadata>,
}

#[derive(Accounts)]
pub struct CollectCpFee<'info> {
    /// CHECK: the authority of token vault that cp is locked
    #[account(
        seeds = [
            crate::LOCK_CP_AUTH_SEED.as_bytes(),
        ],
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

    /// Fee nft owner who is allowed to receive fees
    pub fee_nft_owner: Signer<'info>,

    /// Fee token account
    #[account(
        token::mint = locked_liquidity.fee_nft_mint,
        token::authority = fee_nft_owner,
        constraint = fee_nft_account.amount == 1
    )]
    pub fee_nft_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Store the locked the information of liquidity
    #[account(
        mut,
        constraint = locked_liquidity.fee_nft_mint == fee_nft_account.mint
    )]
    pub locked_liquidity: Account<'info, LockedCpLiquidityState>,

    /// cpmm program
    pub cpmm_program: Program<'info, RaydiumCpmm>,

    /// CHECK: cp program vault and lp mint authority
    #[account(
        seeds = [
            raydium_cpmm_cpi::AUTH_SEED.as_bytes(),
        ],
        bump,
        seeds::program = cpmm_program.key(),
    )]
    pub cp_authority: UncheckedAccount<'info>,

    /// CHECK: CPMM Pool state account
    #[account(
        mut,
        address = locked_liquidity.pool_id
    )]
    pub pool_state: UncheckedAccount<'info>,

    /// lp mint
    /// address = pool_state.lp_mint
    #[account(mut)]
    pub lp_mint: Box<InterfaceAccount<'info, Mint>>,

    /// The token account for receive token_0
    #[account(
        mut,
        token::mint = token_0_vault.mint,
    )]
    pub recipient_token_0_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The token account for receive token_1
    #[account(
        mut,
        token::mint = token_1_vault.mint,
    )]
    pub recipient_token_1_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_0
    /// address = pool_state.token_0_vault
    #[account(mut)]
    pub token_0_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_1
    /// address = pool_state.token_1_vault
    #[account(mut)]
    pub token_1_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The mint of token_0 vault
    #[account(
        address = token_0_vault.mint
    )]
    pub vault_0_mint: Box<InterfaceAccount<'info, Mint>>,

    /// The mint of token_1 vault
    #[account(
        address = token_1_vault.mint
    )]
    pub vault_1_mint: Box<InterfaceAccount<'info, Mint>>,

    /// locked lp token account
    #[account(
        mut,
        associated_token::mint = lp_mint,
        associated_token::authority = authority,
        token::token_program = token_program,
    )]
    pub locked_lp_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// token Program
    pub token_program: Program<'info, Token>,

    /// Token program 2022
    pub token_program_2022: Program<'info, Token2022>,

    /// memo program
    #[account()]
    pub memo_program: Program<'info, Memo>,
}
