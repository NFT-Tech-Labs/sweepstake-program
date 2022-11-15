use crate::common::now;
use crate::config::{LAMPORTS_PER_SWEEPSTAKE, SOL_WALLET_ADDRESS};
use crate::state::{Sweepstake, SweepstakeData, User};
use crate::validator::*;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::token::{
    transfer as token_transfer, Mint, Token, TokenAccount, Transfer as TokenTransfer,
};
use solana_security_txt::security_txt;

pub mod common;
pub mod config;
pub mod error;
pub mod state;
pub mod validator;

declare_id!("BQoZJCb4arM2NypHHVHejAgs7VD31qcWsc1KGwqKexH3");

#[program]
pub mod dagoats_sweepstake {
    use super::*;

    /// Initialize user state
    /// - A user can be created before the start of the World Cup
    /// - All the input data must be valid
    /// Input data:
    /// - `user_id`: User ID from the database (reference ID)
    pub fn create_user(ctx: Context<SweepstakeInitialize>, user_id: i64) -> Result<()> {
        validate_deadline()?;

        let user_state = &mut ctx.accounts.user_state;

        user_state.current_sweepstake_key = None;
        user_state.sweepstakes_submitted = 0;
        user_state.authority = ctx.accounts.authority.key();
        user_state.id = get_valid_id(user_id)?;

        Ok(())
    }

    /// Create sweepstake and pay in $SOL for it
    /// - A sweepstake can be created before the start of the World Cup
    /// - All the input data must be valid
    /// - User cannot exceed a number of sweepstakes per wallet
    /// - Sweepstake must be paid in $SOL
    pub fn create_sweepstake_sol(
        ctx: Context<CreateSweepstakeSol>,
        data: SweepstakeData,
    ) -> Result<()> {
        let user = &mut ctx.accounts.user_state;
        let sweepstake = &mut ctx.accounts.sweepstake_state;

        create_sweepstake(ctx.accounts.authority.key(), user, sweepstake, &data)?;
        transfer(ctx.accounts.to_transfer_context(), LAMPORTS_PER_SWEEPSTAKE)?;

        Ok(())
    }

    /// Create sweepstake and pay in supported SPL token for it
    /// - A sweepstake can be created before the start of the World Cup
    /// - All the input data must be valid
    /// - User cannot exceed a number of sweepstakes per wallet
    /// - Sweepstake must be paid in supported SPL token
    pub fn create_sweepstake_spl(
        ctx: Context<CreateSweepstakeSpl>,
        data: SweepstakeData,
    ) -> Result<()> {
        let user = &mut ctx.accounts.user_state;
        let sweepstake = &mut ctx.accounts.sweepstake_state;

        create_sweepstake(ctx.accounts.authority.key(), user, sweepstake, &data)?;
        let supported_token =
            validate_supported_token(&ctx.accounts.mint, &ctx.accounts.dagoats_wallet)?;
        token_transfer(
            ctx.accounts.to_transfer_context(),
            supported_token.sweepstake_price,
        )?;

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
    sweepstake.predictions = data.predictions.clone();
    sweepstake.pre_sweepstake_key = None;
    sweepstake.id = get_valid_id(data.id)?;

    user.current_sweepstake_key = Some(sweepstake.key());
    user.sweepstakes_submitted += 1;

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
        // discriminator + authority + SHA1 data + submitted_at + pre_sweepstake_key + id
        space = 8 + 32 + 512 + 8 + std::mem::size_of::<Option<Pubkey>>() + 8,
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

#[derive(Accounts)]
pub struct CreateSweepstakeSpl<'info> {
    #[account(mut, has_one = authority)]
    pub user_state: Account<'info, User>,
    #[account(
        init,
        payer = authority,
        // discriminator + authority + SHA1 data + submitted_at + pre_sweepstake_key + id
        space = 8 + 32 + 512 + 8 + std::mem::size_of::<Option<Pubkey>>() + 8,
    )]
    pub sweepstake_state: Account<'info, Sweepstake>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account()]
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        owner = token_program.key(),
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    pub user_wallet: Account<'info, TokenAccount>,
    #[account(
        mut,
        owner = token_program.key(),
        token::mint = mint,
    )]
    pub dagoats_wallet: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> CreateSweepstakeSpl<'info> {
    pub fn to_transfer_context(
        &self,
    ) -> CpiContext<'info, 'info, 'info, 'info, TokenTransfer<'info>> {
        let accounts = TokenTransfer {
            authority: self.authority.to_account_info(),
            from: self.user_wallet.to_account_info(),
            to: self.dagoats_wallet.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accounts)
    }
}

security_txt! {
    name: "DaGOATs Sweepstake",
    project_url: "http://sweepstake.dagoats.io/",
    contacts: "email:info@dagoats.io,link:http://dagoats.io/,discord:SolRetroNFTs#8494",
    policy: "http://dagoats.io/",
    preferred_languages: "en,es"
}
