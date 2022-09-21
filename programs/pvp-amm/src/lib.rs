use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, MintTo};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod pvp_amm {
    use super::*;

    pub fn create_pool(ctx: Context<CreatePool>, long_col: u64, short_col: u64, 
            long_pos: u64, short_pos: u64, asset_price: f32) -> Result<()> {
        
        let pool_data: &mut Account<Pool> = &mut ctx.accounts.pool;
        let long_payer: &Signer = &ctx.accounts.long_payer;
        let short_payer: &Signer = &ctx.accounts.short_payer;
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
        pool_data.long_dist;
        pool_data.short_dist;

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
            to: ctx.accounts.mint_to.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::mint_to(cpi_ctx, mint_amount)?;
        
        Ok(())
    }

    pub fn close_pool(ctx: Context<ClosePool>, final_price: f32) -> Result<()> {
        let pool: &mut Account<Pool> = &mut ctx.accounts.pool_account;
        let clock: Clock = Clock::get().unwrap();
        
        let pos_l = pool.long_pos;
        let col_l = pool.long_col;
        let pos_s = pool.short_pos;
        let col_s = pool.short_col;
        let op = pool.asset_price;

        let mut long_dist: f32 = (((final_price/op)-1.0) / (col_l as f32 / pos_l as f32)) * col_l as f32;
        let mut short_dist: f32 = col_s as f32 / ((col_s as f32 / pos_s as f32) / ((final_price/op)-1.0)) as f32;

        if final_price < op {
            long_dist = long_dist * -1.0;
            short_dist = short_dist * -1.0;
        }
        
        pool.long_dist = long_dist;
        pool.short_dist = short_dist;
        pool.final_price = final_price;
        pool.timestamp_close = clock.unix_timestamp;

        let cpi_accounts1 = Transfer {
            from: ctx.accounts.transfer_from.to_account_info(),
            to: ctx.accounts.transfer_to.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program1 = ctx.accounts.token_program.to_account_info();
        let cpi_ctx1 = CpiContext::new(cpi_program1, cpi_accounts1);

        token::transfer(cpi_ctx1, long_dist as u64)?;

        let cpi_accounts2 = Transfer {
            from: ctx.accounts.transfer_from.to_account_info(),
            to: ctx.accounts.transfer_to2.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program2 = ctx.accounts.token_program.to_account_info();
        let cpi_ctx2 = CpiContext::new(cpi_program2, cpi_accounts2);

        token::transfer(cpi_ctx2, short_dist as u64)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(init, payer = long_payer, space = Pool::LEN)]
    pub pool: Account<'info, Pool>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub long_payer: Signer<'info>,
    #[account(mut)]
    pub short_payer: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub from2: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint_to: Account<'info, TokenAccount>,
    // #[account(mut)]
    // pub transferTo: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClosePool<'info> {
    #[account(mut)]
    pub pool_account: Account<'info, Pool>,
    #[account(mut)]
    pub long_payer: Signer<'info>,
    #[account(mut)]
    pub short_payer: Signer<'info>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub transfer_from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub transfer_to: Account<'info, TokenAccount>,
    #[account(mut)]
    pub transfer_to2: Account<'info, TokenAccount>,
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
    pub asset_price: f32,
    pub final_price: f32,
    pub long_dist: f32,
    pub short_dist: f32,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const REMAINING_ACCOUNTS: usize = 100;

impl Pool {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH
        + PUBLIC_KEY_LENGTH
        + TIMESTAMP_LENGTH
        + TIMESTAMP_LENGTH
        + REMAINING_ACCOUNTS;
}