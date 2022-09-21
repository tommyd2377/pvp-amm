use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, MintTo};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod pvp_amm {
    use super::*;

    pub fn create_pool(ctx: Context<CreatePool>, long_col: u64, short_col: u64, 
            long_pos: u64, short_pos: u64, asset_price: u64) -> Result<()> {
        
        let pool_data: &mut Account<Pool> = &mut ctx.accounts.pool;
        let long_payer: &Signer = &ctx.accounts.longPayer;
        let short_payer: &Signer = &ctx.accounts.shortPayer;
        let clock: Clock = Clock::get().unwrap();

        let mint_amount = long_col + short_col;

        pool_data.long_payer = *long_payer.key;
        pool_data.short_payer = *short_payer.key;
        pool_data.timestamp = clock.unix_timestamp;
        pool_data.long_col = long_col;
        pool_data.short_col = short_col;
        pool_data.long_pos = long_pos;
        pool_data.short_pos = short_pos;
        pool_data.asset_price = asset_price;
        pool_data.final_price;
        pool_data.timestamp_close;

        // let cpi_accounts1 = Transfer {
        //     from: ctx.accounts.from.to_account_info(),
        //     to: ctx.accounts.transferTo.to_account_info(),
        //     authority: ctx.accounts.longPayer.to_account_info(),
        // };
        // let cpi_program1 = ctx.accounts.token_program.to_account_info();
        // let cpi_ctx1 = CpiContext::new(cpi_program1, cpi_accounts1);

        // token::transfer(cpi_ctx1, long_col)?;

        // let cpi_accounts2 = Transfer {
        //     from: ctx.accounts.from2.to_account_info(),
        //     to: ctx.accounts.transferTo.to_account_info(),
        //     authority: ctx.accounts.shortPayer.to_account_info(),
        // };
        // let cpi_program2 = ctx.accounts.token_program.to_account_info();
        // let cpi_ctx2 = CpiContext::new(cpi_program2, cpi_accounts2);

        // token::transfer(cpi_ctx2, short_col)?;

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.mintTo.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::mint_to(cpi_ctx, mint_amount)?;
        
        Ok(())
    }

    pub fn close_pool(ctx: Context<ClosePool>, final_price: u64) -> Result<()> {
        let pool: &mut Account<Pool> = &mut ctx.accounts.poolAccount;
        let clock: Clock = Clock::get().unwrap();
        
        pool.asset_price = final_price;


        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(init, payer = longPayer, space = Pool::LEN)]
    pub pool: Account<'info, Pool>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub longPayer: Signer<'info>,
    #[account(mut)]
    pub shortPayer: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub from2: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mintTo: Account<'info, TokenAccount>,
    // #[account(mut)]
    // pub transferTo: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClosePool<'info> {
    #[account(mut)]
    pub poolAccount: Account<'info, Pool>,
    #[account(mut)]
    pub longPayer: Signer<'info>,
    #[account(mut)]
    pub shortPayer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Pool {
    pub long_payer: Pubkey,
    pub short_payer: Pubkey,
    pub timestamp: i64,
    pub timestamp_close: i64,
    pub long_col: u64,
    pub short_col: u64,
    pub long_pos: u64,
    pub short_pos: u64,
    pub asset_price: u64,
    pub final_price: u64,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_TOPIC_LENGTH: usize = 50 * 4;
const MAX_CONTENT_LENGTH: usize = 280 * 4;
const REVIEW_LENGTH: usize = 32;

impl Pool {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH
        + TIMESTAMP_LENGTH
        + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH
        + REVIEW_LENGTH;
}