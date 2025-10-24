use anchor_lang::prelude::*;
use crate::state::Counter;
use crate::state::Team;
use crate::state::Player;

#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init,
    payer = authority,
    space = 8+32+8+8+1, // recheck
    seeds = [b"counter", authority.key().as_ref()],
    bump )]
    
    pub counter: Account<'info, Counter>,

        #[account(init,
        payer = authority,
        space = 8+3*32+32)] //donut forget to update if changed the Team struct
    pub team: Account<'info, Team>,

    pub system_program: Program<'info, System>
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    counter.authority = ctx.accounts.authority.key();
    counter.count = 0;
    counter.bump = ctx.bumps.counter;

    let team = &mut ctx.accounts.team;
    team.players = vec![];



    // ctx.accounts.authority 
    msg!("Greetings from: {:?}", ctx.program_id);
    Ok(())
}


