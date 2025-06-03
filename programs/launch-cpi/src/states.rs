use anchor_lang::prelude::*;

/// Seed to derive account address and signature
///
pub const GLOBAL_CONFIG_SEED: &str = "global_config";

pub const POOL_SEED: &str = "pool";
pub const POOL_VAULT_SEED: &str = "pool_vault";
pub const POOL_VESTING_SEED: &str = "pool_vesting";

pub const PLATFORM_CONFIG_SEED: &str = "platform_config";
pub const NAME_SIZE: usize = 64;
pub const WEB_SIZE: usize = 256;
pub const IMG_SIZE: usize = 256;

/// Holds the current owner of the factory
#[account]
#[derive(Default, Debug)]
pub struct GlobalConfig {
    /// Account update epoch
    pub epoch: u64,
    /// 0: Constant Product Curve
    /// 1: Fixed Price Curve
    /// 2: Linear Price Curve
    pub curve_type: u8,
    /// Config index
    pub index: u16,
    /// The fee of migrate to amm
    pub migrate_fee: u64,
    /// The trade fee rate, denominated in hundredths of a bip (10^-6)
    pub trade_fee_rate: u64,
    /// The maximum share fee rate, denominated in hundredths of a bip (10^-6)
    pub max_share_fee_rate: u64,
    /// The minimum base supply, the value without decimals
    pub min_base_supply: u64,
    /// The maximum lock rate, denominated in hundredths of a bip (10^-6)
    pub max_lock_rate: u64,
    /// The minimum base sell rate, denominated in hundredths of a bip (10^-6)
    pub min_base_sell_rate: u64,
    /// The minimum base migrate rate, denominated in hundredths of a bip (10^-6)
    pub min_base_migrate_rate: u64,
    /// The minimum quote fund raising, the value with decimals
    pub min_quote_fund_raising: u64,
    /// Mint information for quote token
    pub quote_mint: Pubkey,
    /// Protocol Fee owner
    pub protocol_fee_owner: Pubkey,
    /// Migrate Fee owner
    pub migrate_fee_owner: Pubkey,
    /// Migrate to amm control wallet
    pub migrate_to_amm_wallet: Pubkey,
    /// Migrate to cpswap wallet
    pub migrate_to_cpswap_wallet: Pubkey,
    /// padding for future updates
    pub padding: [u64; 16],
}

/// Represents the state of a trading pool in the protocol
/// Stores all essential information about pool balances, fees, and configuration
#[account]
#[derive(Default, Debug)]
pub struct PoolState {
    /// Account update epoch
    pub epoch: u64,

    /// Bump seed used for PDA address derivation
    pub auth_bump: u8,

    /// Current status of the pool
    /// * 0: Pool is funding
    /// * 1: Pool funding is end, waiting for migration
    /// * 2: Pool migration is done
    pub status: u8,

    /// Decimals of the pool base token
    pub base_decimals: u8,

    /// Decimals of the pool quote token
    pub quote_decimals: u8,

    /// Migrate to AMM or CpSwap
    pub migrate_type: u8,

    /// Supply of the pool base token
    pub supply: u64,

    /// Total sell amount of the base token
    pub total_base_sell: u64,

    /// For different curves, virtual_base and virtual_quote have different meanings
    /// For constant product curve, virtual_base and virtual_quote are virtual liquidity, virtual_quote/virtual_base is the initial price
    /// For linear price curve, virtual_base is the price slope parameter a, virtual_quote has no effect
    /// For fixed price curve, virtual_quote/virtual_base is the initial price
    pub virtual_base: u64,
    pub virtual_quote: u64,

    /// Actual base token amount in the pool
    /// Represents the real tokens available for trading
    pub real_base: u64,

    /// Actual quote token amount in the pool
    /// Represents the real tokens available for trading
    pub real_quote: u64,

    /// The total quote fund raising of the pool
    pub total_quote_fund_raising: u64,

    /// Accumulated trading fees in quote tokens
    /// Can be collected by the protocol fee owner
    pub quote_protocol_fee: u64,

    /// Accumulated platform fees in quote tokens
    /// Can be collected by the platform wallet stored in platform config
    pub platform_fee: u64,

    /// The fee of migrate to amm
    pub migrate_fee: u64,

    /// Vesting schedule for the base token
    pub vesting_schedule: VestingSchedule,

    /// Public key of the global configuration account
    /// Contains protocol-wide settings this pool adheres to
    pub global_config: Pubkey,

    /// Public key of the platform configuration account
    /// Contains platform-wide settings this pool adheres to
    pub platform_config: Pubkey,

    /// Public key of the base mint address
    pub base_mint: Pubkey,

    /// Public key of the quote mint address
    pub quote_mint: Pubkey,

    /// Public key of the base token vault
    /// Holds the actual base tokens owned by the pool
    pub base_vault: Pubkey,

    /// Public key of the quote token vault
    /// Holds the actual quote tokens owned by the pool
    pub quote_vault: Pubkey,

    /// The creator of base token
    pub creator: Pubkey,

    /// padding for future updates
    pub padding: [u64; 8],
}

impl PoolState {
    pub const LEN: usize = 8 + 8 + 1 * 5 + 8 * 10 + 7 * 32 + VestingSchedule::LEN + 8 * 8;
}

