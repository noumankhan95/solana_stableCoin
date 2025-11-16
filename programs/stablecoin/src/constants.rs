use anchor_lang::prelude::*;

pub const SEED_CONFIG_ACCOUNT: &[u8] = b"config_account";
pub const SEED_MINT_ACCOUNT: &[u8] = b"mint";
pub const MINT_DECIMALS: u8 = 8;
pub const LIQUIDATION_BONUS: u64 = 10;
pub const LIQUIDATION_THRESHOLD: u64 = 50;
pub const MINIMUM_HEALTH_FACTOR: u64 = 1;
pub const SEEDS_COLLATERAL_ACCOUNT: &[u8] = b"collateral";
pub const SEEDS_SOL_ACCOUNT:&[u8]=b"sol";
pub const MAX_AGE:u64=100;
pub const PRICE_FEED_DECIMAL_ADJUSTMENT:u128=10;