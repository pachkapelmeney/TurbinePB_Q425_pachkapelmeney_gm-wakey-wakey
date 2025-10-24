use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;

use crate::{Player, Team, Error};
use crate::error::ErrorCode::TooMuchPlayersInTeam;
use crate::error::ErrorCode::OnlyPlayerCanAddHimself;

#[derive(Accounts)]
pub struct PlayerCheckIn<'info>{
    pub authority: Signer<'info>,
    #[account(mut)]
    pub player: Account<'info, Player>,
    #[account(mut)]
    pub team: Account<'info, Team>
}

pub fn handler(ctx: Context<PlayerCheckIn>, submited_wake_up_time_utc: u64) -> Result<()>
{
    let player = &mut ctx.accounts.player;
    let team = &mut ctx.accounts.team;

    require!(ctx.accounts.authority.key() == player.authority, crate::error::ErrorCode::OnlyPlayerCanAddHimself);

    // unique number for each day
    let day_index = Clock::get()?.unix_timestamp / 86400;
    // day_index > team.last_checkin_day_index

    let day_start_ts = day_index * 86400;
    let legit_checkin_window = day_start_ts as u64 + player.scheduled_wake_up_time_utc + team.grace_period_seconds;
    
    //the counter part! ha
    team.streak_counter = team.streak_counter.checked_add(1).ok_or(crate::error::ErrorCode::Overflow)?;

    if (submited_wake_up_time_utc > legit_checkin_window) //:TODO add an "automatic fail" if not submitteed on time
        {team.streak_counter = 0;} //even 1 late-submitted player resets the whole team streak!

    Ok(())
}