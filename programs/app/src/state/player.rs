use anchor_lang::prelude::*;

#[account]
pub struct Player{


    pub authority: Pubkey,

    ///player is assigned to #team
    pub team: Pubkey,

    pub scheduled_wake_up_time_utc: u64,
}