#[derive(Default, Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct VestingSchedule {
    // total amount of tokens to be unlocked
    pub total_locked_amount: u64,
    // Waiting time in seconds before unlocking after fundraising ends
    pub cliff_period: u64,
    // Unlocking period in seconds
    pub unlock_period: u64,
    // Start time of the vesting schedule
    pub start_time: u64,
    /// Total allocated share amount of the base token, not greater than total_locked_amount
    pub allocated_share_amount: u64,
}

impl VestingSchedule {
    pub const LEN: usize = 8 + 8 + 8 + 8 + 8;
}

#[account]
pub struct PlatformConfig {
    /// The epoch for update interval
    pub epoch: u64,
    /// The platform fee wallet
    pub platform_fee_wallet: Pubkey,
    /// The platform nft wallet to receive the platform NFT after migration if platform_scale is not 0(Only support MigrateType::CPSWAP)
    pub platform_nft_wallet: Pubkey,
    /// Scale of the platform liquidity quantity rights will be converted into NFT(Only support MigrateType::CPSWAP)
    pub platform_scale: u64,
    /// Scale of the token creator liquidity quantity rights will be converted into NFT(Only support MigrateType::CPSWAP)
    pub creator_scale: u64,
    /// Scale of liquidity directly to burn
    pub burn_scale: u64,
    /// The platform fee rate
    pub fee_rate: u64,
    /// The platform name
    pub name: [u8; NAME_SIZE],
    /// The platform website
    pub web: [u8; WEB_SIZE],
    /// The platform img link
    pub img: [u8; IMG_SIZE],
    /// The platform specifies the trade fee rate after migration to cp swap
    pub cpswap_config: Pubkey,
    /// padding for future updates
    pub padding: [u8; 224],
}

impl PlatformConfig {
    pub const LEN: usize = 8 + 8 + 32 * 2 + 8 * 4 + NAME_SIZE + WEB_SIZE + IMG_SIZE + 32 + 224;
}

#[account]
pub struct VestingRecord {
    /// Account update epoch
    pub epoch: u64,
    /// The pool state account
    pub pool: Pubkey,
    /// The beneficiary of the vesting account
    pub beneficiary: Pubkey,
    /// The amount of tokens claimed
    pub claimed_amount: u64,
    /// The share amount of the token to be vested
    pub token_share_amount: u64,
    /// padding for future updates
    pub padding: [u64; 8],
}

impl VestingRecord {
    pub const LEN: usize = 8 + 8 + 32 + 32 + 8 + 8 + 8 * 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ConstantCurve {
    pub supply: u64,
    pub total_base_sell: u64,
    pub total_quote_fund_raising: u64,
    pub migrate_type: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct FixedCurve {
    pub supply: u64,
    pub total_quote_fund_raising: u64,
    pub migrate_type: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct LinearCurve {
    pub supply: u64,
    pub total_quote_fund_raising: u64,
    pub migrate_type: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum CurveParams {
    Constant { data: ConstantCurve },
    Fixed { data: FixedCurve },
    Linear { data: LinearCurve },
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct VestingParams {
    // total amount of tokens to be unlocked
    pub total_locked_amount: u64,
    // Waiting time in seconds before unlocking after fundraising ends
    pub cliff_period: u64,
    // Unlocking period in seconds
    pub unlock_period: u64,
}

/// Represents the parameters for initializing a new token mint
/// # Fields
/// * `decimals` - Number of decimal places for the token
/// * `name` - Name of the token
/// * `symbol` - Symbol/ticker of the token
/// * `uri` - URI pointing to token metadata
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MintParams {
    pub decimals: u8,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum PlatformConfigParam {
    FeeWallet(Pubkey),
    NFTWallet(Pubkey),
    MigrateNftInfo(MigrateNftInfo),
    FeeRate(u64),
    Name(String),
    Web(String),
    Img(String),
    CpSwapConfig,
    AllInfo(PlatformConfigInfo),
}

/// Represents the parameters for initializing a platform config account(Only support MigrateType::CPSWAP)
/// # Fields
/// * `platform_scale` - Scale of the platform liquidity quantity rights will be converted into NFT
/// * `creator_scale` - Scale of the token creator liquidity quantity rights will be converted into NFT
/// * `burn_scale` - Scale of liquidity directly to burn
///
/// * platform_scale + creator_scale + burn_scale = RATE_DENOMINATOR_VALUE
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct MigrateNftInfo {
    pub platform_scale: u64,
    pub creator_scale: u64,
    pub burn_scale: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PlatformConfigInfo {
    fee_wallet: Pubkey,
    nft_wallet: Pubkey,
    migrate_nft_info: MigrateNftInfo,
    fee_rate: u64,
    name: String,
    web: String,
    img: String,
}

/// Represents the parameters for initializing a platform config account
/// # Fields
/// * `migrate_nft_info` - The platform configures liquidity info during migration(Only support MigrateType::CPSWAP)
/// * `fee_rate` - Fee rate of the platform
/// * `name` - Name of the platform
/// * `web` - Website of the platform
/// * `img` - Image link of the platform
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PlatformParams {
    pub migrate_nft_info: MigrateNftInfo,
    pub fee_rate: u64,
    pub name: String,
    pub web: String,
    pub img: String,
}
