use anchor_lang::prelude::*;

#[error_code]
pub enum SweepstakeError {
    #[msg("Sweepstake data has invalid length!")]
    InvalidInputLength,
    #[msg("Exceeded number of sweepstakes per wallet!")]
    SweepstakeLimitExceeded,
    #[msg("Sweepstake create period has ended!")]
    SweepstakeDisabled,
    #[msg("Sweepstake input data are not valid!")]
    InvalidInputData,
    #[msg("Not enough tokens to pay for sweepstake!")]
    InvalidSweepstakePrice,
}
