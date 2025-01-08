#![allow(unused)]

pub mod context;
pub mod states;
pub use context::*;

use anchor_lang::prelude::*;

#[cfg(feature = "devnet")]
declare_id!("DLockwT7X7sxtLmGH9g5kmfcjaBtncdbUmi738m5bvQC");
#[cfg(not(feature = "devnet"))]
declare_id!("LockrWmn6K5twhz3y9w1dQERbmgSaRkfnTeTKbpofwE");

pub const LOCK_CLMM_AUTH_SEED: &str = "program_authority_seed";
pub const LOCK_CP_AUTH_SEED: &str = "lock_cp_authority_seed";

#[program]
pub mod raydium_liquidity_locking {
    use super::*;

    /// Lock an existing clmm's position
    ///
    /// # Arguments
    ///
    /// * `ctx` -  The context of accounts
    /// * `with_metadata` -  Create NFT with metadata or not
    ///
    pub fn lock_clmm_position<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, LockClmmPosition<'info>>,
        with_metadata: bool,
    ) -> Result<()> {
        Ok(())
    }

    /// collect clmm locked postion fees and rewards
    /// # Arguments
    ///
    /// * `ctx` -  The context of accounts
    ///
    pub fn collect_clmm_fees_and_rewards<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, CollectClmmFeeAndReward<'info>>,
    ) -> Result<()> {
        Ok(())
    }

    /// Lock cpSwap liquidity and mint a nft to collect locked liquidity's fee.
    ///
    /// # Arguments
    ///
    /// * `ctx` -  The context of accounts
    /// * `lp_amount` -  The lp amount to lock
    /// * `with_metadata` -  Create NFT with metadata or not
    ///
    pub fn lock_cp_liquidity<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, LockCpLiquidity<'info>>,
        lp_amount: u64,
        with_metadata: bool,
    ) -> Result<()> {
        Ok(())
    }

    /// collect cpSwap locked liquidity's fees
    /// # Arguments
    ///
    /// * `ctx` -  The context of accounts
    /// * `fee_lp_amount` -  The amount lp want to claim, please fill in u64::Max.
    ///
    pub fn collect_cp_fees<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, CollectCpFee<'info>>,
        fee_lp_amount: u64,
    ) -> Result<()> {
        Ok(())
    }
}
