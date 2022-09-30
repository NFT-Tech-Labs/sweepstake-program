use crate::common::now;
use crate::state::{Sweepstake, SweepstakeData, User};
use crate::validator::get_valid_sweepstake_input;
use anchor_lang::prelude::*;

pub mod common;
pub mod error;
pub mod state;
pub mod validator;

declare_id!("HqRe8tqXAvD4at3dduSpK2kCNHs9XLosvNf6X5pPh4j8");

#[program]
pub mod dagoats_sweepstake {
    use super::*;
    use crate::validator::{
        get_valid_id, get_valid_world_champion, validate_deadline, validate_sweepstakes_per_wallet,
    };

    pub fn create_user(ctx: Context<SweepstakeInitialize>, user_id: i64) -> Result<()> {
        validate_deadline()?;

        let user_state = &mut ctx.accounts.user_state;

        user_state.current_sweepstake_key = None;
        user_state.sweepstakes_submitted = 0;
        user_state.authority = ctx.accounts.authority.key();
        user_state.id = get_valid_id(user_id)?;

        Ok(())
    }

    pub fn create_sweepstake(ctx: Context<CreateSweepstake>, data: SweepstakeData) -> Result<()> {
        validate_deadline()?;

        // @todo: Transfer tokens to our wallet - use and update `ctx.accounts.to_transfer_context`
        let user = &mut ctx.accounts.user_state;

        validate_sweepstakes_per_wallet(user.sweepstakes_submitted)?;

        let sweepstake = &mut ctx.accounts.sweepstake_state;

        sweepstake.authority = ctx.accounts.authority.key();
        sweepstake.submitted_at = now();
        sweepstake.final_game = get_valid_sweepstake_input(data.final_game, 1)?;
        sweepstake.third_place_game = get_valid_sweepstake_input(data.third_place_game, 1)?;
        sweepstake.semifinals = get_valid_sweepstake_input(data.semifinals, 2)?;
        sweepstake.quarter_finals = get_valid_sweepstake_input(data.quarter_finals, 4)?;
        sweepstake.round_of_16 = get_valid_sweepstake_input(data.round_of_16, 8)?;
        sweepstake.group_stage_3 = get_valid_sweepstake_input(data.group_stage_3, 16)?;
        sweepstake.group_stage_2 = get_valid_sweepstake_input(data.group_stage_2, 16)?;
        sweepstake.group_stage_1 = get_valid_sweepstake_input(data.group_stage_1, 16)?;
        sweepstake.world_champion = get_valid_world_champion(data.world_champion)?;
        sweepstake.pre_sweepstake_key = None;
        sweepstake.id = get_valid_id(data.id)?;

        user.current_sweepstake_key = Some(sweepstake.key());
        user.sweepstakes_submitted = user.sweepstakes_submitted + 1;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SweepstakeInitialize<'info> {
    #[account(
        init,
        payer = authority,
        // discriminator + authority + sweepstakes_submitted + current_sweepstake_key + id
        space = 8 + 32 + 1 + std::mem::size_of::<Option<Pubkey>>() + 8,
    )]
    pub user_state: Account<'info, User>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateSweepstake<'info> {
    #[account(mut, has_one = authority)]
    pub user_state: Account<'info, User>,
    #[account(
        init,
        payer = authority,
        // discriminator + authority + world_champion + group_stage_1 + group_stage_2 + group_stage_3 + round_of_16 + quarter_finals + semifinals + third_place_game + final_game + submitted_at + pre_sweepstake_key + id
        space = 8 + 32 + 8 + 768 + 768 + 768 + 384 + 192 + 96 + 48 + 48 + 8 + std::mem::size_of::<Option<Pubkey>>() + 8,
    )]
    pub sweepstake_state: Account<'info, Sweepstake>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateSweepstake<'info> {
    // pub fn to_transfer_context(&self) -> CpiContext<'info, 'info, 'info, 'info, Transfer<'info>> {
    //     @todo: Transfer tokens to our wallet
    //     let accounts = Transfer {
    //         from: self.authority.to_account_info(),
    //         to: ...,
    //     };
    //     CpiContext::new(self.system_program.to_account_info(), accounts)
    // }
}
