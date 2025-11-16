use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Pyth Account")]
    InvalidPythAccount,
    #[msg("Price Gone Stale")]
    StalePrice,
    #[msg("Health Factor Lower")]
    LowHealth,
    #[msg("Health Factor Above Min Health")]
    AboveMinHealth,
}
