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
    /// Predicted world champion in the ISO 3166-1 alpha-2 country format (e.g. `ES`)
    pub world_champion: String,
    /// Predicted results of the group stage 1 in the `XX-YY=R1:R2;` format where
    /// - `XX` = ISO 3166-1 alpha-2 country code of the first playing country (e.g. `ES`)
    /// - `YY` = ISO 3166-1 alpha-2 country code of the second playing country (e.g. `NL`)
    /// - `R1` = Result of the first playing country (number of goals) up to 99 (e.g. `1`)
    /// - `R2` = Result of the first playing country (number of goals) up to 99 (e.g. `0`)
    /// For example: `ES-NL=1:0;CZ-PL=5:14;`
    pub group_stage_1: String,
    /// Predicted results of the group stage 2 in the `XX-YY=R1:R2;` format where
    /// - `XX` = ISO 3166-1 alpha-2 country code of the first playing country (e.g. `ES`)
    /// - `YY` = ISO 3166-1 alpha-2 country code of the second playing country (e.g. `NL`)
    /// - `R1` = Result of the first playing country (number of goals) up to 99 (e.g. `1`)
    /// - `R2` = Result of the first playing country (number of goals) up to 99 (e.g. `0`)
    /// For example: `ES-NL=1:0;CZ-PL=5:14;`
    pub group_stage_2: String,
    /// Predicted results of the group stage 3 in the `XX-YY=R1:R2;` format where
    /// - `XX` = ISO 3166-1 alpha-2 country code of the first playing country (e.g. `ES`)
    /// - `YY` = ISO 3166-1 alpha-2 country code of the second playing country (e.g. `NL`)
    /// - `R1` = Result of the first playing country (number of goals) up to 99 (e.g. `1`)
    /// - `R2` = Result of the first playing country (number of goals) up to 99 (e.g. `0`)
    /// For example: `ES-NL=1:0;CZ-PL=5:14;`
    pub group_stage_3: String,
    /// Predicted results of the round of 16 in the `XX-YY=R1:R2;` format where
    /// - `XX` = ISO 3166-1 alpha-2 country code of the first playing country (e.g. `ES`)
    /// - `YY` = ISO 3166-1 alpha-2 country code of the second playing country (e.g. `NL`)
    /// - `R1` = Result of the first playing country (number of goals) up to 99 (e.g. `1`)
    /// - `R2` = Result of the first playing country (number of goals) up to 99 (e.g. `0`)
    /// For example: `ES-NL=1:0;CZ-PL=5:14;`
    pub round_of_16: String,
    /// Predicted results of the quarter finals in the `XX-YY=R1:R2;` format where
    /// - `XX` = ISO 3166-1 alpha-2 country code of the first playing country (e.g. `ES`)
    /// - `YY` = ISO 3166-1 alpha-2 country code of the second playing country (e.g. `NL`)
    /// - `R1` = Result of the first playing country (number of goals) up to 99 (e.g. `1`)
    /// - `R2` = Result of the first playing country (number of goals) up to 99 (e.g. `0`)
    /// For example: `ES-NL=1:0;CZ-PL=5:14;`
    pub quarter_finals: String,
    /// Predicted results of the semifinals in the `XX-YY=R1:R2;` format where
    /// - `XX` = ISO 3166-1 alpha-2 country code of the first playing country (e.g. `ES`)
    /// - `YY` = ISO 3166-1 alpha-2 country code of the second playing country (e.g. `NL`)
    /// - `R1` = Result of the first playing country (number of goals) up to 99 (e.g. `1`)
    /// - `R2` = Result of the first playing country (number of goals) up to 99 (e.g. `0`)
    /// For example: `ES-NL=1:0;CZ-PL=5:14;`
    pub semifinals: String,
    /// Predicted results of the third place game in the `XX-YY=R1:R2` format where
    /// - `XX` = ISO 3166-1 alpha-2 country code of the first playing country (e.g. `ES`)
    /// - `YY` = ISO 3166-1 alpha-2 country code of the second playing country (e.g. `NL`)
    /// - `R1` = Result of the first playing country (number of goals) up to 99 (e.g. `1`)
    /// - `R2` = Result of the first playing country (number of goals) up to 99 (e.g. `0`)
    /// For example: `ES-NL=1:0`
    pub third_place_game: String,
    /// Predicted results of the final game in the `XX-YY=R1:R2` format where
    /// - `XX` = ISO 3166-1 alpha-2 country code of the first playing country (e.g. `ES`)
    /// - `YY` = ISO 3166-1 alpha-2 country code of the second playing country (e.g. `NL`)
    /// - `R1` = Result of the first playing country (number of goals) up to 99 (e.g. `1`)
    /// - `R2` = Result of the first playing country (number of goals) up to 99 (e.g. `0`)
    /// For example: `ES-NL=1:0`
    pub final_game: String,
}

