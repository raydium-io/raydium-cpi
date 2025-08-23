use super::states::*;
use anchor_lang::{accounts::interface_account::InterfaceAccount, prelude::*};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token::spl_token,
    token::Token,
    token_2022::{self, spl_token_2022, Token2022},
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use std::ops::{Deref, DerefMut};

/// Accounts required for initializing a new trading pool
#[event_cpi]
#[derive(Accounts)]
#[instruction(base_mint_param: MintParams)]
pub struct Initialize<'info> {
    /// The account paying for the initialization costs
    /// This can be any account with sufficient SOL to cover the transaction
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: The creator of base token
    #[account()]
    pub creator: UncheckedAccount<'info>,

    /// Global configuration account containing protocol-wide settings
    /// Includes settings like quote token mint and fee parameters
    pub global_config: Box<Account<'info, GlobalConfig>>,

    /// Platform configuration account containing platform info
    /// Includes settings like the fee_rate, name, web, img of the platform
    pub platform_config: Box<Account<'info, PlatformConfig>>,

    /// PDA that acts as the authority for pool vault and mint operations
    /// Generated using AUTH_SEED
    /// CHECK: This is a PDA, safety checks performed in seeds constraint
    #[account(
        seeds = [
            crate::AUTH_SEED.as_bytes(),
        ],
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

    /// Account that stores the pool's state and parameters
    /// PDA generated using POOL_SEED and both token mints
    #[account(
        init,
        seeds = [
            POOL_SEED.as_bytes(),
            base_mint.key().as_ref(),
            quote_mint.key().as_ref()
        ],
        bump,
        payer = payer,
        space = PoolState::LEN
    )]
    pub pool_state: Box<Account<'info, PoolState>>,

    /// The mint for the base token (token being sold)
    /// Created in this instruction with specified decimals
    #[account(
        init,
        mint::decimals = base_mint_param.decimals,
        mint::authority = authority,
        payer = payer,
        mint::token_program = base_token_program,
    )]
    pub base_mint: Box<InterfaceAccount<'info, Mint>>,

    /// The mint for the quote token (token used to buy)
    /// Must match the quote_mint specified in global config
    #[account(
        address = global_config.quote_mint,
        mint::token_program = quote_token_program
    )]
    pub quote_mint: Box<InterfaceAccount<'info, Mint>>,

    /// Token account that holds the pool's base tokens
    /// PDA generated using POOL_VAULT_SEED
    /// CHECK: Initialized in this instruction
    #[account(
        init,
        seeds =[
            POOL_VAULT_SEED.as_bytes(),
            pool_state.key().as_ref(),
            base_mint.key().as_ref(),
        ],
        bump,
        payer = payer,
        token::mint = base_mint,
        token::authority = authority,
        token::token_program = base_token_program,
    )]
    pub base_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Token account that holds the pool's quote tokens
    /// PDA generated using POOL_VAULT_SEED
    /// CHECK: Initialized in this instruction
    #[account(
        init,
        seeds =[
            POOL_VAULT_SEED.as_bytes(),
            pool_state.key().as_ref(),
            quote_mint.key().as_ref(),
        ],
        bump,
        payer = payer,
        token::mint = quote_mint,
        token::authority = authority,
        token::token_program = quote_token_program,
    )]
    pub quote_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Account to store the base token's metadata
    /// Created using Metaplex metadata program
    /// CHECK: Safety check performed inside metadata program   
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,

    /// SPL Token program for the base token
    /// Must be the standard Token program
    #[account(
        address = spl_token::id()
    )]
    pub base_token_program: Interface<'info, TokenInterface>,

    /// SPL Token program for the quote token
    pub quote_token_program: Program<'info, Token>,

    /// Metaplex Token Metadata program
    /// Used to create metadata for the base token
    pub metadata_program: Program<'info, Metadata>,

    /// Required for account creation
    pub system_program: Program<'info, System>,

    /// Required for rent exempt calculations
    pub rent_program: Sysvar<'info, Rent>,
    // CHECK: Only the event authority can invoke self-CPI
    // #[account(seeds = [b"__event_authority"], bump)]
    // pub event_authority: AccountInfo<'info>,
    // CHECK: Self-CPI will fail if the program is not the current program
    // pub program: AccountInfo<'info>,
}

