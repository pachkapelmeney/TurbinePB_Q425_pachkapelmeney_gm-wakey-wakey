use anchor_lang::prelude::*;

use crate::{Player, Team, Error};
use crate::error::ErrorCode::TooMuchPlayersInTeam;

#[derive(Accounts)]
pub struct JoinTeam<'info>{

    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = 8+32+32+8,
        seeds = [b"player", authority.key().as_ref()],
        bump
    )]
    pub player: Account<'info, Player>,

    #[account(mut)]
    pub team: Account<'info, Team>,

    #[account(mut)]
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
    if team.player_count >= 3
    {return Err(Error::from(TooMuchPlayersInTeam))}

    let player = &mut ctx.accounts.player;
    
    

    player.team = team.key();

    require!(player.authority.key() == ctx.accounts.authority.key(), crate::error::ErrorCode::TooMuchPlayersInTeam);

    team.player_count.checked_add(1);

    Ok(())
}