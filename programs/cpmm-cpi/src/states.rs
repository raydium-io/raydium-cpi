use anchor_lang::prelude::*;
/// Seed to derive account address and signature
pub const POOL_SEED: &str = "pool";
pub const POOL_LP_MINT_SEED: &str = "pool_lp_mint";
pub const POOL_VAULT_SEED: &str = "pool_vault";
pub const OBSERVATION_SEED: &str = "observation";
pub const AMM_CONFIG_SEED: &str = "amm_config";
pub const PERMISSION_SEED: &str = "permission";

// Number of ObservationState element
pub const OBSERVATION_NUM: usize = 100;
pub const OBSERVATION_UPDATE_DURATION_DEFAULT: u64 = 15;

pub const Q32: u128 = (u32::MAX as u128) + 1; // 2^32

/// Holds the current owner of the factory
#[account]
#[derive(Default, Debug)]
pub struct AmmConfig {
    /// Bump to identify PDA
    pub bump: u8,
    /// Status to control if new pool can be create
    pub disable_create_pool: bool,
    /// Config index
    pub index: u16,
    /// The trade fee, denominated in hundredths of a bip (10^-6)
    pub trade_fee_rate: u64,
    /// The protocol fee
    pub protocol_fee_rate: u64,
    /// The fund fee, denominated in hundredths of a bip (10^-6)
    pub fund_fee_rate: u64,
    /// Fee for create a new pool
    pub create_pool_fee: u64,
    /// Address of the protocol fee owner
    pub protocol_owner: Pubkey,
    /// Address of the fund fee owner
    pub fund_owner: Pubkey,
    /// The pool creator fee, denominated in hundredths of a bip (10^-6)
    pub creator_fee_rate: u64,
    /// padding
    pub padding: [u64; 15],
}

pub enum PoolStatusBitIndex {
    Deposit,
    Withdraw,
    Swap,
}

#[derive(PartialEq, Eq)]
pub enum PoolStatusBitFlag {
    Enable,
    Disable,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum CreatorFeeOn {
    /// Both token0 and token1 can be used as trade fees.
    /// It depends on what the input token is.
    BothToken,
    /// Only token0 can be used as trade fees.
    OnlyToken0,
    /// Only token1 can be used as trade fees.
    OnlyToken1,
}

/// The direction of a trade, since curves can be specialized to treat each
/// token differently (by adding offsets or weights)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TradeDirection {
    /// Input token 0, output token 1
    ZeroForOne,
    /// Input token 1, output token 0
    OneForZero,
}
pub struct SwapParams {
    pub trade_direction: TradeDirection,
    pub total_input_token_amount: u64,
    pub total_output_token_amount: u64,
    pub token_0_price_x64: u128,
    pub token_1_price_x64: u128,
    pub is_creator_fee_on_input: bool,
}

#[account(zero_copy(unsafe))]
#[repr(C, packed)]
#[derive(Default, Debug)]
pub struct PoolState {
    /// Which config the pool belongs
    pub amm_config: Pubkey,
    /// pool creator
    pub pool_creator: Pubkey,
    /// Token A
    pub token_0_vault: Pubkey,
    /// Token B
    pub token_1_vault: Pubkey,

    /// Pool tokens are issued when A or B tokens are deposited.
    /// Pool tokens can be withdrawn back to the original A or B token.
    pub lp_mint: Pubkey,
    /// Mint information for token A
    pub token_0_mint: Pubkey,
    /// Mint information for token B
    pub token_1_mint: Pubkey,

    /// token_0 program
    pub token_0_program: Pubkey,
    /// token_1 program
    pub token_1_program: Pubkey,

    /// observation account to store oracle data
    pub observation_key: Pubkey,

    pub auth_bump: u8,
    /// Bitwise representation of the state of the pool
    /// bit0, 1: disable deposit(value is 1), 0: normal
    /// bit1, 1: disable withdraw(value is 2), 0: normal
    /// bit2, 1: disable swap(value is 4), 0: normal
    pub status: u8,

    pub lp_mint_decimals: u8,
    /// mint0 and mint1 decimals
    pub mint_0_decimals: u8,
    pub mint_1_decimals: u8,

    /// True circulating supply without burns and lock ups
    pub lp_supply: u64,
    /// The amounts of token_0 and token_1 that are owed to the liquidity provider.
    pub protocol_fees_token_0: u64,
    pub protocol_fees_token_1: u64,

    pub fund_fees_token_0: u64,
    pub fund_fees_token_1: u64,

    /// The timestamp allowed for swap in the pool.
    pub open_time: u64,
    /// recent epoch
    pub recent_epoch: u64,

    /// Creator fee collect mode
    /// 0: both token_0 and token_1 can be used as trade fees. It depends on what the input token is when swapping
    /// 1: only token_0 as trade fee
    /// 2: only token_1 as trade fee
    pub creator_fee_on: u8,
    pub enable_creator_fee: bool,
    pub padding1: [u8; 6],
    pub creator_fees_token_0: u64,
    pub creator_fees_token_1: u64,
    /// padding for future updates
    pub padding: [u64; 28],
}
impl PoolState {
    pub const LEN: usize = 8 + 10 * 32 + 1 * 5 + 8 * 7 + 1 * 2 + 6 * 1 + 2 * 8 + 8 * 28;
}

/// The element of observations in ObservationState
#[zero_copy(unsafe)]
#[repr(C, packed)]
#[derive(Default, Debug)]
pub struct Observation {
    /// The block timestamp of the observation
    pub block_timestamp: u64,
    /// the cumulative of token0 price during the duration time, Q32.32, the remaining 64 bit for overflow
    pub cumulative_token_0_price_x32: u128,
    /// the cumulative of token1 price during the duration time, Q32.32, the remaining 64 bit for overflow
    pub cumulative_token_1_price_x32: u128,
}
impl Observation {
    pub const LEN: usize = 8 + 16 + 16;
}

#[account(zero_copy(unsafe))]
#[repr(C, packed)]
pub struct ObservationState {
    /// Whether the ObservationState is initialized
    pub initialized: bool,
    /// the most-recently updated index of the observations array
    pub observation_index: u16,
    pub pool_id: Pubkey,
    /// observation array
    pub observations: [Observation; OBSERVATION_NUM],
    /// padding for feature update
    pub padding: [u64; 4],
}

impl ObservationState {
    pub const LEN: usize = 8 + 1 + 2 + 32 + (Observation::LEN * OBSERVATION_NUM) + 8 * 4;
}

/// Holds the current owner of the factory
#[account]
#[derive(Default, Debug)]
pub struct Permission {
    /// authority
    pub authority: Pubkey,
    /// padding
    pub padding: [u64; 30],
}
