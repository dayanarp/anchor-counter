use anchor_lang::prelude::*;

declare_id!("vAxSkkVFzGbDb2aRyAEC45pD793tYjj7rfGJwMvab3a");
#[program]
mod counter {
    use super::*;

    pub fn create_counter(ctx: Context<CreateCounter>, count: u64) -> Result<()> {
        ctx.accounts.counter.count = count;
        ctx.accounts.counter.authority = ctx.accounts.authority.key();
        msg!("Created counter with count: {}!", count);
        Ok(())
    }

    pub fn update_counter(ctx: Context<UpdateCounter>, count: u64) -> Result<()> {
        ctx.accounts.counter.count = count;
        msg!("Updated counter count to: {}!", count);
        Ok(())
    }

    pub fn increment_counter(ctx: Context<IncrementCounter>) -> Result<()> {
        let count = ctx.accounts.counter.count;
        ctx.accounts.counter.count = count + 1;
        msg!("Updated counter count to: {}!", count);
        Ok(())
    }

    pub fn decrement_counter(ctx: Context<DecrementCounter>) -> Result<()> {
        let count = ctx.accounts.counter.count;
        ctx.accounts.counter.count = count - 1;
        msg!("Updated counter count to: {}!", count);
        Ok(())
    }

    pub fn delete_counter(_ctx: Context<DeleteCounter>) -> Result<()> {
        msg!("Deleted counter!");
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(count: u64)]
pub struct CreateCounter<'info> {
    #[account(init, payer = authority, space = 8 + 8 + 32)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(count: u64)]
pub struct UpdateCounter<'info> {
    #[account(
        mut, 
        constraint = counter.authority == authority.key() @ ErrorCode::NotAuthorized, 
    )]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct IncrementCounter<'info> {
    #[account(
        mut, 
        constraint = counter.authority == authority.key() @ ErrorCode::NotAuthorized, 
    )]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DecrementCounter<'info> {
    #[account(
        mut, 
        constraint = counter.authority == authority.key() @ ErrorCode::NotAuthorized, 
        constraint = counter.count > 0 @ ErrorCode::CantDecrement, 
    )]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteCounter<'info> {
    #[account(
        mut, 
        constraint = counter.authority == authority.key() @ ErrorCode::NotAuthorized, 
        close = authority
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Counter {
    count: u64,
    authority: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized.")]
    NotAuthorized,
    #[msg("Counter already at 0.")]
    CantDecrement,
}