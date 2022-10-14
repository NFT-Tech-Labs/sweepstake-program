use crate::config::{get_supported_tokens, SupportedToken};
use crate::error::SweepstakeError;
use crate::now;
use anchor_lang::{prelude::*, solana_program::clock::UnixTimestamp};
use anchor_spl::token::{Mint, TokenAccount};

const MAX_SWEEPSTAKES_PER_WALLET: u8 = 1;
// 2022-11-20T14:00:00Z
const SWEEPSTAKE_SUBMISSION_DEADLINE: UnixTimestamp = 1668952800;

pub fn validate_supported_token(
    mint: &Account<Mint>,
    dagoats_account: &Account<TokenAccount>,
) -> Result<SupportedToken> {
    for token in get_supported_tokens() {
        if token.eq(mint, dagoats_account) {
            return Ok(token);
        }
    }
    Err(error!(SweepstakeError::InvalidAccount))
}

pub fn get_valid_sweepstake_input(data: String, expected_length: usize) -> Result<String> {
    let input = data
        .split(";")
        .filter(|&text| !text.is_empty())
        .collect::<Vec<&str>>();
    if input.len() != expected_length {
        return Err(error!(SweepstakeError::InvalidInputLength));
    }
    Ok(data)
}

pub fn get_valid_world_champion(data: String) -> Result<String> {
    if data.len() != 2 {
        return Err(error!(SweepstakeError::InvalidInputLength));
    }
    Ok(data)
}

pub fn get_valid_id(id: i64) -> Result<i64> {
    if id <= 0 {
        return Err(error!(SweepstakeError::InvalidInputData));
    }
    Ok(id)
}

pub fn validate_deadline() -> Result<()> {
    if now() > SWEEPSTAKE_SUBMISSION_DEADLINE {
        return Err(error!(SweepstakeError::SweepstakeDisabled));
    }
    Ok(())
}

pub fn validate_sweepstakes_per_wallet(sweepstakes: u8) -> Result<()> {
    if sweepstakes >= MAX_SWEEPSTAKES_PER_WALLET {
        return Err(error!(SweepstakeError::SweepstakeLimitExceeded));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_valid_sweepstake_input() {
        let input = "NL-ES=12:3;";
        let result = get_valid_sweepstake_input(String::from(input), 1).unwrap();
        assert_eq!(result.as_str(), input);

        let input = "NL-ES=12:32;NL-ES=12:32;NL-ES=12:32";
        let result = get_valid_sweepstake_input(String::from(input), 3).unwrap();
        assert_eq!(result.as_str(), input);

        let input = "NL-ES=12:32;NL-ES=12:32";
        let result = get_valid_sweepstake_input(String::from(input), 1);
        assert!(result.is_err());
    }
}
