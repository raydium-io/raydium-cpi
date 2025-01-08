use anchor_lang::prelude::*;

// Seed for LockedCpLiquidityState account
pub const LOCKED_LIQUIDITY_SEED: &str = "locked_liquidity";
// Seed for LockedClmmPositionState account
pub const LOCKED_POSITION_SEED: &str = "locked_position";
#[account]
#[derive(Default, Debug)]
pub struct LockedCpLiquidityState {
    /// The Locked liquidity amount without claimed lp fee
    pub locked_lp_amount: u64,
    /// Claimed lp fee amount
    pub claimed_lp_amount: u64,
    /// Unclaimed lp fee amount
    pub unclaimed_lp_amount: u64,
    /// Last updated cp pool lp total supply
    pub last_lp: u64,
    /// Last updated cp pool k
    pub last_k: u128,
    /// Account update recent epoch
    pub recent_epoch: u64,
    /// The ID of the pool with which this record is connected
    pub pool_id: Pubkey,
    /// nft mint to check who has authority to collect fee
    pub fee_nft_mint: Pubkey,
    /// The owner who has locked liquidity
    pub locked_owner: Pubkey,
    /// The mint of locked lp token
    pub locked_lp_mint: Pubkey,
    /// Unused bytes for future upgrades.
    pub padding: [u64; 8],
}

impl LockedCpLiquidityState {
    pub const LEN: usize = 8 + 4 * 8 + 16 + 8 + 32 * 4 + 8 * 8;
}

#[account]
#[derive(Default, Debug)]
pub struct LockedClmmPositionState {
    /// Bump to identify PDA
    pub bump: [u8; 1],
    /// The owner who has locked clmm NFT
    pub position_owner: Pubkey,
    /// The ID of the pool with which this record is connected
    pub pool_id: Pubkey,
    /// The ID of the position with which this record is connected
    pub position_id: Pubkey,
    /// Program ATA locked NFT account or user ATA position NFT account
    pub locked_nft_account: Pubkey,
    /// nft mint to check who has authority to collect fee
    pub fee_nft_mint: Pubkey,
    /// account update recent epoch
    pub recent_epoch: u64,
    /// Unused bytes for future upgrades.
    pub padding: [u64; 8],
}

impl LockedClmmPositionState {
    pub const LEN: usize = 8 + 1 + 32 * 5 + 8 + 8 * 8;
}