/// Accounts required for initializing a new trading pool
#[event_cpi]
#[derive(Accounts)]
#[instruction(base_mint_param: MintParams)]
pub struct InitializeV2<'info> {
    /// The account paying for the initialization costs
    /// This can be any account with sufficient SOL to cover the transaction
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: The creator of base token
    #[account()]
    pub creator: UncheckedAccount<'info>,

    /// Global configuration account containing protocol-wide settings
    /// Includes settings like quote token mint and fee parameters
    pub global_config: Box<Account<'info, GlobalConfig>>,

    /// Platform configuration account containing platform info
    /// Includes settings like the fee_rate, name, web, img of the platform
    pub platform_config: Box<Account<'info, PlatformConfig>>,

    /// PDA that acts as the authority for pool vault and mint operations
    /// Generated using AUTH_SEED
    /// CHECK: This is a PDA, safety checks performed in seeds constraint
    #[account(
        seeds = [
            crate::AUTH_SEED.as_bytes(),
        ],
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

    /// Account that stores the pool's state and parameters
    /// PDA generated using POOL_SEED and both token mints
    #[account(
        init,
        seeds = [
            POOL_SEED.as_bytes(),
            base_mint.key().as_ref(),
            quote_mint.key().as_ref()
        ],
        bump,
        payer = payer,
        space = PoolState::LEN
    )]
    pub pool_state: Box<Account<'info, PoolState>>,

    /// The mint for the base token (token being sold)
    /// Created in this instruction with specified decimals
    #[account(
        init,
        mint::decimals = base_mint_param.decimals,
        mint::authority = authority,
        payer = payer,
        mint::token_program = base_token_program,
    )]
    pub base_mint: Box<InterfaceAccount<'info, Mint>>,

    /// The mint for the quote token (token used to buy)
    /// Must match the quote_mint specified in global config
    #[account(
        address = global_config.quote_mint,
        mint::token_program = quote_token_program
    )]
    pub quote_mint: Box<InterfaceAccount<'info, Mint>>,

    /// Token account that holds the pool's base tokens
    /// PDA generated using POOL_VAULT_SEED
    /// CHECK: Initialized in this instruction
    #[account(
        init,
        seeds =[
            POOL_VAULT_SEED.as_bytes(),
            pool_state.key().as_ref(),
            base_mint.key().as_ref(),
        ],
        bump,
        payer = payer,
        token::mint = base_mint,
        token::authority = authority,
        token::token_program = base_token_program,
    )]
    pub base_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Token account that holds the pool's quote tokens
    /// PDA generated using POOL_VAULT_SEED
    /// CHECK: Initialized in this instruction
    #[account(
        init,
        seeds =[
            POOL_VAULT_SEED.as_bytes(),
            pool_state.key().as_ref(),
            quote_mint.key().as_ref(),
        ],
        bump,
        payer = payer,
        token::mint = quote_mint,
        token::authority = authority,
        token::token_program = quote_token_program,
    )]
    pub quote_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Account to store the base token's metadata
    /// Created using Metaplex metadata program
    /// CHECK: Safety check performed inside metadata program   
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,

    /// SPL Token program for the base token
    /// Must be the standard Token program
    #[account(
        address = spl_token::id()
    )]
    pub base_token_program: Interface<'info, TokenInterface>,

    /// SPL Token program for the quote token
    pub quote_token_program: Program<'info, Token>,

    /// Metaplex Token Metadata program
    /// Used to create metadata for the base token
    pub metadata_program: Program<'info, Metadata>,

    /// Required for account creation
    pub system_program: Program<'info, System>,

    /// Required for rent exempt calculations
    pub rent_program: Sysvar<'info, Rent>,
    // CHECK: Only the event authority can invoke self-CPI
    // #[account(seeds = [b"__event_authority"], bump)]
    // pub event_authority: AccountInfo<'info>,
    // CHECK: Self-CPI will fail if the program is not the current program
    // pub program: AccountInfo<'info>,
}

