use anchor_lang::prelude::*;

#[account] //self-reminder- this is a marking for an 'ANCHOR' account -> macro for impl serializer
pub struct Counter {
    pub authority: Pubkey,
    pub count: u8,
    pub bump: u8,
}