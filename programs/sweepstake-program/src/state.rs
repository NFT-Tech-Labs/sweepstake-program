use anchor_lang::prelude::*;

#[account]
pub struct User {
    /// Account owner and payer
    pub authority: Pubkey,
    /// Latest created sweepstake
    pub current_sweepstake_key: Option<Pubkey>,
    /// Number of sweepstakes created
    pub sweepstakes_submitted: u8,
    /// User ID from the database (reference ID)
    pub id: i64,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct SweepstakeData {
    /// Sweepstake ID from the database (reference ID)
    pub id: i64,
    /// Hashed (sha1) JSON object of the sweepstake
    pub predictions: String,
}

#[account]
pub struct Sweepstake {
    /// Sweepstake ID from the database (reference ID)
    pub id: i64,
    /// Creator and payer of the sweepstake
    pub authority: Pubkey,
    /// Hashed (sha1) JSON object of the sweepstake
    pub predictions: String,
    /// A `solana_program::clock::UnixTimestamp` timestamp of the time the sweepstake was submitted
    pub submitted_at: i64,
    /// Reference to a previously created sweepstake
    pub pre_sweepstake_key: Option<Pubkey>,
}
