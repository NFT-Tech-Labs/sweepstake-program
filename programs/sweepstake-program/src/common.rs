use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::UnixTimestamp;

pub fn now() -> UnixTimestamp {
    Clock::get().unwrap().unix_timestamp
}
