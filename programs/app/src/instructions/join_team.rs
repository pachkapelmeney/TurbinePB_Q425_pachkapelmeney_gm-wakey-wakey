use anchor_lang::prelude::*;

use crate::{Player, Team, Error};
use crate::error::ErrorCode::TooMuchPlayersInTeam;

#[derive(Accounts)]
pub struct JoinTeam<'info>{
    
    #[account(mut)]
    pub member: Account<'info, Player>,

    #[account(mut)]
    pub team: Account<'info, Team>
}

pub fn handler(ctx: Context<JoinTeam>) -> Result<()>
{
    let team = &mut ctx.accounts.team;
    if team.members.len() >= 3
        {return Err(Error::from(TooMuchPlayersInTeam))}

    team.members.push(
        ctx.accounts.member.pubkey
    );

    Ok(())
}