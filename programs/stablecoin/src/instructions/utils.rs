use anchor_lang::prelude::*;
use anchor_spl::associated_token::spl_associated_token_account::solana_program::native_token::LAMPORTS_PER_SOL;

use crate::error::ErrorCode;
use crate::{Collateral, Config, MAX_AGE, PRICE_FEED_DECIMAL_ADJUSTMENT};
use pyth_sdk_solana::state::SolanaPriceAccount;

pub fn check_health_factor<'info>(
    collateral: &Account<'info, Collateral>,
    config: &Account<'info, Config>,
    price_feed: &AccountInfo<'info>,
) -> Result<()> {
    let health_factor = calculate_health_factor(collateral, config, price_feed)?;
    require!(health_factor >= config.min_health_factor,ErrorCode::LowHealth);
    Ok(())
}

pub fn calculate_health_factor<'info>(
    collateral: &Account<'info, Collateral>,
    config: &Account<'info, Config>,
    price_feed: &AccountInfo<'info>,
) -> Result<u64> {
    let collateral_value_in_usd = get_usd_value(&collateral.lamports_balance, &price_feed)?;
    let collateral_adjusted_for_liquidation_threshold =
        (collateral_value_in_usd * config.liquidation_threshold) / 100;
    if collateral.amount_minted == 0 {
        msg!("Health Factor Max");
        return Ok(u64::MAX);
    }
    let health_factor = (collateral_adjusted_for_liquidation_threshold) / collateral.amount_minted;
    Ok(health_factor)
}

pub fn get_usd_value<'info>(
    amount_in_lamports: &u64,
    price_feed: &AccountInfo<'info>,
) -> Result<u64> {
    let clock = Clock::get()?;

    // ✅ Fetch SOL price from Pyth feed
    let usd_price_feed = SolanaPriceAccount::account_info_to_feed(price_feed)
        .map_err(|_| error!(ErrorCode::InvalidPythAccount))?;
    let usd_price_data = usd_price_feed
        .get_price_no_older_than(clock.unix_timestamp, MAX_AGE)
        .ok_or(error!(ErrorCode::StalePrice))?;
    let usd_price = (usd_price_data.price as u128 * PRICE_FEED_DECIMAL_ADJUSTMENT);
    let usd_expo = usd_price_data.expo;
    let amount_in_usd = (*amount_in_lamports as u128 * usd_price) / (LAMPORTS_PER_SOL as u128);
    Ok(amount_in_usd as u64)
}
