use anchor_lang::prelude::*;

pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
    pub bump: u8,
}