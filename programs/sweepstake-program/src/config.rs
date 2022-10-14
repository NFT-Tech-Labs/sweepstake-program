use anchor_lang::{
    prelude::{Account, Pubkey},
    solana_program::native_token::LAMPORTS_PER_SOL,
    Key,
};
use anchor_spl::token::{Mint, TokenAccount};

// @todo: change SOL price and wallet address
pub const LAMPORTS_PER_SWEEPSTAKE: u64 = LAMPORTS_PER_SOL;
pub const SOL_WALLET_ADDRESS: &str = "53Xa3PVBki4ZT2qJoJPfiGiA42SyuvQ6WXj5ysw8TRv1";

// @todo: change DUST price and wallet address
const DUST_MINT_ADDRESS: &str = "DUSTawucrTsGU8hcqRdHDCbuYhCPADMLM2VcCb8VnFnQ";
const DUST_PER_SWEEPSTAKE: u64 = 10 * LAMPORTS_PER_SOL;
const DUST_WALLET_ADDRESS: &str = "53Xa3PVBki4ZT2qJoJPfiGiA42SyuvQ6WXj5ysw8TRv1";

// @todo: change TSC mint address, price and wallet address
const TSC_MINT_ADDRESS: &str = "DUSTawucrTsGU8hcqRdHDCbuYhCPADMLM2VcCb8VnFnQ";
const TSC_PER_SWEEPSTAKE: u64 = 10 * LAMPORTS_PER_SOL;
const TSC_WALLET_ADDRESS: &str = "53Xa3PVBki4ZT2qJoJPfiGiA42SyuvQ6WXj5ysw8TRv1";

// @todo: change LABS mint address, price and wallet address
const LABS_MINT_ADDRESS: &str = "DUSTawucrTsGU8hcqRdHDCbuYhCPADMLM2VcCb8VnFnQ";
const LABS_PER_SWEEPSTAKE: u64 = 10 * LAMPORTS_PER_SOL;
const LABS_WALLET_ADDRESS: &str = "53Xa3PVBki4ZT2qJoJPfiGiA42SyuvQ6WXj5ysw8TRv1";

const TEST_MINT_ADDRESS: &str = "AKnL4NNf3DGWZJS6cPknBuEGnVsV4A4m5tgebLHaRSZ9";
const TEST_PER_SWEEPSTAKE: u64 = 10 * LAMPORTS_PER_SOL;
const TEST_WALLET_ADDRESS: &str = "GreYZ8jbfridyk3y7TGqpTNRUuhAf7hzkvdZn8zwYHBh";

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
            String::from(TSC_MINT_ADDRESS),
            String::from(TSC_WALLET_ADDRESS),
            TSC_PER_SWEEPSTAKE,
        ),
        SupportedToken::new(
            String::from(LABS_MINT_ADDRESS),
            String::from(LABS_WALLET_ADDRESS),
            LABS_PER_SWEEPSTAKE,
        ),
        // @todo: for testing purposes only!
        // SupportedToken::new(
        //     String::from(TEST_MINT_ADDRESS),
        //     String::from(TEST_WALLET_ADDRESS),
        //     TEST_PER_SWEEPSTAKE,
        // ),
    ]
}
