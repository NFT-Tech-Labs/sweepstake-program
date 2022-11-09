use anchor_lang::{
    prelude::{Account, Pubkey},
    solana_program::native_token::LAMPORTS_PER_SOL,
    Key,
};
use anchor_spl::token::{Mint, TokenAccount};

pub const LAMPORTS_PER_SWEEPSTAKE: u64 = 250_000_000;
pub const SOL_WALLET_ADDRESS: &str = "EbwXL2F6ejQ17ixM8uWND1GpKPotRZJMLA5f36Q1NkvQ";

// @todo: change DUST price, check the decimals
const DUST_MINT_ADDRESS: &str = "DUSTawucrTsGU8hcqRdHDCbuYhCPADMLM2VcCb8VnFnQ";
const DUST_PER_SWEEPSTAKE: u64 = 10 * LAMPORTS_PER_SOL;
const DUST_WALLET_ADDRESS: &str = "4CWQ73bjhZLhYcWmSpdSTUqdpwBywMkcrmHMyUyQhZgY";

// @todo: change FORGE price, check the decimals
const FORGE_MINT_ADDRESS: &str = "FoRGERiW7odcCBGU1bztZi16osPBHjxharvDathL5eds";
const FORGE_PER_SWEEPSTAKE: u64 = 10 * LAMPORTS_PER_SOL;
const FORGE_WALLET_ADDRESS: &str = "4CWQ73bjhZLhYcWmSpdSTUqdpwBywMkcrmHMyUyQhZgY";

// @todo: change USDC price, check the decimals
const USDC_MINT_ADDRESS: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
const USDC_PER_SWEEPSTAKE: u64 = 10 * LAMPORTS_PER_SOL;
const USDC_WALLET_ADDRESS: &str = "4CWQ73bjhZLhYcWmSpdSTUqdpwBywMkcrmHMyUyQhZgY";

const TEST_MINT_ADDRESS: &str = "AKnL4NNf3DGWZJS6cPknBuEGnVsV4A4m5tgebLHaRSZ9";
const TEST_PER_SWEEPSTAKE: u64 = 10 * LAMPORTS_PER_SOL;
const TEST_WALLET_ADDRESS: &str = "ABWfjgHT4AK1QoGrk7jKaRjMvK8t1GFourXR7nACRaUL";

pub struct SupportedToken {
    pub sweepstake_price: u64,
    mint_address: Pubkey,
    dagoats_address: Pubkey,
}

impl SupportedToken {
    fn new(mint_address: String, dagoats_address: String, sweepstake_price: u64) -> Self {
        Self {
            sweepstake_price,
            mint_address: mint_address.parse::<Pubkey>().unwrap(),
            dagoats_address: dagoats_address.parse::<Pubkey>().unwrap(),
        }
    }

    pub fn eq(&self, mint: &Account<Mint>, dagoats_account: &Account<TokenAccount>) -> bool {
        mint.key() == self.mint_address && dagoats_account.key() == self.dagoats_address
    }
}

pub fn get_supported_tokens() -> Vec<SupportedToken> {
    vec![
        SupportedToken::new(
            String::from(DUST_MINT_ADDRESS),
            String::from(DUST_WALLET_ADDRESS),
            DUST_PER_SWEEPSTAKE,
        ),
        SupportedToken::new(
            String::from(FORGE_MINT_ADDRESS),
            String::from(FORGE_WALLET_ADDRESS),
            FORGE_PER_SWEEPSTAKE,
        ),
        SupportedToken::new(
            String::from(USDC_MINT_ADDRESS),
            String::from(USDC_WALLET_ADDRESS),
            USDC_PER_SWEEPSTAKE,
        ),
        // @todo: for testing purposes only!
        // SupportedToken::new(
        //     String::from(TEST_MINT_ADDRESS),
        //     String::from(TEST_WALLET_ADDRESS),
        //     TEST_PER_SWEEPSTAKE,
        // ),
    ]
}
