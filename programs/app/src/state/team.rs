use anchor_lang::prelude::*;

#[account]
pub struct Team
{
    //How many days is the streak
    pub streak_counter: u32,
    // const MAX_MEMBERS = 3;
    pub members: Vec<Pubkey>,
}