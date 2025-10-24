use anchor_lang::prelude::*;

#[account]
pub struct Team
{
    //How many days is the streak
    pub streak_counter: u32,
    // const MAX_MEMBERS = 3;
    // pub players: Vec<Pubkey>,

    // how long is the time-window for checking in?
    // move to the "Game-Session" PDA in the future
    pub grace_period_seconds: u64,
}