/// Accounts required for initializing a new trading pool with base token belongs to spl-token-2022
#[event_cpi]
#[derive(Accounts)]
#[instruction(base_mint_param: MintParams)]
pub struct InitializeWithToken2022<'info> {
    /// The account paying for the initialization costs
    /// This can be any account with sufficient SOL to cover the transaction
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: The creator of base token
    #[account()]
    pub creator: UncheckedAccount<'info>,

    /// Global configuration account containing protocol-wide settings
    /// Includes settings like quote token mint and fee parameters
    pub global_config: Box<Account<'info, GlobalConfig>>,

    /// Platform configuration account containing platform info
    /// Includes settings like the fee_rate, name, web, img of the platform
    pub platform_config: Box<Account<'info, PlatformConfig>>,

    /// PDA that acts as the authority for pool vault and mint operations
    /// Generated using AUTH_SEED
    /// CHECK: This is a PDA, safety checks performed in seeds constraint
    #[account(
        seeds = [
            crate::AUTH_SEED.as_bytes(),
        ],
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

    /// Account that stores the pool's state and parameters
    /// PDA generated using POOL_SEED and both token mints
    #[account(
        init,
        seeds = [
            POOL_SEED.as_bytes(),
            base_mint.key().as_ref(),
            quote_mint.key().as_ref()
        ],
        bump,
        payer = payer,
        space = PoolState::LEN
    )]
    pub pool_state: Box<Account<'info, PoolState>>,

    /// The mint for the base token (token being sold)
    /// Created in this instruction with specified decimals
    #[account(mut)]
    pub base_mint: Signer<'info>,

    /// The mint for the quote token (token used to buy)
    /// Must match the quote_mint specified in global config
    #[account(
        address = global_config.quote_mint,
        mint::token_program = quote_token_program
    )]
    pub quote_mint: Box<InterfaceAccount<'info, Mint>>,

    /// Token account that holds the pool's base tokens
    /// PDA generated using POOL_VAULT_SEED
    /// CHECK: Initialized in this instruction
    #[account(
        mut,
        seeds =[
            POOL_VAULT_SEED.as_bytes(),
            pool_state.key().as_ref(),
            base_mint.key().as_ref(),
        ],
        bump,
    )]
    pub base_vault: UncheckedAccount<'info>,

    /// Token account that holds the pool's quote tokens
    /// PDA generated using POOL_VAULT_SEED
    /// CHECK: Initialized in this instruction
    #[account(
        init,
        seeds =[
            POOL_VAULT_SEED.as_bytes(),
            pool_state.key().as_ref(),
            quote_mint.key().as_ref(),
        ],
        bump,
        payer = payer,
        token::mint = quote_mint,
        token::authority = authority,
        token::token_program = quote_token_program,
    )]
    pub quote_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// SPL Token program for the base token
    pub base_token_program: Program<'info, Token2022>,

    /// SPL Token program for the quote token
    pub quote_token_program: Program<'info, Token>,

    /// Required for account creation
    pub system_program: Program<'info, System>,
    // CHECK: Only the event authority can invoke self-CPI
    // #[account(seeds = [b"__event_authority"], bump)]
    // pub event_authority: AccountInfo<'info>,
    // CHECK: Self-CPI will fail if the program is not the current program
    // pub program: AccountInfo<'info>,
}

