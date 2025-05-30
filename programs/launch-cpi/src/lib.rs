#![allow(unused)]

pub mod context;
pub mod states;

pub use context::*;
pub use states::*;

use anchor_lang::prelude::*;
#[cfg(feature = "devnet")]
declare_id!("LanD8FpTBBvzZFXjTxsAoipkFsxPUCDB4qAqKxYDiNP");
#[cfg(not(feature = "devnet"))]
declare_id!("LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj");

pub mod admin {
    use super::{pubkey, Pubkey};
    #[cfg(feature = "devnet")]
    pub const ID: Pubkey = pubkey!("adMCyoCgfkg7bQiJ9aBJ59H3BXLY3r5LNLfPpQfMzBe");
    #[cfg(not(feature = "devnet"))]
    pub const ID: Pubkey = pubkey!("GThUX1Atko4tqhN2NaiTazWSeFWMuiUvfFnyJyUghFMJ");
}

pub mod openbook_program {
    use super::{pubkey, Pubkey};
    #[cfg(feature = "devnet")]
    pub const ID: Pubkey = pubkey!("EoTcMgcDRTJVZDMZWBoU6rhYHZfkNTVEAfz3uUJRcYGj");
    #[cfg(not(feature = "devnet"))]
    pub const ID: Pubkey = pubkey!("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX");
}

pub mod amm_program {
    use super::{pubkey, Pubkey};
    #[cfg(feature = "devnet")]
    pub const ID: Pubkey = pubkey!("HWy1jotHpo6UqeQxx49dpYYdQB8wj9Qk9MdxwjLvDHB8");
    #[cfg(not(feature = "devnet"))]
    pub const ID: Pubkey = pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");
}

pub mod cpswap_program {
    use super::{pubkey, Pubkey};
    #[cfg(feature = "devnet")]
    pub const ID: Pubkey = pubkey!("CPMDWBwJDtYax9qW7AyRuVC19Cc4L4Vcy4n2BHAbHkCW");
    #[cfg(not(feature = "devnet"))]
    pub const ID: Pubkey = pubkey!("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C");
}

pub mod lock_program {
    use super::{pubkey, Pubkey};
    #[cfg(feature = "devnet")]
    pub const ID: Pubkey = pubkey!("DLockwT7X7sxtLmGH9g5kmfcjaBtncdbUmi738m5bvQC");
    #[cfg(not(feature = "devnet"))]
    pub const ID: Pubkey = pubkey!("LockrWmn6K5twhz3y9w1dQERbmgSaRkfnTeTKbpofwE");
}

pub const AUTH_SEED: &str = "vault_auth_seed";

#[program]
pub mod raydium_launchpad {
    use super::*;

    /// Initializes a new trading pool
    /// # Arguments
    ///
    /// * `ctx` - The context of accounts containing pool and token information
    ///
    pub fn initialize(
        ctx: Context<Initialize>,
        base_mint_param: MintParams,
        curve_param: CurveParams,
        vesting_param: VestingParams,
    ) -> Result<()> {
        Ok(())
    }

    /// Use the given amount of quote tokens to purchase base tokens.
    /// # Arguments
    ///
    /// * `ctx` - The context of accounts
    /// * `amount_in` - Amount of quote token to purchase
    /// * `minimum_amount_out` - Minimum amount of base token to receive (slippage protection)
    /// * `share_fee_rate` - Fee rate for the share
    ///
    pub fn buy_exact_in<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Swap<'info>>,
        amount_in: u64,
        minimum_amount_out: u64,
        share_fee_rate: u64,
    ) -> Result<()> {
        Ok(())
    }

    /// Use quote tokens to purchase the given amount of base tokens.
    /// # Arguments
    ///
    /// * `ctx` - The context of accounts
    /// * `amount_out` - Amount of base token to receive
    /// * `maximum_amount_in` - Maximum amount of quote token to purchase (slippage protection)
    /// * `share_fee_rate` - Fee rate for the share
    pub fn buy_exact_out<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Swap<'info>>,
        amount_out: u64,
        maximum_amount_in: u64,
        share_fee_rate: u64,
    ) -> Result<()> {
        Ok(())
    }

    /// Use the given amount of base tokens to sell for quote tokens.
    /// # Arguments
    ///
    /// * `ctx` - The context of accounts
    /// * `amount_in` - Amount of base token to sell
    /// * `minimum_amount_out` - Minimum amount of quote token to receive (slippage protection)
    /// * `share_fee_rate` - Fee rate for the share
    ///
    pub fn sell_exact_in<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Swap<'info>>,
        amount_in: u64,
        minimum_amount_out: u64,
        share_fee_rate: u64,
    ) -> Result<()> {
        Ok(())
    }

    /// Sell base tokens for the given amount of quote tokens.
    /// # Arguments
    ///
    /// * `ctx` - The context of accounts
    /// * `amount_out` - Amount of quote token to receive
    /// * `maximum_amount_in` - Maximum amount of base token to purchase (slippage protection)
    /// * `share_fee_rate` - Fee rate for the share
    ///
    pub fn sell_exact_out<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Swap<'info>>,
        amount_out: u64,
        maximum_amount_in: u64,
        share_fee_rate: u64,
    ) -> Result<()> {
        Ok(())
    }

    /// Create vesting account
    /// # Arguments
    ///
    /// * `ctx` - The context of accounts
    /// * `share` - The share amount of base token to be vested
    ///
    pub fn create_vesting_account(
        ctx: Context<CreateVestingAccount>,
        share_amount: u64,
    ) -> Result<()> {
        Ok(())
    }

    /// Claim vested token
    /// # Arguments
    pub fn claim_vested_token(ctx: Context<ClaimVestedToken>) -> Result<()> {
        Ok(())
    }

    /// Create platform config account
    /// # Arguments
    ///
    /// * `ctx` - The context of accounts
    /// # Fields
    /// * `fee_rate` - Fee rate of the platform
    /// * `name` - Name of the platform
    /// * `web` - Website of the platform
    /// * `img` - Image link of the platform
    ///
    pub fn create_platform_config(
        ctx: Context<CreatePlatformConfig>,
        platform_params: PlatformParams,
    ) -> Result<()> {
        Ok(())
    }

    /// Claim platform fee
    /// # Arguments
    ///
    /// * `ctx` - The context of accounts
    ///
    pub fn claim_platform_fee(ctx: Context<ClaimPlatformFee>) -> Result<()> {
        Ok(())
    }

    /// Update platform config
    /// # Arguments
    ///
    /// * `ctx` - The context of accounts
    /// * `param` - Parameter to update
    ///
    pub fn update_platform_config(
        ctx: Context<UpdatePlatformConfig>,
        param: PlatformConfigParam,
    ) -> Result<()> {
        Ok(())
    }
}
