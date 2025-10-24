use anchor_lang::prelude::*;

use crate::{Player, Team};
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct JoinTeam<'info>{

    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub team: Account<'info, Team>,

    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 8,
        seeds = [b"player", team.key().as_ref(), authority.key().as_ref()],
        bump
    )]
    pub player: Account<'info, Player>,

    pub system_program: Program<'info, System>
}

pub fn handler(ctx: Context<JoinTeam>) -> Result<()>
{
    let team = &mut ctx.accounts.team;
    // if team.players.len() >= 3
    //     {return Err(Error::from(TooMuchPlayersInTeam))}
    // let push_result = team.players.push(
    //     player.key()
    // );
    if team.player_count >= 3 { return Err(error!(ErrorCode::TooMuchPlayersInTeam)); }

    let player = &mut ctx.accounts.player;
    player.team = team.key();
    player.authority = ctx.accounts.authority.key();
    team.player_count = team.player_count.saturating_add(1);

    Ok(())
}