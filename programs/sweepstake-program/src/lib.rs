use crate::common::now;
use crate::state::{Sweepstake, SweepstakeData, User};
use crate::validator::*;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

pub mod common;
pub mod error;
pub mod state;
pub mod validator;

declare_id!("HqRe8tqXAvD4at3dduSpK2kCNHs9XLosvNf6X5pPh4j8");

// @todo: set correct dagoats wallet addresses
const SOL_WALLET_ADDRESS: &str = "";

#[program]
pub mod dagoats_sweepstake {
    use super::*;

    pub fn create_user(ctx: Context<SweepstakeInitialize>, user_id: i64) -> Result<()> {
        validate_deadline()?;

        let user_state = &mut ctx.accounts.user_state;

        user_state.current_sweepstake_key = None;
        user_state.sweepstakes_submitted = 0;
        user_state.authority = ctx.accounts.authority.key();
        user_state.id = get_valid_id(user_id)?;

        Ok(())
    }

    pub fn create_sweepstake_sol(
        ctx: Context<CreateSweepstakeSol>,
        data: SweepstakeData,
    ) -> Result<()> {
        let user = &mut ctx.accounts.user_state;
        let sweepstake = &mut ctx.accounts.sweepstake_state;

        validate_lamports_per_sweepstake(data.payment_amount)?;
        create_sweepstake(ctx.accounts.authority.key(), user, sweepstake, &data)?;
        transfer(ctx.accounts.to_transfer_context(), data.payment_amount)?;

        Ok(())
    }
}

pub fn create_sweepstake(
    authority: Pubkey,
    user: &mut Account<User>,
    sweepstake: &mut Account<Sweepstake>,
    data: &SweepstakeData,
) -> Result<()> {
    validate_deadline()?;
    validate_sweepstakes_per_wallet(user.sweepstakes_submitted)?;

    sweepstake.authority = authority;
    sweepstake.submitted_at = now();
    sweepstake.final_game = get_valid_sweepstake_input(data.final_game.clone(), 1)?;
    sweepstake.third_place_game = get_valid_sweepstake_input(data.third_place_game.clone(), 1)?;
    sweepstake.semifinals = get_valid_sweepstake_input(data.semifinals.clone(), 2)?;
    sweepstake.quarter_finals = get_valid_sweepstake_input(data.quarter_finals.clone(), 4)?;
    sweepstake.round_of_16 = get_valid_sweepstake_input(data.round_of_16.clone(), 8)?;
    sweepstake.group_stage_3 = get_valid_sweepstake_input(data.group_stage_3.clone(), 16)?;
    sweepstake.group_stage_2 = get_valid_sweepstake_input(data.group_stage_2.clone(), 16)?;
    sweepstake.group_stage_1 = get_valid_sweepstake_input(data.group_stage_1.clone(), 16)?;
    sweepstake.world_champion = get_valid_world_champion(data.world_champion.clone())?;
    sweepstake.pre_sweepstake_key = None;
    sweepstake.id = get_valid_id(data.id)?;

    user.current_sweepstake_key = Some(sweepstake.key());
    user.sweepstakes_submitted = user.sweepstakes_submitted + 1;

    Ok(())
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
pub struct CreateSweepstakeSol<'info> {
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
    /// CHECK:
    #[account(mut, address = SOL_WALLET_ADDRESS.parse::<Pubkey>().unwrap())]
    pub dagoats_wallet: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateSweepstakeSol<'info> {
    pub fn to_transfer_context(&self) -> CpiContext<'info, 'info, 'info, 'info, Transfer<'info>> {
        let accounts = Transfer {
            from: self.authority.to_account_info(),
            to: self.dagoats_wallet.to_account_info(),
        };
        CpiContext::new(self.system_program.to_account_info(), accounts)
    }
}
