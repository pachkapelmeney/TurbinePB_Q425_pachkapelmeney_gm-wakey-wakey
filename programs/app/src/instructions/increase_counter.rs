use anchor_lang::prelude::*;
use crate::Counter;
use anchor_lang::error::AnchorError;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct IncreaseCounter<'info>{
    // pub counter: Counter<'info>;
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut, seeds = [b"counter", authority.key().as_ref()], bump)]
    pub counter: Account<'info, Counter>,

    pub system_program: Program<'info, System>
}

pub fn handler(ctx: Context<IncreaseCounter>) -> Result<()>{
    ctx.accounts.counter.count = ctx.accounts.counter.count.checked_add(1).ok_or(ErrorCode::CustomError)?;

    Ok(())
}