/// Accounts required for performing a buy operation in the pool
/// Buy means trading quote tokens for base tokens
#[event_cpi]
#[derive(Accounts)]
pub struct Swap<'info> {
    /// The user performing the swap operation
    /// Must sign the transaction and pay for fees
    #[account(mut)]
    pub payer: Signer<'info>,

    /// PDA that acts as the authority for pool vault operations
    /// Generated using AUTH_SEED
    /// CHECK: Safety checks performed in seeds constraint
    #[account(
        seeds = [
            crate::AUTH_SEED.as_bytes(),
        ],
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

    /// Global configuration account containing protocol-wide settings
    /// Used to read protocol fee rates and curve type
    #[account(
        address = pool_state.global_config
    )]
    pub global_config: Box<Account<'info, GlobalConfig>>,

    /// Platform configuration account containing platform-wide settings
    /// Used to read platform fee rate
    #[account(
        address = pool_state.platform_config
    )]
    pub platform_config: Box<Account<'info, PlatformConfig>>,

    /// The pool state account where the swap will be performed
    /// Contains current pool parameters and balances
    #[account(mut)]
    pub pool_state: Box<Account<'info, PoolState>>,

    /// The user's token account for base tokens (tokens being bought)
    /// Will receive the output tokens after the swap
    #[account(
        mut,
        token::mint = base_token_mint,
    )]
    pub user_base_token: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The user's token account for quote tokens (tokens being sold)
    /// Will be debited for the input amount
    #[account(
        mut,
        token::mint = quote_token_mint,
    )]
    pub user_quote_token: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The pool's vault for base tokens
    /// Will be debited to send tokens to the user
    #[account(
        mut,
        address = pool_state.base_vault
    )]
    pub base_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The pool's vault for quote tokens
    /// Will receive the input tokens from the user
    #[account(
        mut,
        address = pool_state.quote_vault
    )]
    pub quote_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The mint of the base token
    /// Used for transfer fee calculations if applicable
    #[account()]
    pub base_token_mint: Box<InterfaceAccount<'info, Mint>>,

    /// The mint of the quote token
    #[account()]
    pub quote_token_mint: Box<InterfaceAccount<'info, Mint>>,

    /// SPL Token program for base token transfers
    pub base_token_program: Interface<'info, TokenInterface>,

    /// SPL Token program for quote token transfers
    pub quote_token_program: Program<'info, Token>,
    // CHECK: Only the event authority can invoke self-CPI
    // #[account(seeds = [b"__event_authority"], bump)]
    // pub event_authority: AccountInfo<'info>,
    // CHECK: Self-CPI will fail if the program is not the current program
    // pub program: AccountInfo<'info>,

    // Additional remaining accounts:

    // if share_fee_rate > 0, the following account is required
    // share_fee_receiver

    // if unix_timestamp > get_upgrade_timestamp(), the following three accounts are required
    // system_program

    //  seeds = [
    //     platform_config,
    //     quote_token_mint
    // ],
    // platform_fee_vault

    //  seeds = [
    //     creator,
    //     quote_token_mint
    // ],
    // creator_fee_vault
}

#[derive(Accounts)]
pub struct CreateVestingAccount<'info> {
    /// The account paying for the initialization costs
    /// This can be any account with sufficient SOL to cover the transaction    
    #[account(
        mut,
        address = pool_state.creator
    )]
    pub creator: Signer<'info>,

    /// CHECK:The beneficiary of the vesting account
    /// The beneficiary is used to receive the allocated linear release of tokens.
    /// Once this account is set, it cannot be modified, so please ensure the validity of this account,
    /// otherwise, the unlocked tokens will not be claimable.
    #[account(mut)]
    pub beneficiary: UncheckedAccount<'info>,

    /// The pool state account
    #[account(mut)]
    pub pool_state: Account<'info, PoolState>,

    /// The vesting record account
    #[account(
        init,
        seeds =[
            POOL_VESTING_SEED.as_bytes(),
            pool_state.key().as_ref(),
            beneficiary.key().as_ref(),
        ],
        bump,
        payer = creator,
        space = VestingRecord::LEN,
    )]
    pub vesting_record: Account<'info, VestingRecord>,

    /// Required for account creation
    pub system_program: Program<'info, System>,
}