#[account]
pub struct Sweepstake {
    /// Sweepstake ID from the database (reference ID)
    pub id: i64,
    /// Creator and payer of the sweepstake
    pub authority: Pubkey,
    /// Predicted world champion in the ISO 3166-1 alpha-2 country format (e.g. `ES`)
    pub world_champion: String,
    /// Predicted results of the group stage 1 in the `XX-YY=R1:R2;` format where
    /// - `XX` = ISO 3166-1 alpha-2 country code of the first playing country (e.g. `ES`)
    /// - `YY` = ISO 3166-1 alpha-2 country code of the second playing country (e.g. `NL`)
    /// - `R1` = Result of the first playing country (number of goals) up to 99 (e.g. `1`)
    /// - `R2` = Result of the first playing country (number of goals) up to 99 (e.g. `0`)
    /// For example: `ES-NL=1:0;CZ-PL=5:14;`
    pub group_stage_1: String,
    /// Predicted results of the group stage 2 in the `XX-YY=R1:R2;` format where
    /// - `XX` = ISO 3166-1 alpha-2 country code of the first playing country (e.g. `ES`)
    /// - `YY` = ISO 3166-1 alpha-2 country code of the second playing country (e.g. `NL`)
    /// - `R1` = Result of the first playing country (number of goals) up to 99 (e.g. `1`)
    /// - `R2` = Result of the first playing country (number of goals) up to 99 (e.g. `0`)
    /// For example: `ES-NL=1:0;CZ-PL=5:14;`
    pub group_stage_2: String,
    /// Predicted results of the group stage 3 in the `XX-YY=R1:R2;` format where
    /// - `XX` = ISO 3166-1 alpha-2 country code of the first playing country (e.g. `ES`)
    /// - `YY` = ISO 3166-1 alpha-2 country code of the second playing country (e.g. `NL`)
    /// - `R1` = Result of the first playing country (number of goals) up to 99 (e.g. `1`)
    /// - `R2` = Result of the first playing country (number of goals) up to 99 (e.g. `0`)
    /// For example: `ES-NL=1:0;CZ-PL=5:14;`
    pub group_stage_3: String,
    /// Predicted results of the round of 16 in the `XX-YY=R1:R2;` format where
    /// - `XX` = ISO 3166-1 alpha-2 country code of the first playing country (e.g. `ES`)
    /// - `YY` = ISO 3166-1 alpha-2 country code of the second playing country (e.g. `NL`)
    /// - `R1` = Result of the first playing country (number of goals) up to 99 (e.g. `1`)
    /// - `R2` = Result of the first playing country (number of goals) up to 99 (e.g. `0`)
    /// For example: `ES-NL=1:0;CZ-PL=5:14;`
    pub round_of_16: String,
    /// Predicted results of the quarter finals in the `XX-YY=R1:R2;` format where
    /// - `XX` = ISO 3166-1 alpha-2 country code of the first playing country (e.g. `ES`)
    /// - `YY` = ISO 3166-1 alpha-2 country code of the second playing country (e.g. `NL`)
    /// - `R1` = Result of the first playing country (number of goals) up to 99 (e.g. `1`)
    /// - `R2` = Result of the first playing country (number of goals) up to 99 (e.g. `0`)
    /// For example: `ES-NL=1:0;CZ-PL=5:14;`
    pub quarter_finals: String,
    /// Predicted results of the semifinals in the `XX-YY=R1:R2;` format where
    /// - `XX` = ISO 3166-1 alpha-2 country code of the first playing country (e.g. `ES`)
    /// - `YY` = ISO 3166-1 alpha-2 country code of the second playing country (e.g. `NL`)
    /// - `R1` = Result of the first playing country (number of goals) up to 99 (e.g. `1`)
    /// - `R2` = Result of the first playing country (number of goals) up to 99 (e.g. `0`)
    /// For example: `ES-NL=1:0;CZ-PL=5:14;`
    pub semifinals: String,
    /// Predicted results of the third place game in the `XX-YY=R1:R2` format where
    /// - `XX` = ISO 3166-1 alpha-2 country code of the first playing country (e.g. `ES`)
    /// - `YY` = ISO 3166-1 alpha-2 country code of the second playing country (e.g. `NL`)
    /// - `R1` = Result of the first playing country (number of goals) up to 99 (e.g. `1`)
    /// - `R2` = Result of the first playing country (number of goals) up to 99 (e.g. `0`)
    /// For example: `ES-NL=1:0`
    pub third_place_game: String,
    /// Predicted results of the final game in the `XX-YY=R1:R2` format where
    /// - `XX` = ISO 3166-1 alpha-2 country code of the first playing country (e.g. `ES`)
    /// - `YY` = ISO 3166-1 alpha-2 country code of the second playing country (e.g. `NL`)
    /// - `R1` = Result of the first playing country (number of goals) up to 99 (e.g. `1`)
    /// - `R2` = Result of the first playing country (number of goals) up to 99 (e.g. `0`)
    /// For example: `ES-NL=1:0`
    pub final_game: String,
    /// A `solana_program::clock::UnixTimestamp` timestamp of the time the sweepstake was submitted
    pub submitted_at: i64,
    /// Reference to a previously created sweepstake
    pub pre_sweepstake_key: Option<Pubkey>,
}
