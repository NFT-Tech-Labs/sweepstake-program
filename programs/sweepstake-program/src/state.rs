use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub authority: Pubkey,
    pub current_sweepstake_key: Option<Pubkey>,
    pub sweepstakes_submitted: u8,
    pub id: i64,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct SweepstakeData {
    pub id: i64,
    pub world_champion: String,
    pub group_stage_1: String,
    pub group_stage_2: String,
    pub group_stage_3: String,
    pub round_of_16: String,
    pub quarter_finals: String,
    pub semifinals: String,
    pub third_place_game: String,
    pub final_game: String,
    pub payment_amount: u64,
}

#[account]
pub struct Sweepstake {
    pub id: i64,
    pub authority: Pubkey,
    pub world_champion: String,
    pub group_stage_1: String,
    pub group_stage_2: String,
    pub group_stage_3: String,
    pub round_of_16: String,
    pub quarter_finals: String,
    pub semifinals: String,
    pub third_place_game: String,
    pub final_game: String,
    pub submitted_at: i64, // solana_program::clock::UnixTimestamp
    pub pre_sweepstake_key: Option<Pubkey>,
}
