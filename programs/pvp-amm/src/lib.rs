use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, MintTo};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod pvp_amm {
    use super::*;

    pub fn create_pool(ctx: Context<CreatePool>, long_col: u64, short_col: u64) -> Result<()> {
        let pool_data: &mut Account<Pool> = &mut ctx.accounts.pool;
        let payer: &Signer = &ctx.accounts.payer;
        let clock: Clock = Clock::get().unwrap();

        pool_data.payer = *payer.key;
        pool_data.timestamp = clock.unix_timestamp;
        pool_data.long_col = long_col;
        pool_data.short_col = short_col;

        // let cpi_accounts = MintTo {
        //     from: ctx.accounts.from.to_account_info(),
        //     to: ctx.accounts.to.to_account_info(),
        //     authority: ctx.accounts.author.to_account_info(),
        // };
        // let cpi_program = ctx.accounts.token_program.to_account_info();
        // let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        //token::mint_to(cpi_ctx, amount)?;

        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::transfer(cpi_ctx, long_col)?;

        let cpi_accounts2 = Transfer {
            from: ctx.accounts.from2.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.payer2.to_account_info(),
        };
        let cpi_program2 = ctx.accounts.token_program.to_account_info();
        let cpi_ctx2 = CpiContext::new(cpi_program2, cpi_accounts2);

        token::transfer(cpi_ctx2, short_col)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(init, payer = payer, space = Pool::LEN)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub payer2: Signer<'info>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub from2: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Pool {
    pub payer: Pubkey,
    pub timestamp: i64,
    pub long_col: u64,
    pub short_col: u64,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
const MAX_TOPIC_LENGTH: usize = 50 * 4; // 50 chars max.
const MAX_CONTENT_LENGTH: usize = 280 * 4; // 280 chars max.
const REVIEW_LENGTH: usize = 32;

impl Pool {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Author.
        + TIMESTAMP_LENGTH // Timestamp.
        + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH // Topic.
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH
        + REVIEW_LENGTH; // Content.
}