use anchor_lang::prelude::*;
//use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, MintTo};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod pvp_amm {
    use super::*;

    pub fn create_pool(ctx: Context<CreatePool>, amount: u64) -> Result<()> {
        let poolData: &mut Account<Pool> = &mut ctx.accounts.pool;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        poolData.author = *author.key;
        poolData.timestamp = clock.unix_timestamp;
        poolData.amount = amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(init, payer = author, space = Pool::LEN)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Pool {
    pub author: Pubkey,
    pub timestamp: i64,
    pub amount: u64,
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