use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;

use crate::{Player, Team, Error};
use crate::error::ErrorCode::TooMuchPlayersInTeam;

#[derive(Accounts)]
pub struct CheckIn<'info>{
    pub player: Account<'info, Player>,
    pub team: Account<'info, Team>
}

pub fn handler(ctx: Context<CheckIn>) -> Result<()>
{
    let player = &mut ctx.accounts.player;
    let team = &mut ctx.accounts.team;
    // unique number for each day
    let day_index = Clock::get()?.unix_timestamp / 86400;

    Ok(())
}