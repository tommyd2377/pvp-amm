use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Burn, Token, TokenAccount, Transfer, MintTo};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod pvp_amm {
    use super::*;

    pub fn create_trade(ctx: Context<CreateTrade>, long_col: u64, short_col: u64, 
            long_pos: u64, short_pos: u64, asset_price: f32) -> Result<()> {
        
        let trade_data: &mut Account<Trade> = &mut ctx.accounts.trade;
        let long_payer: &Signer = &ctx.accounts.long_payer;
        let short_payer: &Signer = &ctx.accounts.short_payer;
        let clock: Clock = Clock::get().unwrap();

        let mint_amount = long_col + short_col;

        trade_data.long_payer = *long_payer.key;
        trade_data.short_payer = *short_payer.key;
        trade_data.timestamp = clock.unix_timestamp;
        trade_data.long_col = long_col;
        trade_data.short_col = short_col;
        trade_data.long_pos = long_pos;
        trade_data.short_pos = short_pos;
        trade_data.asset_price = asset_price;
        trade_data.long_open = true;
        trade_data.short_open = true;
        trade_data.final_price;
        trade_data.timestamp_close;
        trade_data.long_dist;
        trade_data.short_dist;

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

    pub fn close_trade(ctx: Context<CloseTrade>, final_price: f32) -> Result<()> {
        let trade: &mut Account<Trade> = &mut ctx.accounts.trade_account;
        let clock: Clock = Clock::get().unwrap();
        
        let pos_l = trade.long_pos;
        let col_l = trade.long_col;
        let pos_s = trade.short_pos;
        let col_s = trade.short_col;
        let op = trade.asset_price;

        let mut long_dist: f32 = (((final_price / op) - 1.0) / (col_l as f32 / pos_l as f32)) * col_l as f32;
        let mut short_dist: f32 = col_s as f32 / ((col_s as f32 / pos_s as f32) / (( final_price / op) - 1.0)) as f32;

        if final_price > op {
            long_dist = long_dist + col_l as f32;
            trade.long_dist = long_dist;
            trade.short_dist = short_dist;
            trade.long_open = false;
            trade.short_open = false;
            trade.final_price = final_price;
            trade.timestamp_close = clock.unix_timestamp;

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

            // let destroyed_tokens = (long_dist + short_dist) + (pos_s + pos_l) as f32;
            
            // let cpi_accounts = Burn {
            //     mint: ctx.accounts.gd_mint.to_account_info(),
            //     to: ctx.accounts.mint_to.to_account_info(),
            //     authority: ctx.accounts.authority1.to_account_info(),
            // };
            // let cpi_program = ctx.accounts.token_program.to_account_info();
            // let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
            // token::mint_to(cpi_ctx, destroyed_tokens as u64)?;
            
        }

        if final_price < op {
            long_dist = col_l as f32 + long_dist;
            short_dist = (short_dist * -1.0) + col_s as f32;

            trade.long_dist = long_dist;
            trade.short_dist = short_dist;
            trade.long_open = false;
            trade.short_open = false;
            trade.final_price = final_price;
            trade.timestamp_close = clock.unix_timestamp;

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

           // let new_tokens = (long_dist + short_dist) + (pos_s + pos_l) as f32;
            
            // let cpi_accounts = MintTo {
            //     mint: ctx.accounts.gd_mint.to_account_info(),
            //     to: ctx.accounts.mint_to.to_account_info(),
            //     authority: ctx.accounts.authority1.to_account_info(),
            // };
            // let cpi_program = ctx.accounts.token_program.to_account_info();
            // let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
            // token::mint_to(cpi_ctx, new_tokens as u64)?;
        }  
    
    Ok(())   
    }
}

#[derive(Accounts)]
pub struct CreateTrade<'info> {
    #[account(init, payer = long_payer, space = Trade::LEN)]
    pub trade: Account<'info, Trade>,
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
pub struct CloseTrade<'info> {
    #[account(mut)]
    pub trade_account: Account<'info, Trade>,
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
    // #[account(mut)]
    // pub gd_mint: Account<'info, Mint>,
    // pub authority1: Signer<'info>,
    // #[account(mut)]
    // pub mint_to: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Trade {
    pub long_payer: Pubkey,
    pub short_payer: Pubkey,
    pub timestamp: i64,
    pub timestamp_close: i64,
    pub long_col: u64,
    pub short_col: u64,
    pub long_pos: u64,
    pub short_pos: u64,
    pub long_open: bool,
    pub short_open: bool,
    pub asset_price: f32,
    pub final_price: f32,
    pub long_dist: f32,
    pub short_dist: f32,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const REMAINING_ACCOUNTS: usize = 100;

impl Trade {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH
        + PUBLIC_KEY_LENGTH
        + TIMESTAMP_LENGTH
        + TIMESTAMP_LENGTH
        + REMAINING_ACCOUNTS;
}