/// Accounts required for initializing a new trading pool
#[derive(Accounts)]
pub struct ClaimVestedToken<'info> {
    /// The beneficiary of the vesting account
    #[account(
        mut,
        address = vesting_record.beneficiary
    )]
    pub beneficiary: Signer<'info>,

    /// PDA that acts as the authority for pool vault and mint operations
    /// Generated using AUTH_SEED
    /// CHECK: This is a PDA, safety checks performed in seeds constraint
    #[account(
        seeds = [
            crate::AUTH_SEED.as_bytes(),
        ],
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

    /// Account that stores the pool's state and parameters
    /// PDA generated using POOL_SEED and both token mints
    #[account(
        mut,
        address = vesting_record.pool
    )]
    pub pool_state: Box<Account<'info, PoolState>>,

    /// The vesting record account
    #[account(
        mut,
        seeds =[
            POOL_VESTING_SEED.as_bytes(),
            pool_state.key().as_ref(),
            beneficiary.key().as_ref(),
        ],
        bump,
    )]
    pub vesting_record: Box<Account<'info, VestingRecord>>,

    /// The pool's vault for base tokens
    /// Will be debited to send tokens to the user
    #[account(
        mut,
        address = pool_state.base_vault
    )]
    pub base_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
       init_if_needed,
       associated_token::mint = base_token_mint,
       associated_token::authority = beneficiary,
       associated_token::token_program = base_token_program,
       payer = beneficiary,
    )]
    pub user_base_token: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The mint for the base token (token being sold)
    /// Created in this instruction with specified decimals
    #[account(
        mint::token_program = base_token_program,
        address = base_vault.mint
    )]
    pub base_token_mint: Box<InterfaceAccount<'info, Mint>>,

    /// SPL Token program for the base token
    /// Must be the standard Token program
    #[account(
        address = spl_token::id()
    )]
    pub base_token_program: Interface<'info, TokenInterface>,

    /// Required for account creation
    pub system_program: Program<'info, System>,

    /// Required for associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct CreatePlatformConfig<'info> {
    /// The account paying for the initialization costs  
    #[account(mut)]
    pub platform_admin: Signer<'info>,

    /// CHECK:The wallet for the platform to receive platform fee
    #[account()]
    pub platform_fee_wallet: UncheckedAccount<'info>,

    /// CHECK:The wallet for the platform to receive migrate liquidity nft(Only support cpswap program)
    #[account()]
    pub platform_nft_wallet: UncheckedAccount<'info>,

    /// The platform config account
    #[account(
        init,
        seeds =[
            PLATFORM_CONFIG_SEED.as_bytes(),
            platform_admin.key().as_ref(),
        ],
        bump,
        payer = platform_admin,
        space = PlatformConfig::MIN_LEN,
    )]
    pub platform_config: Account<'info, PlatformConfig>,

    /// CHECK:CpSwap config Account.
    #[account(
        owner = crate::cpswap_program::ID,
    )]
    pub cpswap_config: UncheckedAccount<'info>,

    /// Required for account creation
    pub system_program: Program<'info, System>,

    /// CHECK: Support the creation of token2022's token and the extension of transferFeeConfig.
    #[account()]
    pub transfer_fee_extension_authority: UncheckedAccount<'info>,
}

/// Accounts required for claim platform fee
#[derive(Accounts)]
pub struct ClaimPlatformFee<'info> {
    /// Only the wallet stored in platform_config can collect platform fees
    #[account(
        mut,
        address = platform_config.platform_fee_wallet 
    )]
    pub platform_fee_wallet: Signer<'info>,

    /// PDA that acts as the authority for pool vault and mint operations
    /// Generated using AUTH_SEED
    /// CHECK: This is a PDA, safety checks performed in seeds constraint
    #[account(
        seeds = [
            crate::AUTH_SEED.as_bytes(),
        ],
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

    /// Account that stores the pool's state and parameters
    /// PDA generated using POOL_SEED and both token mints
    #[account(mut)]
    pub pool_state: Box<Account<'info, PoolState>>,

    /// The platform config account
    #[account(
        address = pool_state.platform_config
    )]
    pub platform_config: Account<'info, PlatformConfig>,

    //// The address that holds pool tokens for quote token
    #[account(
        mut,
        address = pool_state.quote_vault
    )]
    pub quote_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that receives the collected quote token fees
    #[account(
       init_if_needed,
       associated_token::mint = quote_mint,
       associated_token::authority = platform_fee_wallet,
       associated_token::token_program = token_program,
       payer = platform_fee_wallet,
    )]
    pub recipient_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The mint of quote token vault
    #[account(
        address = quote_vault.mint
    )]
    pub quote_mint: Box<InterfaceAccount<'info, Mint>>,

    /// SPL program for input token transfers
    pub token_program: Program<'info, Token>,

    /// Required for account creation
    pub system_program: Program<'info, System>,

    /// Required for associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,
}

