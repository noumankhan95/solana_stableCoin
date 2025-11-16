use crate::error::ErrorCode;
use crate::MAX_AGE;
use crate::{PRICE_FEED_DECIMAL_ADJUSTMENT, SEEDS_SOL_ACCOUNT};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::associated_token::spl_associated_token_account::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_spl::token_2022::{burn, Burn};
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use pyth_sdk_solana::state::SolanaPriceAccount;
pub fn withdraw_sol<'info>(
    bump: u8,
    depositor_key: &Pubkey,
    from: &SystemAccount<'info>,
    to: &AccountInfo<'info>,
    amount: u64,
    system_program: &Program<'info, System>,
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[SEEDS_SOL_ACCOUNT, depositor_key.as_ref(), &[bump]]];
    transfer(
        CpiContext::new_with_signer(
            system_program.to_account_info(),
            Transfer {
                from: from.to_account_info(),
                to: to.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;

    Ok(())
}

pub fn burn_tokens<'info>(
    token_program: &Program<'info, Token2022>,
    mint_account: &InterfaceAccount<'info, Mint>,
    token_account: &InterfaceAccount<'info, TokenAccount>,
    authority: &Signer<'info>,
    amount: u64,
) -> Result<()> {
    burn(
        CpiContext::new(
            token_program.to_account_info(),
            Burn {
                mint: mint_account.to_account_info(),
                from: token_account.to_account_info(),
                authority: authority.to_account_info(),
            },
        ),
        amount,
    )?;
    Ok(())
}

pub fn get_lamports_from_usd<'info>(
    amount_in_usd: &u64,
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
    let amount_in_lamports = (*amount_in_usd as u128 * LAMPORTS_PER_SOL as u128) / (usd_price);
    Ok(amount_in_lamports as u64)
}