/// Accounts required for claim platform fee
#[derive(Accounts)]
pub struct ClaimPlatformFeeFromVault<'info> {
    /// Only the wallet stored in platform_config can collect platform fees
    #[account(
        mut,
        address = platform_config.platform_fee_wallet
    )]
    pub platform_fee_wallet: Signer<'info>,

    /// CHECK: PDA that acts as the authority for platform fee vault
    #[account(
        seeds = [
            crate::PLATFORM_FEE_VAULT_AUTH_SEED.as_bytes(),
        ],
        bump,
    )]
    pub fee_vault_authority: UncheckedAccount<'info>,

    /// The platform config account
    pub platform_config: Account<'info, PlatformConfig>,

    /// The platform fee vault
    #[account(
        mut,
        seeds = [
            platform_config.key().as_ref(),
            quote_mint.key().as_ref()
        ],
        bump,
        token::token_program = token_program,
    )]
    pub platform_fee_vault: InterfaceAccount<'info, TokenAccount>,

    /// The address that receives the collected quote token fees
    #[account(
       init_if_needed,
       associated_token::mint = quote_mint,
       associated_token::authority = platform_fee_wallet,
       associated_token::token_program = token_program,
       payer = platform_fee_wallet,
    )]
    pub recipient_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The mint of quote token vault
    #[account(
        token::token_program = token_program,
    )]
    pub quote_mint: Box<InterfaceAccount<'info, Mint>>,

    /// SPL program for input token transfers
    pub token_program: Program<'info, Token>,

    /// Required for account creation
    pub system_program: Program<'info, System>,

    /// Required for associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct UpdatePlatformConfig<'info> {
    /// The account paying for the initialization costs  
    pub platform_admin: Signer<'info>,

    /// Platform config account to be changed
    #[account(
        mut,
        seeds =[
            PLATFORM_CONFIG_SEED.as_bytes(),
            platform_admin.key().as_ref(),
        ],
        bump,
    )]
    pub platform_config: Account<'info, PlatformConfig>,
    // maybe remaining_accounts need:
    // #[account(
    //     owner = crate::cpswap_program::ID,
    // )]
    // pub cpswap_config: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct UpdatePlatformCurveParam<'info> {
    /// The account paying for the initialization costs  
    #[account(mut)]
    pub platform_admin: Signer<'info>,

    /// Platform config account to be changed
    #[account(
        mut,
        seeds =[
            PLATFORM_CONFIG_SEED.as_bytes(),
            platform_admin.key().as_ref(),
        ],
        bump,
    )]
    pub platform_config: Box<Account<'info, PlatformConfig>>,

    /// Global configuration account containing protocol-wide settings
    /// Includes settings like quote token mint and fee parameters
    #[account()]
    pub global_config: Box<Account<'info, GlobalConfig>>,

    /// System program for lamport transfers
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RemovePlatformCurveParam<'info> {
    /// The account paying for the initialization costs  
    #[account()]
    pub platform_admin: Signer<'info>,

    /// Platform config account to be changed
    #[account(
        mut,
        seeds =[
            PLATFORM_CONFIG_SEED.as_bytes(),
            platform_admin.key().as_ref(),
        ],
        bump,
    )]
    pub platform_config: Box<Account<'info, PlatformConfig>>,
}

#[derive(Accounts)]
pub struct ClaimCreatorFee<'info> {
    /// The pool creator
    #[account(mut)]
    pub creator: Signer<'info>,

    /// CHECK: PDA that acts as the authority for creator fee vault
    #[account(
        seeds = [
            crate::CREATOR_FEE_VAULT_AUTH_SEED.as_bytes(),
        ],
        bump,
    )]
    pub fee_vault_authority: UncheckedAccount<'info>,

    /// The creator fee vault
    #[account(
        mut,
        seeds = [
            creator.key().as_ref(),
            quote_mint.key().as_ref()
        ],
        bump,
        token::token_program = token_program,
        token::mint = quote_mint,
    )]
    pub creator_fee_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
       init_if_needed,
       associated_token::mint = quote_mint,
       associated_token::authority = creator,
       associated_token::token_program = token_program,
       payer = creator,
    )]
    pub recipient_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The mint for the quote token
    #[account(
        mint::token_program = token_program,
    )]
    pub quote_mint: Box<InterfaceAccount<'info, Mint>>,

    /// SPL Token program for the quote token
    pub token_program: Program<'info, Token>,

    /// Required for account creation
    pub system_program: Program<'info, System>,

    /// Required for associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,
}
